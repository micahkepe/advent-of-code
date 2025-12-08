fn is_invalid_id_part1(id: &str) -> bool {
    let len = id.len();
    if !len.is_multiple_of(2) {
        return false;
    }
    let mid = len / 2;
    id[..mid] == id[mid..]
}

fn is_invalid_id_part2(id: &str) -> bool {
    let len = id.len();
    (1..len)
        .filter(|&sub_size| len.is_multiple_of(sub_size))
        .any(|sub_size| id == id[0..sub_size].repeat(len / sub_size))
}

fn find_invalid_ids(line: &str, validator: fn(&str) -> bool) -> anyhow::Result<Vec<usize>> {
    let mut invalid_ids = vec![];
    for mut id_range in line.trim().split(',') {
        id_range = id_range.trim();
        if let Some((start, end)) = id_range.split_once('-') {
            let start: usize = start.parse()?;
            let end: usize = end.parse()?;
            for id in start..=end {
                if validator(&id.to_string()) {
                    invalid_ids.push(id);
                }
            }
        } else {
            return Err(anyhow::anyhow!("invalid id {}", id_range));
        }
    }
    Ok(invalid_ids)
}

fn main() -> anyhow::Result<()> {
    let contents = std::fs::read_to_string("./data/day-02-input.txt")?;

    // Part 1
    let invalid_ids = find_invalid_ids(&contents, is_invalid_id_part1)?;
    println!("Part 1: {}", invalid_ids.iter().sum::<usize>());

    // Part 2
    let invalid_ids = find_invalid_ids(&contents, is_invalid_id_part2)?;
    println!("Part 2: {}", invalid_ids.iter().sum::<usize>());

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_example() {
        let input = r"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
        1698522-1698528,446443-446449,38593856-38593862,565653-565659,
        824824821-824824827,2121212118-2121212124";
        let invalid_ids = find_invalid_ids(input, is_invalid_id_part1).unwrap();
        assert_eq!(invalid_ids.iter().sum::<usize>(), 1227775554)
    }

    #[test]
    fn test_part2_example() {
        let input = r"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
        1698522-1698528,446443-446449,38593856-38593862,565653-565659,
        824824821-824824827,2121212118-2121212124";
        let invalid_ids = find_invalid_ids(input, is_invalid_id_part2).unwrap();
        assert_eq!(invalid_ids.iter().sum::<usize>(), 4174379265)
    }
}
