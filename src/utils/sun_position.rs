use super::d_math::*;
use crate::utils::numbers::{fix_angle, fix_hour};

pub struct SunPosition {
    pub declination: f64,
    pub equation: f64,
}

pub fn sun_position(jd: f64) -> SunPosition {
    let d = jd - 2451545.0;

    let g = fix_angle(357.529 + 0.98560028 * d);

    let q = fix_angle(280.459 + 0.98564736 * d);

    let l = fix_angle(q + 1.915 * sin(g) + 0.020 * sin(g * 2.0));

    let e = 23.439 - 0.00000036 * d;

    let ra = arctan2(cos(e) * sin(l), cos(l)) / 15.0;

    let eqt = q / 15.0 - fix_hour(ra);

    let decl = arcsin(sin(e) * sin(l));

    SunPosition {
        declination: decl,
        equation: eqt,
    }
}
