use chrono::Local;
use praytimes::{
    types::{FormattedTimes, Location},
    Calculator,
};

pub mod command {
    use crate::base::SelectedMethod;
    use chrono::{Datelike, Local, NaiveDate};
    use clap::Parser;

    /// cli Prayertime calculator
    #[derive(Parser, Debug, Clone)]
    #[command(author, version, about, long_about = None)]
    pub struct CalculatorCliArgs {
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
}

pub fn run_cli(args: command::CalculatorCliArgs) {
    let times = Calculator::new(args.method.get_method(), Default::default()).calculate(
        &Location {
            latitude: args.latitude,
            longitude: args.longitude,
            elevation: args.elevation,
        },
        &args.date,
    );
    let formatted = times.format_times(&args.format, &Local);
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
