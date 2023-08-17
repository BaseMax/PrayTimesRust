use crate::internal_calculator;
use crate::types::{Location, Parameters, PraytimesOutput, TuneOffsets};
use crate::utils::julian_date::to_julian_date;
use chrono::{NaiveDate, NaiveDateTime};

pub struct Calculator {
    params: Parameters,
    tuning_offsets: TuneOffsets,
}

impl Calculator {
    pub fn calculate(&self, location: &Location, date: &NaiveDate) -> PraytimesOutput {
        let calculator = internal_calculator::InternalCalculator {
            date: &date,
            location: &location,
            params: &self.params,
            julian_date: to_julian_date(date, location),
        };
        let times = calculator.calculate();
        self.tune(times)
    }
    fn tune(&self, times: PraytimesOutput) -> PraytimesOutput {
        PraytimesOutput {
            imsak: Self::tune_time(times.imsak, self.tuning_offsets.imsak),
            fajr: Self::tune_time(times.fajr, self.tuning_offsets.fajr),
            dhuhr: Self::tune_time(times.dhuhr, self.tuning_offsets.dhuhr),
            asr: Self::tune_time(times.asr, self.tuning_offsets.asr),
            sunset: Self::tune_time(times.sunset, self.tuning_offsets.sunset),
            maghrib: Self::tune_time(times.maghrib, self.tuning_offsets.maghrib),
            isha: Self::tune_time(times.isha, self.tuning_offsets.isha),
            midnight: Self::tune_time(times.midnight, self.tuning_offsets.midnight),
            sunrise: Self::tune_time(times.sunrise, self.tuning_offsets.sunrise),
        }
    }
    fn tune_time(time: Option<NaiveDateTime>, offset: Option<f64>) -> Option<NaiveDateTime> {
        match (time, offset) {
            (Some(time), Some(o)) => NaiveDateTime::from_timestamp_millis(
                time.timestamp_millis() + (o * 60.0 * 1000.0) as i64,
            ),
            _ => time,
        }
    }
    pub fn new(params: Parameters, tuning_offsets: TuneOffsets) -> Self {
        Self {
            params,
            tuning_offsets,
        }
    }
}
