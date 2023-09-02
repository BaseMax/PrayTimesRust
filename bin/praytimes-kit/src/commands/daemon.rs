use std::{ops::Add, path::PathBuf};

use crate::base::CustomizableParams;
use chrono::{Datelike, Duration, Local, NaiveDate, NaiveDateTime, Utc};
use clap::Parser;
use praytimes::{
    types::{Location, PraytimesOutput, TuneOffsets},
    Calculator,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Parser)]
pub struct Args {
    config: PathBuf,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Config {
    location: Location,
    params: CustomizableParams,
    tune: Option<TuneOffsets>,
    commands: Vec<PraytimeCmd>,
}

#[derive(Debug, Copy, Eq, PartialEq, PartialOrd, Ord, Hash, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Praytime {
    Imsak,
    Fajr,
    Sunrise,
    Dhuhr,
    Asr,
    Sunset,
    Maghrib,
    Isha,
    Midnight,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
struct PraytimeCmd {
    time_diff: i32,
    cmd: String,
    praytime: Praytime,
}

pub async fn run(args: Args) {
    dbg!(&args);
    let config = std::fs::read(&args.config).expect("could not read config file");
    let config: Config = serde_json::from_slice(&config).expect("invalid json file");

    let calculator = Calculator::new(config.params.get_params(), config.tune.unwrap_or_default());

    let today = Local::now();
    let today = NaiveDate::from_ymd_opt(today.year(), today.month(), today.day()).unwrap();

    let cal = [today.pred_opt().unwrap(), today, today.succ_opt().unwrap()]
        .into_iter()
        .map(|d| into_vec(calculator.calculate(&config.location, &d)))
        .flatten()
        .collect::<Vec<_>>();

    let data = config
        .commands
        .into_iter()
        .map(|c| {
            cal.iter()
                .filter(|a| a.0 == c.praytime)
                .map(|(t, d)| {
                    (
                        c.clone(),
                        t.clone(),
                        d.add(Duration::seconds(c.time_diff as i64)),
                    )
                })
                .filter(|(_, _, d)| d > &Utc::now().naive_utc())
                .collect::<Vec<_>>()
                .into_iter()
        })
        .flatten()
        .collect::<Vec<_>>();

    dbg!(&data);

    let data = data
        .into_iter()
        .map(|(c, _, d)| {
            let dur = d.signed_duration_since(Utc::now().naive_utc());
            tokio::spawn(async move {
                println!("waiting {dur}");
                tokio::time::sleep(dur.to_std().unwrap()).await;

                let a = tokio::process::Command::new("sh")
                    .arg("-c")
                    .arg(&c.cmd)
                    .spawn();
                match a {
                    Ok(mut a) => match a.wait().await {
                        Ok(_) => println!("successfully ran command for {:?}", c),
                        Err(_) => todo!(),
                    },
                    Err(e) => println!("failed to spawn, {e}"),
                }
            })
        })
        .collect::<Vec<_>>();

    for d in data {
        d.await.unwrap()
    }
}

fn into_vec(times: PraytimesOutput) -> Vec<(Praytime, NaiveDateTime)> {
    use Praytime::*;
    let a = vec![
        (Imsak, times.imsak),
        (Fajr, times.fajr),
        (Sunrise, times.sunrise),
        (Dhuhr, times.dhuhr),
        (Asr, times.asr),
        (Sunset, times.sunset),
        (Maghrib, times.maghrib),
        (Isha, times.isha),
        (Midnight, times.midnight),
    ];
    return a
        .into_iter()
        .filter_map(|(t, p)| p.map(|p| (t, p)))
        .collect();
}
