use std::collections::{HashMap, HashSet};

fn parse_device_adjacency_list(
    contents: &str,
) -> anyhow::Result<HashMap<&str, Vec<&str>>> {
    let contents = contents.trim();
    let mut adj = HashMap::new();
    for line in contents.lines() {
        if let Some((device, rest)) = line.split_once(':') {
            let entry = adj.entry(device).or_insert(Vec::new());
            for output in rest.split_whitespace() {
                entry.push(output.trim());
            }
        } else {
            anyhow::bail!("Invalid input line: {}", line)
        }
    }

    #[cfg(test)]
    println!("adjacency graph: {:?}", adj);

    Ok(adj)
}

fn count_paths<'a>(
    adj: &'a HashMap<&'a str, Vec<&'a str>>,
    source: &'a str,
    target: &'a str,
) -> anyhow::Result<usize> {
    if !adj.contains_key(source) {
        anyhow::bail!("unreachable: source node is not in the graph")
    }

    let mut count = 0;
    let mut visited: HashSet<&str> = HashSet::new();
    visited.insert(source);

    fn dfs<'a>(
        current: &'a str,
        target: &'a str,
        adj: &HashMap<&'a str, Vec<&'a str>>,
        visited: &mut HashSet<&'a str>,
        count: &mut usize,
    ) {
        if current == target {
            *count += 1;
            return;
        }

        if let Some(nbrs) = adj.get(current) {
            for &nbr in nbrs {
                if !visited.contains(nbr) {
                    visited.insert(nbr);
                    dfs(nbr, target, adj, visited, count);
                    visited.remove(nbr);
                }
            }
        }
    }

    dfs(source, target, adj, &mut visited, &mut count);

    Ok(count)
}

fn main() -> anyhow::Result<()> {
    let contents = std::fs::read_to_string("./data/day-11-input.txt")?;
    let adj = parse_device_adjacency_list(&contents)?;

    /* Part 1 */
    println!("Part 1: {}", count_paths(&adj, "you", "out")?);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_example() {
        let input = "
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";
        let adj = parse_device_adjacency_list(input).unwrap();
        assert_eq!(count_paths(&adj, "you", "out").unwrap(), 5);
    }

    #[test]
    fn test_part2_example() {
        // TODO: fill me in
    }
}
