use praytimes::{
    types::{Location, TuneOffsets},
    Calculator,
};
use serde::{Deserialize, Serialize};
use std::{matches, path::PathBuf};

use chrono::{Datelike, Local, NaiveDate, Utc};
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
        let now = Utc::now().naive_utc();
        if matches!(times.imsak,Some(imsak) if imsak > now) {
            println!("----------")
        }
        println!("imsak\t\t{}", formatted.imsak.unwrap_or("-----".into()));
        if matches!((times.imsak,times.fajr),(Some(imsak),Some(fajr)) if imsak < now && now< fajr) {
            println!("----------")
        }
        println!("fajr\t\t{}", formatted.fajr.unwrap_or("-----".into()));
        if matches!((times.fajr,times.sunrise),(Some(a),Some(b)) if a < now && now< b) {
            println!("----------")
        }
        println!("sunrise\t\t{}", formatted.sunrise.unwrap_or("-----".into()));
        if matches!((times.sunrise,times.dhuhr),(Some(a),Some(b)) if a < now && now< b) {
            println!("----------")
        }
        println!("dhuhr\t\t{}", formatted.dhuhr.unwrap_or("-----".into()));
        if matches!((times.dhuhr,times.asr),(Some(a),Some(b)) if a < now && now< b) {
            println!("----------")
        }
        println!("asr\t\t{}", formatted.asr.unwrap_or("-----".into()));
        if matches!((times.asr,times.sunset),(Some(a),Some(b)) if a < now && now< b) {
            println!("----------")
        }
        println!("sunset\t\t{}", formatted.sunset.unwrap_or("-----".into()));
        if matches!((times.sunset,times.maghrib),(Some(a),Some(b)) if a < now && now< b) {
            println!("----------")
        }
        println!("maghrib\t\t{}", formatted.maghrib.unwrap_or("-----".into()));

        if matches!((times.maghrib,times.isha),(Some(a),Some(b)) if a < now && now< b) {
            println!("----------")
        }
        println!("isha\t\t{}", formatted.isha.unwrap_or("-----".into()));
        if matches!((times.isha,times.midnight),(Some(a),Some(b)) if a < now && now< b) {
            println!("----------")
        }
        println!("midnight\t{}", formatted.midnight.unwrap_or("-----".into()));
        if matches!(times.midnight,Some(a) if a < now ) {
            println!("----------")
        }
    }
}
