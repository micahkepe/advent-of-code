const DIRS: [(i32, i32); 8] =
    [(-1, 1), (0, 1), (1, 1), (-1, 0), (1, 0), (-1, -1), (0, -1), (1, -1)];

fn construct_occupied(contents: &str) -> anyhow::Result<Vec<Vec<bool>>> {
    let rows: i32 = contents.trim().lines().count() as i32;
    let cols: i32 =
        contents.lines().next().unwrap().trim().chars().count() as i32;
    let mut occupied: Vec<Vec<bool>> =
        vec![vec![false; cols as usize]; rows as usize];
    for (i, line) in contents.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            match c {
                '@' => occupied[i][j] = true,
                '.' => {}
                _ => anyhow::bail!(
                    "line {}: found unexpected character '{}' at column {}",
                    i + 1,
                    c,
                    j + 1
                ),
            }
        }
    }

    Ok(occupied)
}

fn find_forklift_accessible(
    occupied: &[Vec<bool>],
) -> anyhow::Result<Vec<(usize, usize)>> {
    let (rows, cols) = (occupied.len() as i32, occupied[0].len() as i32);
    let mut indices: Vec<(usize, usize)> = Vec::new();
    for r in 0..rows {
        for c in 0..cols {
            if !occupied[r as usize][c as usize] {
                continue;
            }
            let mut surrounding_count = 0;
            for (dr, dc) in DIRS {
                let (nr, nc): (i32, i32) = (r + dr, c + dc);
                if nr < 0 || nr >= rows || nc < 0 || nc >= cols {
                    continue;
                }
                if occupied[nr as usize][nc as usize] {
                    surrounding_count += 1;
                }
            }
            if surrounding_count < 4 {
                indices.push((r as usize, c as usize))
            }
        }
    }

    Ok(indices)
}

fn find_accessible_iterative(
    mut occupied: Vec<Vec<bool>>,
) -> anyhow::Result<usize> {
    let indices = find_forklift_accessible(&occupied)?;
    let mut last_found = indices.len();
    for (r, c) in indices.iter() {
        occupied[*r][*c] = false
    }
    let mut total_count = last_found;

    // iterative fixpoint
    while last_found != 0 {
        let indices = find_forklift_accessible(&occupied)?;
        for (r, c) in indices.iter() {
            occupied[*r][*c] = false
        }
        last_found = indices.len();
        total_count += last_found
    }

    Ok(total_count)
}

fn main() -> anyhow::Result<()> {
    let contents = std::fs::read_to_string("./data/day-04-input.txt")?;

    /* Part 1 */
    let occupied = construct_occupied(&contents)?;
    let indices = find_forklift_accessible(&occupied)?;
    println!("Part 1: {}", indices.len());

    /* Part 2 */
    let total_iterative = find_accessible_iterative(occupied)?;
    println!("Part 2: {}", total_iterative);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_example() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        let occupied = construct_occupied(input).unwrap();
        assert_eq!(find_forklift_accessible(&occupied).unwrap().len(), 13);
    }

    #[test]
    fn test_part2_example() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        let occupied = construct_occupied(input).unwrap();
        assert_eq!(find_accessible_iterative(occupied).unwrap(), 43);
    }
}
