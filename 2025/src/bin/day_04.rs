const DIRS: [(i32, i32); 8] = [
    (-1, 1),
    (0, 1),
    (1, 1),
    (-1, 0),
    (1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];

fn count_forklift_accessible(contents: &str) -> anyhow::Result<usize> {
    let rows: i32 = contents.trim().lines().count() as i32;
    let cols: i32 = contents.lines().next().unwrap().trim().chars().count() as i32;
    let mut occupied: Vec<Vec<bool>> = vec![vec![false; cols as usize]; rows as usize];
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

    let mut count = 0;
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
                count += 1;
            }
        }
    }

    Ok(count)
}
fn main() -> anyhow::Result<()> {
    let contents = std::fs::read_to_string("./data/day-04-input.txt")?;

    /* Part 1 */
    let count = count_forklift_accessible(&contents)?;
    println!("Part 1: {}", count);

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
        assert_eq!(count_forklift_accessible(input).unwrap(), 13);
    }
}
