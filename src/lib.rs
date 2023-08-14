use chrono::{ Datelike, NaiveDate, TimeZone, Timelike, Utc, NaiveDateTime};
use types::{Location, Parameters, PraytimesOutput};
use utils::{julian_date::to_julian_date, numbers::fix_hour, sun_position::sun_position};

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
            dhuhr: self.datetime_from_hours(self.dhuhr()),
            ..Default::default()
        }
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
    fn datetime_from_hours(&self, hours: f64) -> Option<NaiveDateTime> {
        if hours.is_nan() {
            return None;
        }

        dbg!(self.date);
        
        let time = self.date.and_hms_milli_opt(0, 0, 0,0).unwrap().timestamp_millis()+(hours * 3600.0 * 1000.0) as i64;
        Some(NaiveDateTime::from_timestamp_millis(time).unwrap())


    }
}
