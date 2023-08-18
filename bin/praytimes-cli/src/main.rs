use chrono::{Date, Datelike, Local, NaiveDate};
use clap::{Parser, Subcommand, ValueEnum};

use praytimes::{types::Parameters, Calculator};
/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    method: SelectedMethod,

    #[arg(long)]
    latitude: f64,

    #[arg(long)]
    longitude: f64,

    #[arg(long, short, default_value_t = 0.0)]
    elevation: f64,

    #[arg(short, long,default_value_t=get_today())]
    date: NaiveDate,
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

fn main() {
    let a: Args = Args::parse();
    dbg!(a);
}
