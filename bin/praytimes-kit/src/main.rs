use std::error::Error;

use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = praytimes_kit::Args::parse();
    praytimes_kit::run(args).await;
    Ok(())
}
