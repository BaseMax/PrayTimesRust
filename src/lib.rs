use chrono::{DateTime, Local};
use types::{Location, Parameters, PraytimesOutput};

pub mod methods;
pub mod types;
pub struct Calculator(Parameters);
impl Calculator {
    pub fn calculate(&self,location: &Location, date: &DateTime<Local>) -> PraytimesOutput {
        todo!("write the calculation logic")
    }
    pub fn new(params: Parameters) -> Self {
        Self(params)
    }
}
