use std::collections::HashSet;

fn compute_total_beam_splits(contents: &str) -> anyhow::Result<(usize, usize)> {
    let contents = contents.trim();
    let mut curr_beam_indices: HashSet<usize> = HashSet::new();
    let starting_idx = contents
        .lines()
        .next()
        .unwrap()
        .find('S')
        .expect("no 'S' on first line");

    curr_beam_indices.insert(starting_idx);

    let mut total_splits = 0;

    // beam_counts[pos] -> number of timelines with beam at that position
    let cols = contents.lines().next().unwrap().chars().count();
    let mut beam_counts = vec![0; cols];
    beam_counts[starting_idx] = 1;

    for line in contents.lines().skip(1) {
        let splitter_indices: HashSet<usize> =
            line.match_indices("^").map(|(idx, _)| idx).collect();

        if splitter_indices.is_empty() {
            // no splitters on this line
            continue;
        }

        //#[cfg(test)]
        //println!(
        //    "found splitters at: {:?}; total splits: {}",
        //    splitter_indices, total_splits
        //);

        // find intersecting splitters
        let intersecting_splitters: HashSet<usize> = curr_beam_indices
            .intersection(&splitter_indices)
            .copied()
            .collect();

        for &idx in &intersecting_splitters {
            curr_beam_indices.remove(&idx);
        }

        let mut new_active: HashSet<usize> = HashSet::new();
        for &splitter_idx in &intersecting_splitters {
            total_splits += 1;
            new_active.insert(splitter_idx - 1);
            new_active.insert(splitter_idx + 1);

            let count = beam_counts[splitter_idx];
            beam_counts[splitter_idx] = 0;
            beam_counts[splitter_idx - 1] += count;
            beam_counts[splitter_idx + 1] += count;
        }

        // update active indices
        curr_beam_indices.extend(new_active);

        #[cfg(test)]
        println!("beams: {:?}", curr_beam_indices)
    }

    let total_combinations = beam_counts.iter().sum();

    Ok((total_splits, total_combinations))
}

fn main() -> anyhow::Result<()> {
    let contents = std::fs::read_to_string("./data/day-07-input.txt")?;

    /* Part 1 */
    let (total_beam_splits, total_beam_combos) = compute_total_beam_splits(&contents)?;
    println!("Part 1: {}", total_beam_splits);

    /* Part 2 */
    println!("Part 2: {}", total_beam_combos);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_example() {
        let input = "
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
            ";
        assert_eq!(compute_total_beam_splits(input).unwrap().0, 21);
    }

    #[test]
    fn test_part2_example() {
        let input = "
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
            ";
        assert_eq!(compute_total_beam_splits(input).unwrap().1, 40);
    }
}
