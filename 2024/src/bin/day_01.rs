use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();

    let file = File::open("data/day_01_input.txt")?;
    let reader = BufReader::new(file);

    // PART A

    for line in reader.lines() {
        let line = line?;
        let mut words = line.split_whitespace();

        let left_id = words.next().ok_or("No left")?.parse::<i32>()?;
        let right_id = words.next().ok_or("No left")?.parse::<i32>()?;

        left_list.push(left_id);
        right_list.push(right_id);
    }

    assert!(left_list.len() == right_list.len());

    // println!("Head of left list {:?}", &left_list[0..10]);
    // println!("Head of right list {:?}", &right_list[0..10]);

    left_list.sort_by(|a, b| b.cmp(a));
    right_list.sort_by(|a, b| b.cmp(a));

    let mut distance: i32 = 0;

    let mut left_list_copy = left_list.clone();
    let mut right_list_copy = right_list.clone();

    while !left_list_copy.is_empty() {
        let diff = (left_list_copy.pop().unwrap()
            - right_list_copy.pop().unwrap())
        .abs();
        distance += diff;
    }

    println!("Total distance: {}", distance);

    // PART B

    let mut total_similarity: i32 = 0;
    let mut similarity_map: HashMap<i32, i32> = HashMap::new();

    for item in left_list.iter() {
        similarity_map.insert(*item, 0);
    }

    for item in right_list.iter() {
        if let Some(val) = similarity_map.get_mut(item) {
            *val += 1;
        }
    }

    for (key, value) in similarity_map.iter() {
        total_similarity += key * value;
    }

    print!("Total similarity: {}", total_similarity);

    Ok(())
}
