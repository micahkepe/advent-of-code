fn normal_num_parse(contents: &str) -> anyhow::Result<Vec<Vec<usize>>> {
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
    Ok(num_grid)
}

fn part_1(contents: &str) -> anyhow::Result<Vec<usize>> {
    let num_grid = normal_num_parse(contents)?;
    let mut results = Vec::new();
    for (col, op) in
        contents.lines().last().unwrap().split_whitespace().enumerate()
    {
        match op {
            "+" => results
                .push(num_grid.iter().fold(0, |acc, row| acc + row[col])),
            "*" => results
                .push(num_grid.iter().fold(1, |acc, row| acc * row[col])),
            _ => anyhow::bail!("unsupported operation: '{}'", op),
        }
    }
    Ok(results)
}

fn right_left_columnar_parse(
    contents: &str,
) -> anyhow::Result<Vec<Vec<usize>>> {
    let rows = contents.lines().count();
    let lines: Vec<&str> = contents.lines().take(rows - 1).collect(); // skip last

    // Find max length
    let max_len = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    let mut problems: Vec<Vec<usize>> = Vec::new();
    let mut current_number_digits: Vec<Vec<char>> = Vec::new();

    // Process character columns from right to left
    for col_idx in (0..max_len).rev() {
        let mut column_chars: Vec<char> = Vec::new();

        // Collect characters at this position from all rows
        for line in &lines {
            if col_idx < line.len() {
                column_chars.push(line.chars().nth(col_idx).unwrap());
            } else {
                column_chars.push(' ');
            }
        }

        // If all spaces --> problem separator
        let is_separator = column_chars.iter().all(|&c| c == ' ');

        if is_separator {
            // End of a problem - convert accumulated digits to numbers
            if !current_number_digits.is_empty() {
                let mut numbers = Vec::new();
                for digit_group in &current_number_digits {
                    if !digit_group.is_empty() {
                        let num_str: String = digit_group.iter().collect();
                        numbers.push(num_str.parse::<usize>()?);
                    }
                }
                numbers.reverse(); // Since we built them backwards
                problems.push(numbers);
                current_number_digits.clear();
            }
        } else {
            // Part of a number - collect digits from this column
            let mut digit_group = Vec::new();
            for &ch in &column_chars {
                if ch.is_ascii_digit() {
                    digit_group.push(ch);
                }
            }
            if !digit_group.is_empty() {
                current_number_digits.push(digit_group);
            }
        }
    }

    // Last problem (leftmost)
    if !current_number_digits.is_empty() {
        let mut numbers = Vec::new();
        for digit_group in &current_number_digits {
            if !digit_group.is_empty() {
                let num_str: String = digit_group.iter().collect();
                numbers.push(num_str.parse::<usize>()?);
            }
        }
        numbers.reverse();
        problems.push(numbers);
    }

    #[cfg(test)]
    println!("parsed grid: {:?}", problems);

    Ok(problems)
}

fn part_2(contents: &str) -> anyhow::Result<Vec<usize>> {
    let num_grid = right_left_columnar_parse(contents)?;
    let mut results = Vec::new();

    let mut ops: Vec<&str> =
        contents.lines().last().unwrap().split_whitespace().collect();
    ops.reverse();

    for (col, op) in ops.iter().enumerate() {
        match *op {
            "+" => results.push(num_grid[col].iter().sum()),
            "*" => results.push(num_grid[col].iter().product()),
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

    /* Part 2 */
    let applied_aps = part_2(&contents)?;
    println!("Part 2: {}", applied_aps.iter().sum::<usize>());

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

    #[test]
    fn test_part2_example() {
        let input = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   + ";
        let ops = part_2(input).unwrap();
        assert_eq!(ops, vec![1058, 3253600, 625, 8544]);
        let sum = ops.iter().sum::<usize>();
        assert_eq!(sum, 3263827)
    }
}
