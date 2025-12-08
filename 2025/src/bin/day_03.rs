fn total_joltage(contents: &str) -> anyhow::Result<usize> {
    // > the joltage that the bank produces is equal to the number formed by the digits on the
    // > batteries you've turned on
    let mut total = 0;
    for bank in contents.lines() {
        let bank = bank.trim();
        let len = bank.len();

        // first pass to find max battery value (excluding the last battery)
        let first_val = bank[..len - 1]
            .chars()
            .map(|ch| ch.to_digit(10).unwrap())
            .max()
            .unwrap();
        let first_idx = bank[..len - 1]
            .chars()
            .position(|ch| ch.to_digit(10).unwrap() == first_val)
            .unwrap();

        // find max battery after the first value
        let second_val = bank[first_idx + 1..]
            .chars()
            .map(|ch| ch.to_digit(10).unwrap())
            .max()
            .unwrap();

        let val = (first_val * 10) + second_val;

        total += val;
    }

    Ok(total as usize)
}

fn main() -> anyhow::Result<()> {
    let contents = std::fs::read_to_string("./data/day-03-input.txt")?;

    // Part 1
    println!("Part 1: {}", total_joltage(&contents)?);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_example() {
        let input = "987654321111111
        811111111111119
        234234234234278
        818181911112111";
        assert_eq!(total_joltage(input).unwrap(), 357)
    }
}
