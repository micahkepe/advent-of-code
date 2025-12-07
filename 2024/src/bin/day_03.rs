use regex::Regex;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

// only accept mul(X,Y), where X, Y are 1-3 digit numbers without surrounding
// whitespace (or trailing zeroes?)
// return the summation of the valid mul instructions

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("data/day_03_input.txt").expect("Input file doesn't exist");
    let reader = BufReader::new(file);

    let re = Regex::new(r"mul\(([1-9][0-9]{0,2}),([1-9][0-9]{0,2})\)")?;
    let mut total_sum = 0;

    for line in reader.lines() {
        let line = line?;

        for cap in re.captures_iter(&line) {
            let x: i32 = cap[1].parse()?;
            let y: i32 = cap[2].parse()?;
            total_sum += x * y;
        }
    }

    // TODO: part 2 with setting enable/disabled sections

    println!("Total sum of muls: {}", total_sum);

    Ok(())
}
