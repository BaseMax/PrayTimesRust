use crate::internal_calculator;
use crate::types::{Location, Parameters, PraytimesOutput};
use crate::utils::julian_date::to_julian_date;
use chrono::NaiveDate;

pub struct Calculator {
    params: Parameters,
}

impl Calculator {
    pub fn calculate(&self, location: &Location, date: &NaiveDate) -> PraytimesOutput {
        let calculator = internal_calculator::InternalCalculator {
            date: &date,
            location: &location,
            params: &self.params,
            julian_date: to_julian_date(date, location),
        };
        calculator.calculate()
    }
    pub fn new(params: Parameters) -> Self {
        Self { params }
    }
}
