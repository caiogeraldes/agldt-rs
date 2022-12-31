use clap::Parser;

mod cli;
mod tools;
use cli::{run_command, Cli};

fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let args = Cli::parse();
    run_command(args)?;

    Ok(())
}
