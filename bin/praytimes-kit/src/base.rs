use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Zone {
    Local,
    Utc,
    Fixed(i32),
}

use praytimes::{
    methods,
    types::{
        AsrFactor, CalculationUnit, Degrees, HighLatsMethod, MidnightMethod, Minutes, Parameters,
    },
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, ValueEnum)]
pub enum SelectedMethod {
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
pub enum CustomizableParams {
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
