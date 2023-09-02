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
    Serve,
    Calculate(commands::calculate::Args),
    Daemon(commands::daemon::Args),
}

pub async fn run(args: Args) {
    match args.command {
        SubCommands::Serve => commands::serve::serve().await,
        SubCommands::Calculate(c) => commands::calculate::run(c),
        SubCommands::Daemon(d) => commands::daemon::run(d).await,
    }
}
