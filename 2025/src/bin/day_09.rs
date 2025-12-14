use std::iter::zip;

const DIRS: [(isize, isize); 8] =
    [(-1, 1), (0, 1), (1, 1), (-1, 0), (1, 0), (-1, -1), (0, -1), (1, -1)];

fn parse_coords(contents: &str) -> anyhow::Result<Vec<(usize, usize)>> {
    contents
        .trim()
        .lines()
        .enumerate()
        .map(|(idx, l)| {
            Ok(if let Some((x, y)) = l.split_once(",") {
                (y.parse::<usize>()?, x.parse::<usize>()?)
            } else {
                anyhow::bail!("Line {} is invalid: {}", idx + 1, l)
            })
        })
        .collect()
}

/// Maximum area found by constructing a rectangle with at least two corners that are red tiles.
fn max_area(red_coords: &[(usize, usize)]) -> usize {
    let mut res = 0;
    for i in 0..red_coords.len() {
        for j in i..red_coords.len() {
            let (a, b) = (red_coords[i], red_coords[j]);
            let curr_area = (a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1);
            res = res.max(curr_area);
        }
    }
    res
}

/// Fill in all loops with green (mark as occupied)
/// https://en.wikipedia.org/wiki/Flood_fill
fn flood_fill(grid: &mut [Vec<bool>]) {
    let (rows, cols) = (grid.len(), grid[0].len());

    // outside[r][c] -> cell reachable from the outside
    let mut outside = vec![vec![false; cols]; rows];
    let mut stack = Vec::new();

    for r in 0..rows {
        if !grid[r][0] {
            stack.push((r, 0));
            outside[r][0] = true;
        }
        if !grid[r][cols - 1] {
            stack.push((r, cols - 1));
            outside[r][cols - 1] = true;
        }
    }
    for c in 0..cols {
        if !grid[0][c] {
            stack.push((0, c));
            outside[0][c] = true;
        }
        if !grid[rows - 1][c] {
            stack.push((rows - 1, c));
            outside[rows - 1][c] = true;
        }
    }
    while let Some((r, c)) = stack.pop() {
        for (dr, dc) in DIRS {
            let (nr, nc) = (r as isize + dr, c as isize + dc);
            if nr >= 0 && nr < rows as isize && nc >= 0 && nc < cols as isize {
                let (nr, nc) = (nr as usize, nc as usize);
                if !grid[nr][nc] && !outside[nr][nc] {
                    outside[nr][nc] = true;
                    stack.push((nr, nc));
                }
            }
        }
    }

    for r in 0..rows {
        for c in 0..cols {
            if !outside[r][c] {
                grid[r][c] = true
            }
        }
    }
}

/// Maximum area found by constructing a rectangle with at least two corners that are red tiles
fn max_area_red_green(red_coords: &[(usize, usize)]) -> anyhow::Result<usize> {
    let (rows, cols) = (
        red_coords.iter().map(|(x, _)| x).max().expect("no max row value") + 1,
        red_coords.iter().map(|(_, y)| y).max().expect("no max col value") + 1,
    );

    // grid[r][c] -> whether (r, c) is occupied by red or green tile
    let mut grid = vec![vec![false; cols]; rows];
    for (r, c) in red_coords {
        grid[*r][*c] = true;
    }

    // connect all coordinates with straight lines of green tiles
    let mut second_iter: Vec<&(usize, usize)> =
        red_coords.iter().skip(1).collect();
    second_iter.push(&red_coords[0]);

    for (a, b) in zip(red_coords, second_iter) {
        if a.0 == b.0 {
            let row = a.0;
            let (start_col, end_col) = (a.1.min(b.1), b.1.max(a.1));
            (start_col..=end_col).for_each(|col| grid[row][col] = true);
        } else if a.1 == b.1 {
            let col = a.1;
            let (start_row, end_row) = (a.0.min(b.0), b.0.max(a.0));
            (start_row..=end_row).for_each(|row| grid[row][col] = true);
        } else {
            anyhow::bail!(
                "cannot connect points in straight line: {:?} {:?}",
                a,
                b
            )
        }
    }

    #[cfg(test)]
    {
        println!("Flood filled grid:");
        (0..rows).for_each(|r| {
            let mut row = String::new();
            (0..cols).for_each(|c| {
                if red_coords.contains(&(r, c)) {
                    row.push('#');
                } else if grid[r][c] {
                    row.push('X');
                } else {
                    row.push('.');
                }
            });
            println!("{}", row)
        });
    }

    // flood fill
    flood_fill(&mut grid);

    #[cfg(test)]
    {
        println!("Flood filled grid:");
        (0..rows).for_each(|r| {
            let mut row = String::new();
            (0..cols).for_each(|c| {
                if red_coords.contains(&(r, c)) {
                    row.push('#');
                } else if grid[r][c] {
                    row.push('X');
                } else {
                    row.push('.');
                }
            });
            println!("{}", row)
        });
    }

    // Naive first pass
    // go through the squares with two red corners and count number of tiles
    let mut res = 0;
    for (i, a) in red_coords.iter().enumerate() {
        for b in red_coords.iter().skip(i) {
            let mut count = 0;
            let (start_row, end_row) = (a.0.min(b.0), b.0.max(a.0));
            let (start_col, end_col) = (a.1.min(b.1), b.1.max(a.1));
            let area = (end_row - start_row + 1) * (end_col - start_col + 1);
            // Skip if this rectangle can't possibly beat our current best
            if area <= res {
                continue;
            }
            let mut is_fully_filled = true;

            #[allow(clippy::needless_range_loop)]
            'outer: for r in start_row..=end_row {
                for c in start_col..=end_col {
                    if grid[r][c] {
                        count += 1
                    } else {
                        is_fully_filled = false;
                        break 'outer;
                    }
                }
            }
            if is_fully_filled {
                res = res.max(count)
            }
        }
    }

    Ok(res)
}

fn main() -> anyhow::Result<()> {
    let contents = std::fs::read_to_string("./data/day-09-input.txt")?;
    let contents = contents.trim();
    let coords = parse_coords(contents)?;

    /* Part 1 */
    println!("Part 1: {}", max_area(&coords));

    /* Part 2 */
    println!("Part 2: {}", max_area_red_green(&coords)?);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_example() {
        let input = "
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3            ";
        let coords = parse_coords(input).unwrap();
        assert_eq!(max_area(&coords), 50)
    }

    #[test]
    fn test_part2_example() {
        let input = "
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3            ";
        let coords = parse_coords(input).unwrap();
        assert_eq!(max_area_red_green(&coords).unwrap(), 24)
    }
}
