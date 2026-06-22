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
