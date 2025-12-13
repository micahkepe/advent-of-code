/// xtask for AoC day automation.
use anyhow::Context;
use clap::Parser;
use std::{env, ops::RangeInclusive, path::PathBuf, time};

/// Starting year for AoC.
const AOC_YEAR_START: u16 = 2015;

/// Advent of Code code template
const TEMPLATE: &str = r#"
fn main() -> anyhow::Result<()> {
    let contents = std::fs::read_to_string("./data/day-<DAY>-input.txt")?;

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_example() {
        // TODO: fill me in
    }

    #[test]
    fn test_part2_example() {
        // TODO: fill me in
    }
}
"#;

/// Automation to create a new day for Advent of Code.
#[derive(Parser)]
#[command(about, long_about = None)]
struct Args {
    /// Day number (1..=25).
    #[arg(short, long, value_name = "INTEGER", value_parser=clap::value_parser!(u8).range(1..=25))]
    day: u8,

    /// Year. Defaults to current year or detected from current directory.
    #[arg(short, long)]
    year: Option<u16>,
}

/// Finds the path for the specified year, if it exists.
fn find_year_dir(year: u16) -> anyhow::Result<PathBuf> {
    let current_dir = env::current_dir()?;
    let year_str = year.to_string();

    if current_dir.ends_with(&year_str) {
        return Ok(current_dir);
    }

    let year_path = current_dir.join(&year_str);
    if year_path.exists() {
        return Ok(year_path);
    }

    if let Some(parent) = current_dir.parent() {
        let year_path = parent.join(&year_str);
        if year_path.exists() {
            return Ok(year_path);
        }
    }

    anyhow::bail!(
        "Could not find directory for year {}, first run `cargo new --bin <YEAR>` from the root of the project",
        year
    )
}

/// Detects the year from current directory, if possible.
fn detect_year_from_cwd(current_year: u16) -> Option<u16> {
    env::current_dir()
        .ok()?
        .file_name()?
        .to_str()?
        .parse::<u16>()
        .ok()
        .filter(|year| (AOC_YEAR_START..=current_year).contains(year))
}

/**
Entry point.
*/
fn main() -> anyhow::Result<()> {
    let now = time::SystemTime::now();
    let args = Args::parse();

    env_logger::builder()
        .filter_level(log::LevelFilter::Trace)
        .init();

    let day = args.day;
    let current_year = jiff::Zoned::now().year() as u16;

    let year = if let Some(year) = args.year {
        let valid_years = RangeInclusive::new(AOC_YEAR_START, current_year);
        if !valid_years.contains(&year) {
            anyhow::bail!("Invalid year: '{}'. Needs to be in {:?}", year, valid_years);
        }
        year
    } else {
        detect_year_from_cwd(current_year).unwrap_or(current_year)
    };

    log::info!("Creating day {:02} for year {}", day, year);

    let year_dir = find_year_dir(year)?;
    let src_bin_dir = year_dir.join("src").join("bin");
    let data_dir = year_dir.join("data");

    std::fs::create_dir_all(&src_bin_dir).context("Failed to create src/bin directory")?;
    std::fs::create_dir_all(&data_dir).context("Failed to create data directory")?;

    let day_file = src_bin_dir.join(format!("day_{:02}.rs", day));
    let data_file = data_dir.join(format!("day-{:02}-input.txt", day));

    if day_file.exists() {
        log::warn!(
            "Day {:02} binary already exists at {}, skipping...",
            day,
            day_file.display()
        )
    } else {
        let contents = TEMPLATE
            .replace("<DAY>", &format!("{:02}", day))
            .trim()
            .to_string();
        std::fs::write(&day_file, &contents)?;
        log::info!("Created {}", day_file.display());
    }

    if data_file.exists() {
        log::warn!(
            "Day {:02} data file already exists at {}, skipping...",
            day,
            data_file.display()
        )
    } else {
        // TODO: fetch the data from the AoC API and populate
        std::fs::write(&data_file, "")?;
        log::info!("Created {}", data_file.display());
    }

    log::info!("Completed in {:.2} seconds", now.elapsed()?.as_secs_f64());

    Ok(())
}
