use std::collections::HashSet;

fn compute_total_beam_splits(contents: &str) -> anyhow::Result<usize> {
    let contents = contents.trim();
    let mut curr_beam_indices: HashSet<usize> = HashSet::new();
    curr_beam_indices.insert(
        contents
            .lines()
            .next()
            .unwrap()
            .find('S')
            .expect("no 'S' on first line"),
    );

    let mut total_splits = 0;
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
        }

        // update active indices
        curr_beam_indices.extend(new_active);

        #[cfg(test)]
        println!("beams: {:?}", curr_beam_indices)
    }

    Ok(total_splits)
}

fn main() -> anyhow::Result<()> {
    let contents = std::fs::read_to_string("./data/day-07-input.txt")?;

    /* Part 1 */
    let total_beam_splits = compute_total_beam_splits(&contents)?;
    println!("Part 1: {}", total_beam_splits);

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
        assert_eq!(compute_total_beam_splits(input).unwrap(), 21);
    }
}
