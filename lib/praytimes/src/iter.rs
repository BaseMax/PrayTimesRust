use chrono::{NaiveDate, NaiveDateTime};

use crate::{
    types::{Location, PraytimeType},
    Calculator,
};

pub struct PraytimesIterator {
    location: Location,
    calculator: Calculator,
    date: NaiveDate,
    calculated: Vec<(PraytimeType, NaiveDateTime)>,
}

impl Iterator for PraytimesIterator {
    type Item = (PraytimeType, NaiveDateTime);

    fn next(&mut self) -> Option<Self::Item> {
        if self.calculated.len() == 0 {
            match self.date.succ_opt() {
                Some(d) => {
                    let times = self.calculator.calculate(&self.location, &d).into_vec();
                    self.date = d;
                    self.calculated = times;
                }
                None => return None,
            }
        }

        self.calculated.pop()
    }
}
