use anyhow::Context;
use std::io::{BufRead, BufReader};

fn position_change(input: &str) -> anyhow::Result<isize> {
    let mut sign = 1;
    match input.chars().next() {
        Some('L') => sign *= -1,
        Some('R') => (),
        Some(_) | None => {
            return Err(anyhow::anyhow!("unexpected input line: {}", input));
        }
    };

    let change: isize = input[1..]
        .parse()
        .with_context(|| format!("Invalid number in input: {}", input))?;
    Ok(change * sign)
}

fn main() -> anyhow::Result<()> {
    // dial starts at 50
    let mut position: isize = 50;
    let f = std::fs::File::open("./data/day-01-input.txt")?;
    let f = BufReader::new(f);
    let mut password_1 = 0;
    let mut password_2 = 0;

    for line in f.lines() {
        let line = line.expect("invalid line");
        let delta = position_change(&line)?;
        let old_pos = position;
        let new_pos = (position + delta).rem_euclid(100);

        // Part 1
        if new_pos == 0 {
            password_1 += 1;
        }

        // Part 2
        let zero_crossings = if delta == 0 {
            0
        } else {
            let steps = delta.abs();
            let mut count = 0;
            for i in 1..=steps {
                let pos = if delta > 0 {
                    (old_pos + i).rem_euclid(100)
                } else {
                    (old_pos - i).rem_euclid(100)
                };

                if pos == 0 {
                    count += 1
                }
            }

            count
        };
        password_2 += zero_crossings;

        position = new_pos
    }

    println!("Part 1 password: {}", password_1);
    println!("Part 2 password: {}", password_2);

    Ok(())
}
