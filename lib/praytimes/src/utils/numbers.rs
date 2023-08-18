pub fn fix_angle(angle: f64) -> f64 {
    fix(angle, 360.0)
}

pub fn fix_hour(angle: f64) -> f64 {
    fix(angle, 24.0)
}

pub fn fix(mut num: f64, base: f64) -> f64 {
    num = num % base;

    if num < 0.0 {
        num + base
    } else {
        num
    }
}
