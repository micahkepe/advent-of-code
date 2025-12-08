use anyhow::Context;

fn total_joltage(contents: &str) -> anyhow::Result<usize> {
    // > the joltage that the bank produces is equal to the number formed by the digits on the
    // > batteries you've turned on
    let mut total = 0;
    for bank in contents.lines() {
        let bank = bank.trim();
        let len = bank.len();

        /*
         * Greedy heuristic - select maximum element from bank[..len - 1], then select maximum
         * subsequent element.
         *
         * Need to select the first occurrence of the maximum element.
         */

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

/// Get max joltage by selecting k elements and parsing, respecting original order.
///
/// # Example
/// Given k = 12:
/// > - In 987654321111111, the largest joltage can be found by turning on everything
/// >   except some 1s at the end to produce 987654321111.
/// > - In the digit sequence 811111111111119, the largest joltage can be found by
/// >   turning on everything except some 1s, producing 811111111119.
/// > - In 234234234234278, the largest joltage can be found by turning on everything
/// >   except a 2 battery, a 3 battery, and another 2 battery near the start to produce
/// >   434234234278.
/// > - In 818181911112111, the joltage 888911112111 is produced by turning on everything
/// >   except some 1s near the front.
///
/// # Errors
///
/// - Given bank of length n, panics on k > n.
fn total_joltage_select_k(contents: &str, k: usize) -> anyhow::Result<usize> {
    let mut total = 0;
    for bank in contents.lines() {
        let bank = bank.trim();
        let len = bank.len();
        if k >= len {
            return Err(anyhow::anyhow!("Can not select {k} from {len} elements"));
        }

        let digits: Vec<char> = bank.chars().collect();
        let mut remaining = len - k;
        let mut stack: Vec<char> = Vec::new(); // monotonic stack
        for &digit in &digits {
            while remaining > 0 && !stack.is_empty() && stack.last().unwrap() < &digit {
                stack.pop();
                remaining -= 1;
            }
            stack.push(digit);
        }
        while remaining > 0 {
            stack.pop();
            remaining -= 1;
        }

        let joltage: usize = stack
            .iter()
            .collect::<String>()
            .parse()
            .with_context(|| format!("invalid stack: {:?}", stack))?;
        total += joltage

        /* Old attempt: too complicated and not fully correct */
        // bitmap for tracking which elements have been used
        //let digits: Vec<u32> = bank.chars().map(|ch| ch.to_digit(10).unwrap()).collect();
        //let mut not_used = vec![false; len];
        //let mut curr_min = u32::MAX;
        //let mut curr_min_idx: isize = -1;
        //let mut remaining = len - k;
        //while remaining > 0 {
        //    // Greedy heuristic -> find minimum element not yet eliminated from right to left traversal
        //    // - In the case of multiple minimum elements, take the left-most
        //    // - On last element to remove, remove first element less than next element.
        //    //
        //    // Case: 234234234234278
        //    //
        //    // [1]: 34234234234278
        //    // [2]:  3434234234278
        //    // [3]:   434234234278
        //    //      --------------
        //    //        434234234278
        //    if remaining == 1 {
        //        let active_pos: Vec<usize> = (0..len).filter(|&idx| !not_used[idx]).collect();
        //        let mut found = false;
        //        for i in 0..active_pos.len() - 1 {
        //            let pos = active_pos[i];
        //            let nex_pos = active_pos[i + 1];
        //            if digits[pos] < digits[nex_pos] {
        //                curr_min_idx = pos as isize;
        //                found = true;
        //                break;
        //            }
        //        }
        //        if !found {
        //            curr_min_idx = *active_pos.last().unwrap() as isize;
        //        }
        //    } else {
        //        for (i, d) in digits.iter().enumerate().rev() {
        //            if not_used[i] {
        //                continue;
        //            }
        //            if *d <= curr_min {
        //                curr_min_idx = i as isize;
        //                curr_min = *d;
        //            }
        //        }
        //    }
        //    not_used[curr_min_idx as usize] = true;
        //    curr_min = u32::MAX;
        //    remaining -= 1;
        //}
        //// collect and parse remaining digits
        //let joltage: usize = bank
        //    .chars()
        //    .enumerate()
        //    .filter(|(i, _)| !not_used[*i])
        //    .map(|(_, c)| c)
        //    .collect::<String>()
        //    .parse()?;
        //
        ////println!("bank: {}, joltage: {}", bank, joltage);
        //total += joltage
    }
    Ok(total)
}

fn main() -> anyhow::Result<()> {
    let contents = std::fs::read_to_string("./data/day-03-input.txt")?;

    // Part 1
    println!("Part 1: {}", total_joltage(&contents)?);

    // Part 2
    println!("Part 2: {}", total_joltage_select_k(&contents, 12)?);

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

    #[test]
    fn test_part2_example() {
        let input = "987654321111111
        811111111111119
        234234234234278
        818181911112111";
        assert_eq!(total_joltage_select_k(input, 12).unwrap(), 3121910778619)
    }
}
