use axum::Json;
use chrono::{FixedOffset, Local, NaiveDate, Utc};
use praytimes::{
    methods,
    types::{
        AsrFactor, CalculationUnit, Degrees, FormattedTimes, HighLatsMethod, Location,
        MidnightMethod, Minutes, Parameters, TuneOffsets,
    },
    Calculator,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CalculationInputs {
    #[serde(default = "default_format")]
    format: String,
    date: NaiveDate,
    location: Location,
    parameters: CustomizableParams,
    tuning: Option<TuneOffsets>,
    #[serde(default = "default_timezone")]
    zone: Zone,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
enum Zone {
    Local,
    Utc,
    Fixed(i32),
}
fn default_timezone() -> Zone {
    Zone::Local
}
fn default_format() -> String {
    "%+".into()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub(crate) enum SelectedMethod {
    MWL,
    ISNA,
    Egypt,
    Makkah,
    Karachi,
    Tehran,
    Jafari,
}

impl SelectedMethod {
    pub(crate) fn get_method(&self) -> Parameters {
        match self {
            SelectedMethod::MWL => methods::MUSLIM_WORLD_LEAGUE,
            SelectedMethod::ISNA => methods::ISLAMIC_SOCIETY_OF_NORTH_AMERICA,
            SelectedMethod::Egypt => methods::EGYPTIAN_GENERAL_AUTHORITY_OF_SURVEY,
            SelectedMethod::Makkah => methods::UMM_AL_QURA_UNIVERSITY_MAKKAH,
            SelectedMethod::Karachi => methods::UNIVERSITY_OF_ISLAMIC_SCIENCES_KARACHI,
            SelectedMethod::Tehran => methods::INSTITUTE_OF_GEOPHYSICS_UNIVERSITY_OF_TEHRAN,
            SelectedMethod::Jafari => methods::SHIA_ITHNA_ASHARI_LEVA_INSTITUTE_QUM,
        }
    }
}
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
enum CustomizableParams {
    Full(Parameters),
    MethodAndExtra {
        method: SelectedMethod,
        extra: Option<PartialParameters>,
    },
}
impl CustomizableParams {
    pub fn get_params(self) -> Parameters {
        match self {
            CustomizableParams::Full(p) => p,
            CustomizableParams::MethodAndExtra {
                method,
                extra: Some(partial),
            } => {
                let method = method.get_method();
                Parameters {
                    imsak: partial.imsak.unwrap_or(method.imsak),
                    fajr: partial.fajr.unwrap_or(method.fajr),
                    dhuhr: partial.dhuhr.unwrap_or(method.dhuhr),
                    asr: partial.asr.unwrap_or(method.asr),
                    maghrib: partial.maghrib.unwrap_or(method.maghrib),
                    isha: partial.isha.unwrap_or(method.isha),
                    midnight: partial.midnight.unwrap_or(method.midnight),
                    high_latitudes: partial.high_latitudes.unwrap_or(method.high_latitudes),
                }
            }
            CustomizableParams::MethodAndExtra {
                method,
                extra: None,
            } => method.get_method(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PartialParameters {
    pub imsak: Option<CalculationUnit>,
    pub fajr: Option<Degrees>,
    pub dhuhr: Option<Minutes>,
    pub asr: Option<AsrFactor>,
    pub maghrib: Option<CalculationUnit>,
    pub isha: Option<CalculationUnit>,
    pub midnight: Option<MidnightMethod>,
    #[serde(rename = "highLats")]
    pub high_latitudes: Option<HighLatsMethod>,
}
pub async fn calculate_handler(Json(payload): Json<CalculationInputs>) -> Json<FormattedTimes> {
    let result = Calculator::new(
        payload.parameters.get_params(),
        payload.tuning.unwrap_or_default(),
    )
    .calculate(&payload.location, &payload.date);

    match payload.zone {
        Zone::Local => result.format_times(&payload.format, &Local),
        Zone::Utc => result.format_times(&payload.format, &Utc),
        Zone::Fixed(o) => result.format_times(&payload.format, &FixedOffset::east_opt(o).unwrap()),
    }
    .into()
}
