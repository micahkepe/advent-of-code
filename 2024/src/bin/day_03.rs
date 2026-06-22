use regex::Regex;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("data/day_03_input.txt")?;

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)")?;
    let mut part1_sum: i64 = 0;
    let mut part2_sum: i64 = 0;
    let mut enabled = true;

    for cap in re.captures_iter(&input) {
        let token = cap.get(0).unwrap().as_str();
        if token == "do()" {
            enabled = true;
        } else if token == "don't()" {
            enabled = false;
        } else {
            let x: i64 = cap[1].parse()?;
            let y: i64 = cap[2].parse()?;
            part1_sum += x * y;
            if enabled {
                part2_sum += x * y;
            }
        }
    }

    println!("PART 1: {}", part1_sum);
    println!("PART 2: {}", part2_sum);

    Ok(())
}
