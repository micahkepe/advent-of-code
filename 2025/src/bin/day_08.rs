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
        let dx = self.x as f64 - other.x as f64;
        let dy = self.y as f64 - other.y as f64;
        let dz = self.z as f64 - other.z as f64;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

#[derive(Debug)]
struct Edge {
    source: usize,
    target: usize,
    dist: f64,
}

/// Naive O(n ^ 2) build edges, could make this more efficient but ¯\_(ツ)_/¯
///
/// Optimization with k-d trees: https://en.wikipedia.org/wiki/K-d_tree
fn build_edges(junctions: &[Junction]) -> Vec<Edge> {
    let mut edges = Vec::new();
    for i in 0..junctions.len() {
        for j in (i + 1)..junctions.len() {
            let dist = junctions[i]
                .position
                .euclidean_distance(&junctions[j].position);
            edges.push(Edge { source: i, target: j, dist });
        }
    }
    // sort by distance in ascending order
    edges.sort_by(|e1, e2| e1.dist.partial_cmp(&e2.dist).unwrap());
    edges
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

#[derive(Debug)]
/// Disjoint set union over the junction nodes.
///
/// Invariant: only roots represent components
struct DisjointSetUnion {
    /// parent[i] -> idx of parent of node i
    parent: Vec<usize>,
    /// size[i] -> size of the node i's component
    size: Vec<usize>,
}

impl DisjointSetUnion {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(), // initially disjoint union of all n nodes
            size: vec![1; n],         // all nodes initially disjoint
        }
    }

    /// Find the component that contains node `x`.
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            // PERF: flatten trees for amortized log finds
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    /// Connect the components containing nodes `a` and `b`.
    fn union(&mut self, a: usize, b: usize) -> bool {
        let mut root_a = self.find(a);
        let mut root_b = self.find(b);
        // check if already connected
        if root_a == root_b {
            return false;
        }

        // Invariant: root_a is root of larger tree
        // NOTE: this only affects local variables, not the DSU arrays
        if self.size[root_a] < self.size[root_b] {
            std::mem::swap(&mut root_a, &mut root_b);
        }

        // PERF: attach smaller tree under the larger one
        self.parent[root_b] = root_a;
        self.size[root_a] += self.size[root_b];
        true
    }
}

fn part1(contents: &str, target_connections: usize) -> anyhow::Result<usize> {
    let junctions: Vec<Junction> = contents
        .trim()
        .lines()
        .map(str::parse)
        .collect::<anyhow::Result<_>>()?;

    let edges = build_edges(&junctions);
    let mut dsu = DisjointSetUnion::new(junctions.len());
    let mut edges_used = 0;
    for edge in edges {
        edges_used += 1;
        if dsu.union(edge.source, edge.target)
            && edges_used == target_connections
        {
            break;
        }
    }

    // component_size[i] -> size of component
    let mut component_size = vec![0; junctions.len()];
    (0..junctions.len()).for_each(|i| {
        let root = dsu.find(i);
        component_size[root] += 1;
    });
    component_size.sort_by(|a, b| b.cmp(a)); // sort descending

    Ok(component_size[0] * component_size[1] * component_size[2])
}

/// Connect all the closest unconnected pairs of junction boxes together until they're all in one
/// circuit, then return the product of the X coordinates of the last two junction boxes needs to
/// connect.
fn part2(contents: &str) -> anyhow::Result<usize> {
    let junctions: Vec<Junction> = contents
        .trim()
        .lines()
        .map(str::parse)
        .collect::<anyhow::Result<_>>()?;

    let edges = build_edges(&junctions);
    let mut dsu = DisjointSetUnion::new(junctions.len());
    let mut connections = 0;
    for edge in edges {
        if dsu.union(edge.source, edge.target) {
            connections += 1;
            // continue until all are connected
            if connections == junctions.len() - 1 {
                let a = junctions[edge.source].position.x;
                let b = junctions[edge.target].position.x;
                return Ok(a * b);
            }
        }
    }
    unreachable!("graph should be connected")
}

fn main() -> anyhow::Result<()> {
    let contents = std::fs::read_to_string("./data/day-08-input.txt")?;

    /* Part 1 */
    let component_size_sum = part1(&contents, 1000)?;
    println!("Part 1: {}", component_size_sum);

    /* Part 2 */
    let component_size_sum = part2(&contents)?;
    println!("Part 2: {}", component_size_sum);

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
        assert_eq!(part1(input, 10).unwrap(), 40)
    }

    #[test]
    fn test_part2_example() {
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
        assert_eq!(part2(input).unwrap(), 25272)
    }
}
