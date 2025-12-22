use anyhow::Context;

#[derive(Debug)]
struct Shape {
    /// The shape's diagram of its area.
    display: Vec<Vec<bool>>,
}

impl Shape {}

#[derive(Debug)]
struct Region {
    /// The width of the region.
    width: usize,
    /// The length of the region.
    length: usize,
    /// The quantities of each shape, where quantities[i] = amount of the ith shape
    quantities: Vec<usize>,
}

impl Region {
    /// 2D version of the bin packing problem.
    ///
    /// See: <https://en.wikipedia.org/wiki/Bin_packing_problem>
    fn can_fit_shapes(&self, shapes: &[Shape]) -> bool {
        let mut grid = vec![vec![false; self.width]; self.length];
        true
    }
}

fn parse_input_spec(
    contents: &str,
) -> anyhow::Result<(Vec<Shape>, Vec<Region>)>
where
{
    let mut lines = contents.trim().lines().peekable();
    let mut shapes: Vec<Shape> = Vec::new();
    let mut curr_idx = 0;

    // parse shapes
    while let Some(line) = lines.peek()
        && !line.contains("x")
    {
        let line = lines.next().unwrap();
        let (idx, rest) = line.split_once(":").with_context(|| {
            format!("expected shape index line, got: {}", line)
        })?;
        assert_eq!(idx.parse::<usize>()?, curr_idx);
        assert!(rest.is_empty());

        // parse diagram
        let mut display: Vec<Vec<bool>> = Vec::new();
        while let Some(row) = lines.next()
            && !row.is_empty()
        {
            display.push(
                row.chars()
                    .map(|c| {
                        Ok(match c {
                            '#' => true,
                            '.' => false,
                            _ => {
                                anyhow::bail!(
                                    "Invalid character in shape display: {}",
                                    c
                                )
                            }
                        })
                    })
                    .collect::<anyhow::Result<Vec<bool>>>()?,
            );
        }

        shapes.push(Shape { display });

        curr_idx += 1;
    }

    // parse regions
    let mut regions = Vec::new();
    for line in lines {
        if let Some((dims, quantities)) = line.split_once(':') {
            let (width, length) = dims
                .split_once('x')
                .with_context(|| format!("no 'x' in dims: {}", dims))?;
            let (width, length): (usize, usize) =
                (width.parse()?, length.parse()?);

            let quantities = quantities
                .split_whitespace()
                .map(|q| {
                    q.parse::<usize>()
                        .with_context(|| format!("invalid quantity: {}", q))
                })
                .collect::<anyhow::Result<Vec<usize>>>()?;

            assert_eq!(quantities.len(), shapes.len());
            regions.push(Region { length, width, quantities });
        } else {
            anyhow::bail!("invalid region line: {}", line)
        }
    }

    #[cfg(test)]
    {
        println!("Shapes:\n{:?}", shapes);
        println!("Regions:\n{:?}", regions);
    }

    Ok((shapes, regions))
}

fn part1(shapes: &[Shape], regions: &[Region]) -> usize {
    regions.iter().fold(0, |acc, e| acc + e.can_fit_shapes(shapes) as usize)
}

fn main() -> anyhow::Result<()> {
    let contents = std::fs::read_to_string("./data/day-12-input.txt")?;
    let (shapes, regions) = parse_input_spec(&contents)?;

    /* Part 1 */
    println!("Part 1: {}", part1(&shapes, &regions));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_example() {
        let input = "
0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";
        let (shapes, regions) = parse_input_spec(input).unwrap();
        assert_eq!(part1(&shapes, &regions), 2)
    }

    #[test]
    fn test_part2_example() {
        // TODO: fill me in
    }
}
