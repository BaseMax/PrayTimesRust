mod base;
mod commands;
use clap::{Parser, Subcommand};
#[derive(Parser, Debug, Clone)]
pub struct Args {
    #[command(subcommand)]
    command: SubCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum SubCommands {
    /// http API for praytime calculation
    Serve,
    /// simple cli to show praytimes for specific day
    Calculate(commands::calculate::Args),

    /// daemon which helps with notification and similar works on praytimes
    Daemon(commands::daemon::Args),

    /// get next praytime event 
    Next(commands::next::Args)
}

pub async fn run(args: Args) {
    match args.command {
        SubCommands::Serve => commands::serve::serve().await,
        SubCommands::Calculate(c) => commands::calculate::run(c),
        SubCommands::Daemon(d) => commands::daemon::run(d).await,
        SubCommands::Next(n) => commands::next::run(n),
    }
}
