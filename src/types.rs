use chrono::{DateTime, Local};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum MidnightMethod {
    Standard,
    Jafari,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum HighLatsMethod {
    None,
    NightMiddle,
    OneSeventh,
    AngleBased,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Format {
    H24,   // 24-hour
    H12,   // 12-hour
    H12NS, // 12-hour no suffix
    Float, // Floating point number
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Degrees(pub f64);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Minutes(pub f64);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AsrFactor(pub f64);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CalculationUnit {
    Degrees(Degrees),
    Minutes(Minutes),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameters {
    // sun angle below the horizon or minutes before fajr
    pub imsak: CalculationUnit,

    // sun angle below the horizon to calculate fajr
    pub fajr: Degrees,

    // minutes after (or before with negative numbers) the midday
    pub dhuhr: Minutes,

    // asr shadow factor
    pub asr: AsrFactor,

    // sun angle below the horizon or minutes after sunset
    pub maghrib: CalculationUnit,

    // sun angle below the horizon or minutes after maghrib
    pub isha: CalculationUnit,

    // midnight calculation method
    pub midnight: MidnightMethod,

    // adjustment method for higher latitudes
    pub high_latitudes: HighLatsMethod,
}
pub struct Praytimes {}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Location {
    pub longitude: f64,
    pub latitude: f64,
    pub elevation: f64,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct PraytimesOutput {
    pub imsak: Option<DateTime<Local>>,
    pub fajr: Option<DateTime<Local>>,
    pub sunrise: Option<DateTime<Local>>,
    pub dhuhr: Option<DateTime<Local>>,
    pub asr: Option<DateTime<Local>>,
    pub sunset: Option<DateTime<Local>>,
    pub maghrib: Option<DateTime<Local>>,
    pub isha: Option<DateTime<Local>>,
    pub midnight: Option<DateTime<Local>>,
}
