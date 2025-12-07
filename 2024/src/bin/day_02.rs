use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn is_safe(record: &[i32]) -> bool {
    let diff_bounds: bool = record.windows(2).all(|wnd| {
        let diff = (wnd[1] - wnd[0]).abs();
        (1..=3).contains(&diff)
    });

    let is_monotomne: bool = record.windows(2).all(|wnd| wnd[1] <= wnd[0])
        || record.windows(2).all(|wnd| wnd[1] > wnd[0]);

    diff_bounds && is_monotomne
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("data/day_02_input.txt")?;
    let reader = BufReader::new(file);

    let mut num_safe: i32 = 0;

    for report in reader.lines() {
        let data: Vec<i32> = report?
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        if is_safe(&data) {
            num_safe += 1;
        } else {
            // Part 2: allow at most one violation of the safety rules
            for i in 0..=data.len() - 1 {
                let mut data_mod = data.clone();
                data_mod.remove(i);
                if is_safe(&data_mod) {
                    num_safe += 1;
                    break;
                }
            }
        }
    }

    println!("Number of safe reports: {}", num_safe);

    Ok(())
}
