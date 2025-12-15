/// xtask for AoC day automation.
use clap::{Parser, Subcommand};
use owo_colors::OwoColorize;
use std::time;

mod generate;

/// Automation to create a new day for Advent of Code.
#[derive(Parser)]
#[command(about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Gen {
        /// Day number (1..=25).
        #[arg(short, long, value_name = "INTEGER", value_parser=clap::value_parser!(u8).range(1..=25))]
        day: u8,

        /// Year. Defaults to current year or detected from current directory.
        #[arg(short, long)]
        year: Option<u16>,
    },
}

/**
Entry point.
*/
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let now = time::SystemTime::now();
    let args = Args::parse();
    env_logger::builder().filter_level(log::LevelFilter::Trace).init();

    match args.command {
        Commands::Gen { day, year } => generate::generate(day, year).await?,
    }

    log::info!(
        "Completed in {:.2} seconds ✨",
        now.elapsed()?.as_secs_f64().green()
    );

    Ok(())
}
