use clap::{Parser, Subcommand};

use crate::cli::command::CalculatorCliArgs;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct MainArgs {
    #[command(subcommand)]
    pub command: MainCommand,
}

#[derive(Subcommand, Debug, Clone)]
pub enum MainCommand {
    Serve,
    Calculate(CalculatorCliArgs),
}
