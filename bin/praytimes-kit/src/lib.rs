use clap::Parser;
use cli::run_cli;
use command::MainArgs;
use serve::serve;

mod base;
mod cli;
pub mod command;
mod serve;

pub async fn run_app() {
    let args = MainArgs::parse();

    match args.command {
        command::MainCommand::Serve => serve().await,
        command::MainCommand::Calculate(c) => run_cli(c),
    };
}
