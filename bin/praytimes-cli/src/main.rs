use clap::Parser;
use praytimes_cli::{cli::Args, run};

fn main() {
    let args: Args = Args::parse();
    run(args);
}
