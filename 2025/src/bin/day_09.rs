fn parse_coords(contents: &str) -> anyhow::Result<Vec<(usize, usize)>> {
    contents
        .trim()
        .lines()
        .enumerate()
        .map(|(idx, l)| {
            Ok(if let Some((x, y)) = l.split_once(",") {
                (x.parse::<usize>()?, y.parse::<usize>()?)
            } else {
                anyhow::bail!("Line {} is invalid: {}", idx + 1, l)
            })
        })
        .collect()
}

fn max_area(coords: &[(usize, usize)]) -> usize {
    let mut res = 0;
    for i in 0..coords.len() {
        for j in i..coords.len() {
            let (a, b) = (coords[i], coords[j]);
            let curr_area = (a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1);
            res = res.max(curr_area);
        }
    }
    res
}

fn main() -> anyhow::Result<()> {
    let contents = std::fs::read_to_string("./data/day-09-input.txt")?;

    /* Part 1 */
    let coords = parse_coords(&contents)?;
    println!("Part 1: {}", max_area(&coords));

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
    fn test_part2_example() {}
}
