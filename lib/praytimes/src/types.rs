use chrono::NaiveDateTime;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// midnight calculation method
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum MidnightMethod {
    /// The mean time from Sunset to Sunrise
    Standard,
    /// The mean time from Maghrib to Fajr
    Jafari,
}
/// higher latitudes adjustment methods [more info](http://praytimes.org/wiki/Calculation#Higher_Latitudes) |
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum HighLatsMethod {
    /// No adjustments
    None,
    /// The middle of the night method
    NightMiddle,
    /// The 1/7th of the night method
    OneSeventh,
    /// The angle-based method (recommended)
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

/// Asr factor ([more info](http://praytimes.org/calculation#Asr))
/// - Standard  -  Shafii, Maliki, Jafari and Hanbali (shadow factor = 1)
/// - Hanafi - Hanafi school of thought (shadow factor = 2)
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

/// Parameters for the calculation
/// normally you wouldn't specify these manually and use a method instead (see [`crate::methods`])
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct Parameters {
    /// sun angle below the horizon or minutes before fajr
    pub imsak: CalculationUnit,

    /// sun angle below the horizon to calculate fajr
    pub fajr: Degrees,

    /// minutes after (or before with negative numbers) the midday
    pub dhuhr: Minutes,

    /// asr shadow factor
    pub asr: AsrFactor,

    /// sun angle below the horizon or minutes after sunset
    pub maghrib: CalculationUnit,

    /// sun angle below the horizon or minutes after maghrib
    pub isha: CalculationUnit,

    /// midnight calculation method
    pub midnight: MidnightMethod,

    /// adjustment method for higher latitudes (see [`HighLatsMethod`])
    #[cfg_attr(feature = "serde", serde(rename = "highLats"))]
    pub high_latitudes: HighLatsMethod,
}

/// Specifies the geographic coordinates and elevation of a location.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Location {
    /// The latitude coordinate in degrees.
    /// Should be a number between -90 and 90.
    pub latitude: f64,

    /// The longitude coordinate in degrees.
    /// Should be a number between -180 and 180.
    pub longitude: f64,

    /// The elevation of the location in meters with respect to surrounding terrain.
    /// Should be a positive number.
    #[cfg_attr(feature = "serde", serde(default))]
    pub elevation: f64,
}

/// calculated prayertimes
///
/// and if doesn't exists ( for example in high latitudes ) None
/// you can easily convert them to Datetime<Utc> or DateTime<Local> using
/// ```rust,ignore
/// times = /* calculate here */
/// let a: DateTime<Utc>  = times.imsak.into()
/// ```
///
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct PraytimesOutput {
    /// The beginning time for morning prayers.
    /// Marks the start of the fasting period.
    pub imsak: Option<NaiveDateTime>,

    /// The time when the sky begins to lighten at dawn.
    pub fajr: Option<NaiveDateTime>,

    /// The time when the sky begins to lighten at dawn.
    pub sunrise: Option<NaiveDateTime>,

    /// Noon prayer time.
    /// When the sun begins to decline after reaching its zenith.
    pub dhuhr: Option<NaiveDateTime>,

    /// Afternoon prayer.
    /// Based on the length of shadows compared to noon.
    pub asr: Option<NaiveDateTime>,

    /// When the sun disappears below the horizon.
    pub sunset: Option<NaiveDateTime>,

    /// Evening prayer time.
    /// Shortly after sunset.
    pub maghrib: Option<NaiveDateTime>,

    /// Night prayer time.
    /// When darkness has fallen with no scattered light.
    pub isha: Option<NaiveDateTime>,

    /// Midnight
    /// calculation based on sunset to sunrise ( or fajr in some methods ).
    pub midnight: Option<NaiveDateTime>,
}

/// tuning  offsets in minutes for precaution
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TuneOffsets {
    pub imsak: Option<f64>,
    pub fajr: Option<f64>,
    pub sunrise: Option<f64>,
    pub dhuhr: Option<f64>,
    pub asr: Option<f64>,
    pub sunset: Option<f64>,
    pub maghrib: Option<f64>,
    pub isha: Option<f64>,
    pub midnight: Option<f64>,
}
