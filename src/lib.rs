use chrono::{DateTime, Datelike, Local, NaiveDate, TimeZone, Timelike, Utc};
use types::{Location, Parameters, PraytimesOutput};
use utils::{julian_date::to_julian_date, numbers::fix_hour, sun_position::sun_position};

pub mod methods;
pub mod types;
mod utils;

pub struct Calculator(Parameters);
impl Calculator {
    pub fn calculate(&self, location: &Location, date: &DateTime<Local>) -> PraytimesOutput {
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
    date: &'a DateTime<Local>,
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
    fn datetime_from_hours(&self, hours: f64) -> Option<DateTime<Local>> {
        if hours.is_nan() {
            return None;
        }

        let time = self.date.timestamp_nanos()+(hours * 3600.0 * 1000.0*1000_000.0) as i64;
        Some(Local.timestamp_nanos(time))


    }
}
