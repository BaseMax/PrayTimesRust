use chrono::{NaiveDate, NaiveDateTime};
use types::{
    CalculationUnit, Degrees, HighLatsMethod, Location, MidnightMethod, Minutes, Parameters,
    PraytimesOutput,
};
use utils::{d_math, julian_date::to_julian_date, numbers::fix_hour, sun_position::sun_position};

pub mod methods;
pub mod types;
mod utils;

pub struct Calculator(Parameters);
impl Calculator {
    pub fn calculate(&self, location: &Location, date: &NaiveDate) -> PraytimesOutput {
        let calculator = InternalCalculator {
            date: &date,
            location: &location,
            params: &self.0,
            julian_date: to_julian_date(date, location),
        };
        calculator.calculate()
    }
    pub fn new(params: Parameters) -> Self {
        Self(params)
    }
}

struct InternalCalculator<'a> {
    params: &'a Parameters,
    location: &'a Location,
    date: &'a NaiveDate,
    julian_date: f64,
}
impl<'a> InternalCalculator<'a> {
    fn calculate(&self) -> PraytimesOutput {
        PraytimesOutput {
            imsak: self.datetime_from_hours(self.imsak()),
            fajr: self.datetime_from_hours(self.fajr()),
            sunrise: self.datetime_from_hours(self.sunrise()),
            dhuhr: self.datetime_from_hours(self.dhuhr()),
            asr: self.datetime_from_hours(self.asr()),
            sunset: self.datetime_from_hours(self.sunset()),
            maghrib: self.datetime_from_hours(self.maghrib()),
            isha: self.datetime_from_hours(self.isha()),
            midnight: self.datetime_from_hours(self.midnight()),
        }
    }
    fn midnight(&self) -> f64 {
        let sunset = self.sunset();
        let sunrise = self.sunrise();
        dbg!(&sunset);
        dbg!(&sunrise);
        dbg!(Self::time_difference(sunrise, sunset));
        let midnight = match self.params.midnight {
            MidnightMethod::Standard => sunset + Self::time_difference(sunset, sunrise) / 2.0,
            MidnightMethod::Jafari => sunset + Self::time_difference(sunset, self.fajr()) / 2.0,
        };

        midnight
    }
    fn asr(&self) -> f64 {
        self.asr_time(self.params.asr.0, 13.0 / 24.0)
    }

    fn asr_time(&self, factor: f64, time: f64) -> f64 {
        let decl = sun_position(self.julian_date + time).declination;
        let angle = -d_math::arccot(factor + d_math::tan((self.location.latitude - decl).abs()));
        self.mid_day(time) + self.sat(time, angle)
    }
    pub fn sat(&self, time: f64, angle: f64) -> f64 {
        let decl = sun_position(self.julian_date + time).declination;

        (1.0 / 15.0)
            * d_math::arccos(
                (-d_math::sin(angle) - d_math::sin(decl) * d_math::sin(self.location.latitude))
                    / (d_math::cos(decl) * d_math::cos(self.location.latitude)),
            )
    }

    pub fn sunrise(&self) -> f64 {
        let angle = self.rise_set_angle();
        let time = self.mid_day(6.0 / 24.0) - self.sat(6.0 / 24.0, angle);
        time
    }
    pub fn rise_set_angle(&self) -> f64 {
        let angle = 0.0347 * self.location.elevation.sqrt();

        let angle = 0.833 + angle;

        angle
    }

    pub fn sunset(&self) -> f64 {
        let angle = self.rise_set_angle();
        let time = self.mid_day(18.0 / 24.0) + self.sat(18.0 / 24.0, angle);
        time
    }
    fn mid_day(&self, time: f64) -> f64 {
        let sun_pos = sun_position(self.julian_date + time);

        let eqt = sun_pos.equation;

        let noon = fix_hour(12.0 - eqt) - self.location.longitude / 15.0;

        noon
    }
    fn dhuhr(&self) -> f64 {
        let mid_day = self.mid_day(12.0 / 24.0);
        dbg!(&mid_day);
        let var_name = mid_day + self.params.dhuhr.0 / 60.0;
        dbg!(&var_name);
        var_name
    }

    pub fn imsak(&self) -> f64 {
        match self.params.imsak {
            types::CalculationUnit::Degrees(Degrees(angle)) => {
                let time = self.mid_day(5.0 / 24.0) - self.sat(5.0 / 24.0, angle);
                let base = self.sunrise();
                if self.high_lat_adjustment_needed(time, base, angle) {
                    base - self.night_portion(angle)
                } else {
                    time
                }
            }
            types::CalculationUnit::Minutes(Minutes(minutes)) => self.fajr() - minutes / 60.0,
        }
    }
    pub fn maghrib(&self) -> f64 {
        let base = self.sunset();

        match self.params.maghrib {
            CalculationUnit::Minutes(Minutes(minutes)) => base + minutes / 60.0,
            CalculationUnit::Degrees(Degrees(angle)) => {
                let time = self.mid_day(18.0 / 24.0) + self.sat(18.0 / 24.0, angle);
                {
                    let ref this = self;
                    if this.high_lat_adjustment_needed(time, base, angle) {
                        base + this.night_portion(angle)
                    } else {
                        time
                    }
                }
            }
        }
    }
    pub fn isha(&self) -> f64 {
        match self.params.isha {
            CalculationUnit::Minutes(Minutes(minutes)) => self.maghrib() + minutes / 60.0,
            CalculationUnit::Degrees(Degrees(angle)) => {
                let time = self.mid_day(18.0 / 24.0) + self.sat(18.0 / 24.0, angle);
                if self.high_lat_adjustment_needed(time, self.sunset(), angle) {
                    self.sunset() + self.night_portion(angle)
                } else {
                    time
                }
            }
        }
    }
    pub fn fajr(&self) -> f64 {
        let angle = self.params.fajr.0;
        let time = self.mid_day(5.0 / 24.0) - self.sat(5.0 / 24.0, angle);
        dbg!(time);
        if self.high_lat_adjustment_needed(time, self.sunrise(), angle) {
            dbg!("adjusting for high lats");
            self.sunrise() - self.night_portion(angle)
        } else {
            time
        }
    }

    fn high_lat_adjustment_needed(&self, time: f64, base: f64, angle: f64) -> bool {
        let portion = self.night_portion(angle);
        let time_diff = (time - base).abs();

        time.is_nan() || (self.params.high_latitudes != HighLatsMethod::None && time_diff > portion)
    }

    fn night_portion(&self, angle: f64) -> f64 {
        let night = self.night_time();
        let portion = match self.params.high_latitudes {
            HighLatsMethod::AngleBased => angle / 60.0,
            HighLatsMethod::OneSeventh => 1.0 / 7.0,
            HighLatsMethod::None => f64::NAN,
            HighLatsMethod::NightMiddle => 1.0 / 2.0, // default to MidNight
        };

        portion * night
    }

    fn night_time(&self) -> f64 {
        Self::time_difference(self.sunset(), self.sunrise())
    }

    fn datetime_from_hours(&self, hours: f64) -> Option<NaiveDateTime> {
        if hours.is_nan() {
            return None;
        }

        let time = self
            .date
            .and_hms_milli_opt(0, 0, 0, 0)
            .unwrap()
            .timestamp_millis()
            + (hours * 3600.0 * 1000.0) as i64;
        Some(NaiveDateTime::from_timestamp_millis(time).unwrap())
    }
    fn time_difference(time1: f64, time2: f64) -> f64 {
        let difference = time2 - time1;

        let fixed = fix_hour(difference);
        fixed
    }
}
