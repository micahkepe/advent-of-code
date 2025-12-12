fn part_1(contents: &str) -> anyhow::Result<Vec<usize>> {
    let rows = contents.lines().count();
    let num_grid: Vec<Vec<usize>> = contents
        .lines()
        .take(rows - 1) // skip last row
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    let mut results = Vec::new();
    for (col, op) in contents
        .lines()
        .last()
        .unwrap()
        .split_whitespace()
        .enumerate()
    {
        match op {
            "+" => results.push(num_grid.iter().fold(0, |acc, row| acc + row[col])),
            "*" => results.push(num_grid.iter().fold(1, |acc, row| acc * row[col])),
            _ => anyhow::bail!("unsupported operation: '{}'", op),
        }
    }

    Ok(results)
}

fn main() -> anyhow::Result<()> {
    let contents = std::fs::read_to_string("./data/day-06-input.txt")?;

    /* Part 1 */
    let applied_aps = part_1(&contents)?;
    println!("Part 1: {}", applied_aps.iter().sum::<usize>());

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_example() {
        let input = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   + ";
        let ops = part_1(input).unwrap();
        assert_eq!(ops, vec![33210, 490, 4243455, 401]);
        let sum = ops.iter().sum::<usize>();
        assert_eq!(sum, 4277556)
    }
}
