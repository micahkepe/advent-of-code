use std::str::FromStr;

/// Position with XYZ coordinates.
#[derive(Debug, Clone, Copy)]
struct Position {
    /// X position
    x: usize,
    /// Y position
    y: usize,
    /// Z position
    z: usize,
}

impl Position {
    fn new(x: usize, y: usize, z: usize) -> Self {
        Self { x, y, z }
    }

    /// Straight line distance to another point
    ///
    /// See: https://en.wikipedia.org/wiki/Euclidean_distance
    fn euclidean_distance(self, other: &Position) -> f64 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)) as f64)
            .sqrt()
    }
}

#[derive(Debug, Clone, Copy)]
struct Junction {
    position: Position,
}

impl Junction {
    fn new(position: Position) -> Self {
        Self { position }
    }
}

impl FromStr for Junction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(',').map(str::parse::<usize>);
        let x = iter.next().ok_or_else(|| anyhow::anyhow!("missing x"))??;
        let y = iter.next().ok_or_else(|| anyhow::anyhow!("missing y"))??;
        let z = iter.next().ok_or_else(|| anyhow::anyhow!("missing z"))??;
        if iter.next().is_some() {
            anyhow::bail!("too many coords");
        }
        let position = Position::new(x, y, z);
        Ok(Junction::new(position))
    }
}

/// Graph
///
/// Graphs in Rust: <https://smallcultfollowing.com/babysteps/blog/2015/04/06/modeling-graphs-in-rust-using-vector-indices/>
#[derive(Debug)]
struct Graph {}

impl Graph {}

fn construct_connected_graph(_junctions: &[Junction]) -> Graph {
    Graph {}
}

fn part1(contents: &str) -> anyhow::Result<usize> {
    let junctions: Vec<Junction> = contents
        .trim()
        .lines()
        .map(str::parse)
        .collect::<anyhow::Result<_>>()?;

    //#[cfg(test)]
    //println!("{:?}", junctions);

    // TODO: construct graph

    // TODO: find components and their sizes

    // TODO: sum sizes of top 3 largest components
    Ok(0)
}

fn main() -> anyhow::Result<()> {
    let contents = std::fs::read_to_string("./data/day-08-input.txt")?;

    /* Part 1 */
    let component_size_sum = part1(&contents)?;
    println!("Part 1: {}", component_size_sum);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_example() {
        let input = "
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
        assert_eq!(part1(input).unwrap(), 40)
    }
}
