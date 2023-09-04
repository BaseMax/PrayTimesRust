use praytimes::{
    types::{FormattedTimes, Location, TuneOffsets},
    Calculator,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use chrono::{Datelike, Local, NaiveDate};
use clap::Parser;

use crate::base::CustomizableParams;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// configuration file
    #[arg(short, long)]
    config: PathBuf,

    /// date for calculation ( default is today )
    #[arg(short, long,default_value_t = get_today())]
    pub date: NaiveDate,

    /// strftime compatible format ( overwrites the config file's format field )
    #[arg(short, long)]
    pub format: Option<String>,

    /// whether to output as json format or not
    #[arg(short, long, default_value_t = false)]
    pub json: bool,
}

fn get_today() -> NaiveDate {
    let date_time = Local::now();
    NaiveDate::from_ymd_opt(date_time.year(), date_time.month(), date_time.day()).unwrap()
}
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Config {
    location: Location,
    parameters: CustomizableParams,
    tune: Option<TuneOffsets>,
    #[serde(default = "default_format")]
    format: String,
}

fn default_format() -> String {
    "%T".into()
}

pub fn run(args: Args) {
    let conf = std::fs::read(args.config).expect("failed to open file");
    let conf: Config = serde_json::from_slice(&conf).expect("failed to read configuration");

    let times = Calculator::new(conf.parameters.get_params(), conf.tune.unwrap_or_default())
        .calculate(&conf.location, &args.date);
    let formatted = times.format_times(&args.format.unwrap_or(conf.format), &Local);
    if args.json {
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
