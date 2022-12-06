use clap::*;
use dotenv;
use pretty_env_logger;

mod cli;
mod tools;
use cli::*;

fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let args = Cli::parse();
    run_command(args)?;

    Ok(())
}
