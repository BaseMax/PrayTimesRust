use chrono::Datelike;
use praytimes::methods;

use clap::{Parser, ValueEnum};
use praytimes::types::Parameters;

use chrono::Local;

use chrono::NaiveDate;

/// cli Prayertime calculator
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// calculation methods
    #[arg(short, long)]
    pub(crate) method: SelectedMethod,

    /// location's latitude
    #[arg(long)]
    pub(crate) latitude: f64,

    /// location's longitude
    #[arg(long)]
    pub(crate) longitude: f64,

    /// elevation from surrounding terrain
    #[arg(long, short, default_value_t = 0.0)]
    pub(crate) elevation: f64,

    /// date for calculation ( default is today )
    #[arg(short, long,default_value_t=get_today())]
    pub(crate) date: NaiveDate,

    /// strftime compatible format
    #[arg(short, long, default_value = "%H:%M:%S")]
    pub(crate) format: String,

    #[arg(short, long, default_value_t = false)]
    pub(crate) json: bool,
}

pub(crate) fn get_today() -> NaiveDate {
    let date_time = Local::now();
    NaiveDate::from_ymd_opt(date_time.year(), date_time.month(), date_time.day()).unwrap()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
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
