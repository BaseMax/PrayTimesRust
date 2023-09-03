use std::{ops::Add, path::PathBuf, process::exit};

use crate::base::CustomizableParams;
use chrono::{Datelike, Duration, Local, NaiveDate, NaiveDateTime, Utc};
use clap::Parser;
use praytimes::{
    types::{format_time, Location, PraytimesOutput, TuneOffsets},
    Calculator,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Parser)]
pub struct Args {
    config: PathBuf,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Config {
    #[serde(default = "default_format")]
    format: String,
    location: Location,
    parameters: CustomizableParams,
    tune: Option<TuneOffsets>,
    commands: Vec<PraytimeCmd>,
}
fn default_format() -> String {
    "%T".into()
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

    if config.commands.is_empty() {
        eprintln!("no commands");
        exit(1);
    }

    let calculator = Calculator::new(
        config.parameters.get_params(),
        config.tune.clone().unwrap_or_default(),
    );

    let daemon = Daemon {
        calculator,
        commands: config.commands,
        location: config.location,
        format: config.format,
    };

    let today = Local::now();
    let today = NaiveDate::from_ymd_opt(today.year(), today.month(), today.day()).unwrap();
    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(24 * 60 * 60));

    daemon.execute_for_day(today.pred_opt().unwrap());
    daemon.execute_for_day(today);

    let mut next_day = today.succ_opt().unwrap();

    loop {
        daemon.execute_for_day(next_day);
        next_day = next_day.succ_opt().unwrap();
        interval.tick().await;
    }
}

struct Daemon {
    format: String,
    location: Location,
    commands: Vec<PraytimeCmd>,
    calculator: Calculator,
}
impl Daemon {
    fn execute_for_day(&self, next_day: NaiveDate) {
        let praytimes = into_vec(self.calculator.calculate(&self.location, &next_day));
        let commands_to_run = self.get_runnable_commands(praytimes);
        self.wait_and_run(commands_to_run);
    }
    fn wait_and_run(&self, commands_to_run: Vec<(PraytimeCmd, Praytime, NaiveDateTime)>) {
        for (command, p, date_time) in commands_to_run.into_iter() {
            let dur = date_time.signed_duration_since(Utc::now().naive_utc());
            let format = self.format.clone();
            tokio::spawn(async move {
                eprintln!(
                    "waiting for {:?} for duration of : '{}' to run command :\n >>  `{}`\n",
                    command.praytime, dur, command.cmd
                );
                tokio::time::sleep(dur.to_std().unwrap()).await;

                let child = tokio::process::Command::new("sh")
                    .arg("-c")
                    .env("TYPE", format!("{:?}", p))
                    .env("DIFF", format!("{}", command.time_diff))
                    .env("TIME", format!("{}", format_time(date_time, "%T", &Local)))
                    .arg(&command.cmd)
                    .spawn();
                match child {
                    Ok(mut a) => match a.wait().await {
                        Ok(_) => println!("successfully ran command for {:?}", command),
                        Err(_) => todo!(),
                    },
                    Err(e) => println!("failed to spawn, {e}"),
                }
            });
        }
    }

    fn get_runnable_commands(
        &self,
        praytimes: Vec<(Praytime, NaiveDateTime)>,
    ) -> Vec<(PraytimeCmd, Praytime, NaiveDateTime)> {
        let commands_to_run = self
            .commands
            .iter()
            .map(|c| {
                praytimes
                    .iter()
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
        commands_to_run
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
