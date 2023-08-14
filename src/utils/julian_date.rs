use chrono::{DateTime, Datelike, Local};

use crate::types::Location;

pub fn to_julian_date(date: &DateTime<Local>, location: &Location) -> f64 {
    let julian = julian_from_gregorian(date.year(), date.month(), date.day());
    julian - location.longitude / (15.0 * 24.0)
}

fn julian_from_gregorian(mut year: i32, mut month: u32, day: u32) -> f64 {
    if month <= 2 {
        year -= 1;
        month += 12;
    }

    let a = year / 100;
    let b = 2 - a + a / 4;

    let jd = (365.25 * (year + 4716) as f64).floor()
        + (30.6001 * (month + 1) as f64).floor() as f64
        + day as f64
        + b as f64
        - 1524.5;

    jd
}

// TODO: write tests
