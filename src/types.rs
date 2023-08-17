use chrono::NaiveDateTime;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum MidnightMethod {
    Standard,
    Jafari,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum HighLatsMethod {
    None,
    NightMiddle,
    OneSeventh,
    AngleBased,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Format {
    H24,   // 24-hour
    H12,   // 12-hour
    H12NS, // 12-hour no suffix
    Float, // Floating point number
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Degrees {
    pub degree: f64,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Minutes {
    pub minutes: f64,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AsrFactor {
    pub factor: f64,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CalculationUnit {
    #[cfg_attr(feature = "serde", serde(rename = "degree"))]
    Degrees(Degrees),

    #[cfg_attr(feature = "serde", serde(rename = "minutes"))]
    Minutes(Minutes),
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
    #[cfg_attr(feature = "serde", serde(rename = "highLats"))]
    pub high_latitudes: HighLatsMethod,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Location {
    pub longitude: f64,
    pub latitude: f64,
    #[cfg_attr(feature = "serde", serde(default))]
    pub elevation: f64,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct PraytimesOutput {
    pub imsak: Option<NaiveDateTime>,
    pub fajr: Option<NaiveDateTime>,
    pub sunrise: Option<NaiveDateTime>,
    pub dhuhr: Option<NaiveDateTime>,
    pub asr: Option<NaiveDateTime>,
    pub sunset: Option<NaiveDateTime>,
    pub maghrib: Option<NaiveDateTime>,
    pub isha: Option<NaiveDateTime>,
    pub midnight: Option<NaiveDateTime>,
}
