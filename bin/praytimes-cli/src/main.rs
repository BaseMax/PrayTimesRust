use chrono::{Datelike, Local, NaiveDate, TimeZone};
use clap::{Parser, ValueEnum};
use serde::{Deserialize, Serialize};

use praytimes::{
    methods,
    types::{Location, Parameters, PraytimesOutput},
    Calculator,
};
/// cli Prayertime calculator
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// calculation methods
    #[arg(short, long)]
    method: SelectedMethod,

    /// location's latitude
    #[arg(long)]
    latitude: f64,

    /// location's longitude
    #[arg(long)]
    longitude: f64,

    /// elevation from surrounding terrain
    #[arg(long, short, default_value_t = 0.0)]
    elevation: f64,

    /// date for calculation ( default is today )
    #[arg(short, long,default_value_t=get_today())]
    date: NaiveDate,

    /// strftime compatible format
    #[arg(short, long, default_value = "%H:%M:%S")]
    format: String,

    #[arg(short, long, default_value_t = false)]
    json: bool,
}
fn get_today() -> NaiveDate {
    let date_time = Local::now();
    NaiveDate::from_ymd_opt(date_time.year(), date_time.month(), date_time.day()).unwrap()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum SelectedMethod {
    MWL,
    ISNA,
    Egypt,
    Makkah,
    Karachi,
    Tehran,
    Jafari,
}
impl SelectedMethod {
    fn get_method(&self) -> Parameters {
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
#[derive(Debug, Clone, Serialize, Deserialize)]
struct FormattedTimes {
    pub imsak: Option<String>,
    pub fajr: Option<String>,
    pub sunrise: Option<String>,
    pub dhuhr: Option<String>,
    pub asr: Option<String>,
    pub sunset: Option<String>,
    pub maghrib: Option<String>,
    pub isha: Option<String>,
    pub midnight: Option<String>,
}
fn format_times(times: PraytimesOutput, format: &str) -> FormattedTimes {
    FormattedTimes {
        imsak: times.imsak.map(|d| format_time(d, format)),
        fajr: times.fajr.map(|d| format_time(d, format)),
        sunrise: times.sunrise.map(|d| format_time(d, format)),
        dhuhr: times.dhuhr.map(|d| format_time(d, format)),
        asr: times.asr.map(|d| format_time(d, format)),
        sunset: times.sunset.map(|d| format_time(d, format)),
        maghrib: times.maghrib.map(|d| format_time(d, format)),
        isha: times.isha.map(|d| format_time(d, format)),
        midnight: times.midnight.map(|d| format_time(d, format)),
    }
}

fn format_time(time: chrono::NaiveDateTime, format: &str) -> String {
    let local = Local.from_utc_datetime(&time);
    let formatted = format!("{}", local.format(format));
    formatted
}

fn main() {
    let a: Args = Args::parse();
    let times = Calculator::new(a.method.get_method(), Default::default()).calculate(
        &Location {
            latitude: a.latitude,
            longitude: a.longitude,
            elevation: a.elevation,
        },
        &a.date,
    );
    let formatted = format_times(times, &a.format);
    if a.json {
        let json = serde_json::to_string_pretty(&formatted).unwrap();
        println!("{json}");
    } else {
        print_times(formatted);
    }
}

fn print_times(times: FormattedTimes) {
    println!("imsak\t\t{}", times.imsak.unwrap_or("-----".into()));
    println!("fajr\t\t{}", times.fajr.unwrap_or("-----".into()));
    println!("sunrise\t\t{}", times.sunrise.unwrap_or("-----".into()));
    println!("dhuhr\t\t{}", times.dhuhr.unwrap_or("-----".into()));
    println!("asr\t\t{}", times.asr.unwrap_or("-----".into()));
    println!("sunset\t\t{}", times.sunset.unwrap_or("-----".into()));
    println!("maghrib\t\t{}", times.maghrib.unwrap_or("-----".into()));
    println!("isha\t\t{}", times.isha.unwrap_or("-----".into()));
    println!("midnight\t{}", times.midnight.unwrap_or("-----".into()));
}
