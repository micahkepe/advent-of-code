use std::str::FromStr;

use anyhow::Context;

#[derive(Debug)]
struct Machine {
    /// Bitmask of lights currently on
    lights: u16,
    /// Button wiring schematics
    wirings: Vec<u16>,
    /// Joltage requirements
    joltages: Vec<u16>,
}

impl Machine {
    fn min_button_presses(&self) -> anyhow::Result<usize> {
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
                let mask = 1 << idx;
                schematic |= mask;
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

        if parts.next().is_some() {
            anyhow::bail!("too many parts")
        }

        Ok(Machine { lights, wirings, joltages })
    }
}

fn parse_input(contents: &str) -> anyhow::Result<Vec<Machine>> {
    let machines = contents
        .trim()
        .lines()
        .map(Machine::from_str)
        .collect::<anyhow::Result<_>>();

    #[cfg(test)]
    println!("{:?}", machines);

    machines
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
    fn test_part1_example() {
        let input = "
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
            ";
        let machines = parse_input(input).unwrap();
    }

    #[test]
    fn test_part2_example() {
        // TODO: fill me in
    }
}
