pub mod cli;
use chrono::Local;
use praytimes::{
    types::{FormattedTimes, Location},
    Calculator,
};

pub fn run(args: cli::Args) {
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
