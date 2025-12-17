use anyhow::Context;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    str::FromStr,
};

#[derive(Debug)]
struct Machine {
    /// Bitmask of the target light pattern
    target: u16,
    /// Button wiring schematics
    toggles: Vec<u16>,
    /// Joltage requirements
    joltages: Vec<u16>,
}

impl Machine {
    fn min_button_presses(&self) -> anyhow::Result<usize> {
        let mut visited: HashSet<u16> = HashSet::new();

        // NOTE: this is a max-heap by default, use Reverse
        let mut heap = BinaryHeap::new();
        heap.push((Reverse(0), 0u16));

        while let Some(entry) = heap.pop() {
            let (ops, curr) = entry;
            if curr == self.target {
                return Ok(ops.0);
            }
            if visited.contains(&curr) {
                continue; // were able to reach in less ops
            }
            visited.insert(curr);
            for toggle in &self.toggles {
                let curr = curr ^ toggle;
                heap.push((Reverse(ops.0 + 1), curr));
            }
        }

        println!("visited: {:?}", visited);

        anyhow::bail!("Target not reachable with given toggles:\n{}", self)
    }
}

impl FromStr for Machine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace().peekable();

        // light diagram
        let diagram = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("missing light diagram"))?;
        if !(diagram.starts_with('[') && diagram.ends_with(']')) {
            anyhow::bail!("invalid light diagram")
        }
        let lights: u16 = diagram
            .chars()
            .take(diagram.len() - 1)
            .skip(1)
            .try_fold(0u16, |acc, ch| {
                let bit = match ch {
                    '.' => 0,
                    '#' => 1,
                    other => anyhow::bail!(
                        "invalid light diagram character: '{}'",
                        other
                    ),
                };
                Ok((acc << 1) | bit)
            })?;

        // wirings
        let mut wirings = Vec::new();
        let n = diagram.len() - 2;
        while let Some(wiring) =
            parts.next_if(|w| w.starts_with('(') && w.ends_with(')'))
        {
            let mut schematic = 0u16;
            let btns: Vec<usize> = wiring[1..wiring.len() - 1]
                .split(',')
                .map(|idx| {
                    idx.parse::<usize>()
                        .with_context(|| format!("wiring idx str: {}", idx))
                })
                .collect::<anyhow::Result<Vec<usize>>>()?;
            for idx in btns {
                // Use LSB-first encoding for indexing
                let bit = n - 1 - idx;
                schematic |= 1 << bit;
            }
            wirings.push(schematic);
        }

        // joltages
        let joltages =
            parts.next().ok_or_else(|| anyhow::anyhow!("missing joltages"))?;
        if !(joltages.starts_with('{') && joltages.ends_with('}')) {
            anyhow::bail!("invalid joltages")
        }
        let joltages: Vec<u16> = joltages[1..joltages.len() - 1]
            .split(',')
            .map(|j| {
                j.parse::<u16>()
                    .with_context(|| format!("joltage digit str: {}", j))
            })
            .collect::<anyhow::Result<Vec<u16>>>()?;

        assert_eq!(joltages.len(), diagram.len() - 2);

        if parts.next().is_some() {
            anyhow::bail!("too many parts")
        }

        Ok(Machine { target: lights, toggles: wirings, joltages })
    }
}

impl std::fmt::Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let n = self.joltages.len();
        writeln!(f, "Diagram:  {:0>width$b}", self.target, width = n)?;
        writeln!(f, "Toggles:")?;
        for toggle in &self.toggles {
            writeln!(f, "\t  {:0>width$b}", toggle, width = n)?
        }
        writeln!(f, "Joltages: {:?}", self.joltages)?;
        Ok(())
    }
}

fn parse_input(contents: &str) -> anyhow::Result<Vec<Machine>> {
    contents
        .trim()
        .lines()
        .map(Machine::from_str)
        .collect::<anyhow::Result<_>>()
}

fn compute_min_button_presses(machines: &[Machine]) -> anyhow::Result<usize> {
    machines.iter().try_fold(0usize, |acc, machine| {
        Ok(acc + machine.min_button_presses()?)
    })
}

fn main() -> anyhow::Result<()> {
    let contents = std::fs::read_to_string("./data/day-10-input.txt")?;
    let machines = parse_input(&contents)?;

    /* Part 1 */
    println!("Part 1: {}", compute_min_button_presses(&machines)?);

    /* Part 2 */

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_example_machine1() {
        let input = "
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
            ";
        let machines = parse_input(input).unwrap();
        // Press last 2 buttons:
        //
        //       start: [....]
        // press (0,2): [#.#.]
        // press (0,1): [.##.] == [.##.]
        assert_eq!(compute_min_button_presses(&machines).unwrap(), 2)
    }

    #[test]
    fn test_part1_example_machine2() {
        let input = "
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
            ";
        let machines = parse_input(input).unwrap();
        // Press ((0,4), (0,1,2), and (1,2,3,4))
        //
        //     start:   [.....]
        //     (0,4):   [#...#]
        //   (0,1,2):   [.##.#]
        // (1,2,3,4):   [...#.] == [...#.]
        assert_eq!(compute_min_button_presses(&machines).unwrap(), 3)
    }

    #[test]
    fn test_part1_example_machine3() {
        let input = "
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
            ";
        let machines = parse_input(input).unwrap();
        // Press (0,3,4) and (0,1,2,4,5)
        //
        //       start: [......]
        //     (0,3,4): [#..##.]
        // (0,1,2,4,5): [.###.#] == [.###.#]
        assert_eq!(compute_min_button_presses(&machines).unwrap(), 2)
    }

    #[test]
    fn test_part1_example() {
        let input = "
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
            ";
        let machines = parse_input(input).unwrap();
        for machine in &machines {
            println!("{}", machine)
        }
        assert_eq!(compute_min_button_presses(&machines).unwrap(), 7)
    }

    #[test]
    fn test_part2_example() {
        // TODO: fill me in
    }
}
