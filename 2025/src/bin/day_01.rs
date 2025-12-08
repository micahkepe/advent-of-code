use anyhow::Context;
use std::io::{BufRead, BufReader};

fn position_change(input: &str) -> anyhow::Result<isize> {
    let mut sign = 1;
    match input.chars().next() {
        Some('L') => sign *= -1,
        Some('R') => (),
        Some(_) | None => return Err(anyhow::anyhow!("unexpected input line: {}", input)),
    };

    let change: isize = input[1..]
        .parse()
        .with_context(|| format!("Invalid number in input: {}", input))?;
    Ok(change * sign)
}

fn main() -> anyhow::Result<()> {
    // dial starts at 50
    let mut position: usize = 50;
    let f = std::fs::File::open("./data/day-01-input.txt")?;
    let f = BufReader::new(f);
    let mut password = 0;
    for line in f.lines() {
        let line = line.expect("invalid line");
        let delta = position_change(&line)?;
        position = (position as isize + delta).rem_euclid(100) as usize;

        if position == 0 {
            password += 1;
        }
    }

    println!("Password: {}", password);

    Ok(())
}
