use std::path::PathBuf;

use chrono::{Local, Utc};
use clap::Parser;
use praytimes::{
    types::{format_time, Location, TuneOffsets},
    Calculator,
};
use serde::{Deserialize, Serialize};

use crate::base::CustomizableParams;
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// configuration file
    #[arg(short, long)]
    config: PathBuf,

    /// strftime compatible format ( overwrites the config file's format field )
    #[arg(short, long)]
    pub format: Option<String>,

    /// whether to output as json format or not
    #[arg(short, long, default_value_t = false)]
    pub json: bool,
}
fn default_format() -> String {
    "%T".into()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Config {
    location: Location,
    parameters: CustomizableParams,
    tune: Option<TuneOffsets>,
    #[serde(default = "default_format")]
    format: String,
}

pub fn run(args: Args) {
    let conf = std::fs::read(args.config).expect("failed to open file");
    let conf: Config = serde_json::from_slice(&conf).expect("failed to read configuration");

    let now = Utc::now().naive_utc();
    let cal = Calculator::new(conf.parameters.get_params(), conf.tune.unwrap_or_default());
    let date = [
        now.date().pred_opt().unwrap(),
        now.date(),
        now.date().succ_opt().unwrap(),
    ]
    .iter()
    .map(|d| cal.calculate(&conf.location, &d).into_vec())
    .flatten()
    .skip_while(|(_, d)| d < &now)
    .next();

    if let Some((t, d)) = date {
        println!(
            "{t}: {}",
            format_time(d, &args.format.unwrap_or(conf.format), &Local)
        )
    }
}
