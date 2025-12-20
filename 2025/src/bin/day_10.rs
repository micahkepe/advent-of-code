// See: https://www.reddit.com/r/adventofcode/comments/1pp98cr/2025_day_10_part_2_solution_without_using_a_3rd/
use anyhow::Context;
use std::{
    collections::{HashSet, VecDeque},
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
    fn min_button_presses_match_target(&self) -> anyhow::Result<usize> {
        let mut visited: HashSet<u16> = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((0usize, 0u16));

        while let Some(entry) = queue.pop_front() {
            let (ops, curr) = entry;
            if curr == self.target {
                return Ok(ops);
            }
            if visited.contains(&curr) {
                continue; // were able to reach in less ops
            }
            visited.insert(curr);
            for toggle in &self.toggles {
                let curr = curr ^ toggle;
                queue.push_back((ops + 1, curr));
            }
        }

        anyhow::bail!("Target not reachable with given toggles:\n{}", self)
    }

    /// First attempt: too slow on large input
    #[allow(dead_code)]
    fn min_button_presses_match_joltages_naive_bfs(
        &self,
    ) -> anyhow::Result<usize> {
        let n = self.joltages.len();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        queue.push_back((0usize, vec![0u16; n]));
        while let Some((presses, current)) = queue.pop_front() {
            if current == self.joltages {
                return Ok(presses);
            }

            if visited.contains(&current) {
                continue;
            }
            visited.insert(current.clone());

            for &toggle in &self.toggles {
                let mut next = current.clone();
                (0..n).for_each(|i| {
                    let bit_pos = n - 1 - i;
                    if ((toggle >> bit_pos) & 1) == 1 {
                        next[i] += 1
                    }
                });

                // early stop if any joltage exceeds specified
                if next
                    .iter()
                    .zip(&self.joltages)
                    .all(|(curr, target)| curr <= target)
                {
                    queue.push_back((presses + 1, next));
                }
            }
        }
        anyhow::bail!("Target not reachable with given toggles:\n{}", self)
    }

    #[allow(dead_code)]
    fn min_button_presses_match_joltages_packed_u64(
        &self,
    ) -> anyhow::Result<usize> {
        let n = self.joltages.len();
        let max_joltage = *self.joltages.iter().max().expect("no joltages");

        // use extra bit to prevent overflow in search space
        let bits_per_value =
            ((16 - max_joltage.leading_zeros()) as usize).max(5);

        if n * bits_per_value > 64 {
            anyhow::bail!("Can't pack joltages into u64: {:?}", self.joltages);
        }

        let target_limits: Vec<u64> =
            self.joltages.iter().map(|&j| j as u64).collect();

        #[cfg(test)]
        {
            eprintln!(
                "n={}, max_joltage={}, bits_per_value={}",
                n, max_joltage, bits_per_value
            );
            eprintln!("Target joltages: {:?}", self.joltages);
        }

        let value_mask = (1u64 << bits_per_value) - 1;

        let target_state =
            self.joltages.iter().enumerate().fold(0u64, |acc, (i, &j)| {
                acc | (j as u64) << (i * bits_per_value)
            });

        let toggle_deltas: Vec<u64> = self
            .toggles
            .iter()
            .map(|&toggle| {
                let mut delta = 0u64;
                (0..n).for_each(|i| {
                    let bit_pos = n - 1 - i;
                    if ((toggle >> bit_pos) & 1) == 1 {
                        delta |= 1u64 << (i * bits_per_value);
                    }
                });
                delta
            })
            .collect();

        let add_packed = |state: u64, delta: u64| -> Option<u64> {
            let mut res = 0u64;
            for (i, limit) in target_limits.iter().enumerate().take(n) {
                let shift = i * bits_per_value;
                let a = (state >> shift) & value_mask;
                let b = (delta >> shift) & value_mask;
                let sum = a + b;
                if sum > *limit {
                    return None;
                }
                res |= sum << shift;
            }
            Some(res)
        };

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new(); // (ops, state) entries
        queue.push_back((0usize, 0u64));
        while let Some((presses, state)) = queue.pop_front() {
            if state == target_state {
                return Ok(presses);
            }
            if !visited.insert(state) {
                continue;
            }
            for &delta in &toggle_deltas {
                if let Some(next) = add_packed(state, delta) {
                    queue.push_back((presses + 1, next));
                }
            }
        }

        anyhow::bail!("Target not reachable with given toggles:\n{}", self)
    }

    /// Uses [Gaussian elimination] to efficiently find the minimum
    /// toggle presses required.
    ///
    /// Givens:
    /// - m buttons, n lights
    ///
    /// [Gaussian elimination]: <https://en.wikipedia.org/wiki/Gaussian_elimination>
    fn min_button_presses_match_joltages(&self) -> anyhow::Result<usize> {
        Ok(0)
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

/// Compute the minimum number of presses of the toggles to achieve the machine's lighting diagram.
/// Initially, all lights are off.
///
///  # Errors
///
/// If the end state is not possible.
fn compute_min_button_presses(machines: &[Machine]) -> anyhow::Result<usize> {
    machines.iter().try_fold(0usize, |acc, machine| {
        Ok(acc + machine.min_button_presses_match_target()?)
    })
}

/// Sum the number of minimum presses required for each machine to achieve the exact its exact
/// joltages, where each toggle represents the lights whose joltages will be incremented by 1.
/// Initially, all lights are off.
///
/// # Errors
///
/// If the end state is not possible.
fn compute_min_presses_to_match_joltages(
    machines: &[Machine],
) -> anyhow::Result<usize> {
    machines.iter().try_fold(0usize, |acc, machine| {
        Ok(acc + machine.min_button_presses_match_joltages()?)
    })
}

fn main() -> anyhow::Result<()> {
    let contents = std::fs::read_to_string("./data/day-10-input.txt")?;
    let machines = parse_input(&contents)?;

    /* Part 1 */
    println!("Part 1: {}", compute_min_button_presses(&machines)?);

    /* Part 2 */
    println!("Part 2: {}", compute_min_presses_to_match_joltages(&machines)?);

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
        assert_eq!(compute_min_button_presses(&machines).unwrap(), 7)
    }

    #[test]
    fn test_part2_example_machine1() {
        let input = "
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
            ";
        let machines = parse_input(input).unwrap();
        assert_eq!(
            compute_min_presses_to_match_joltages(&machines).unwrap(),
            10
        )
    }

    #[test]
    fn test_part2_example_machine2() {
        let input = "
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
            ";
        let machines = parse_input(input).unwrap();
        assert_eq!(
            compute_min_presses_to_match_joltages(&machines).unwrap(),
            12
        )
    }

    #[test]
    fn test_part2_example_machine3() {
        let input = "
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
            ";
        let machines = parse_input(input).unwrap();
        assert_eq!(
            compute_min_presses_to_match_joltages(&machines).unwrap(),
            11
        )
    }

    #[test]
    fn test_part2_example() {
        let input = "
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
            ";
        let machines = parse_input(input).unwrap();
        assert_eq!(
            compute_min_presses_to_match_joltages(&machines).unwrap(),
            33
        )
    }
}
