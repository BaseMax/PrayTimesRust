use std::error::Error;

use clap::Parser;
use env_logger::Env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let env = Env::new()
        .filter("PRAYTIMES_LOG")
        .write_style("PRAYTIMES_LOG_STYLE");

    env_logger::init_from_env(env);

    let args = praytimes_kit::Args::parse();
    praytimes_kit::run(args).await;
    Ok(())
}
