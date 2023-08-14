pub fn sin(degrees: f64) -> f64 {
    degrees.to_radians().sin()
}

pub fn cos(degrees: f64) -> f64 {
    degrees.to_radians().cos()
}

pub fn tan(degrees: f64) -> f64 {
    degrees.to_radians().tan()
}

pub fn arcsin(x: f64) -> f64 {
    x.asin().to_degrees()
}

pub fn arccos(x: f64) -> f64 {
    x.acos().to_degrees()
}

pub fn arctan(x: f64) -> f64 {
    x.atan().to_degrees()
}

pub fn arccot(x: f64) -> f64 {
    (1.0 / x).atan().to_degrees()
}

pub fn arctan2(y: f64, x: f64) -> f64 {
    y.atan2(x).to_degrees()
}
