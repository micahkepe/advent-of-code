/*
 * Input format:
 * - fresh ingredient ID ranges (inclusive)
 * - blank line
 * - available ingredient IDs
 *
 * Example:
 *
 * ```txt
 * 3-5
 * 10-14
 * 16-20
 * 12-18
 *
 * 1
 * 5
 * 8
 * 11
 * 17
 * 32
 * ```
 */

use std::{
    cmp::{max, min},
    collections::HashSet,
    ops::RangeInclusive,
};

#[derive(Debug)]
struct ParsedContent {
    ranges: Vec<RangeInclusive<usize>>,
    ids: HashSet<usize>,
}

fn parse_contents(contents: &str) -> anyhow::Result<ParsedContent> {
    let contents = contents.trim();
    let mut lines_iter = contents.lines();
    // process ranges
    let mut ranges = Vec::new();
    while let Some(line) = lines_iter.next()
        && !line.is_empty()
    {
        if let Some((start, end)) = line.split_once('-') {
            let range = RangeInclusive::new(start.parse::<usize>()?, end.parse::<usize>()?);
            ranges.push(range)
        } else {
            anyhow::bail!("invalid range line: '{}'", line)
        }
    }

    // process IDs
    let ids: HashSet<usize> = lines_iter
        .map(|id_str| id_str.trim().parse().unwrap())
        .collect();
    let parsed = ParsedContent { ranges, ids };

    #[cfg(test)]
    println!("{:#?}", parsed);

    Ok(parsed)
}

fn merge_intervals(ranges: &[RangeInclusive<usize>]) -> anyhow::Result<Vec<RangeInclusive<usize>>> {
    if ranges.is_empty() {
        return Ok(Vec::new());
    }

    let mut sorted_ranges: Vec<RangeInclusive<usize>> = ranges.to_vec();
    sorted_ranges.sort_by(|a, b| a.start().cmp(b.start()));
    let mut merged_ranges: Vec<RangeInclusive<usize>> = Vec::new();

    merged_ranges.push(sorted_ranges[0].clone());
    let mut last_end = *sorted_ranges[0].end();

    for range in sorted_ranges.iter().skip(1) {
        if *range.start() > last_end {
            // no merge conflict
            last_end = *range.end();
            merged_ranges.push(range.clone());
        } else {
            // merge conflict
            let last_range = merged_ranges.pop().expect("expected nonempty ranges");
            let new_start = min(last_range.start(), range.start());
            let new_end = max(last_range.end(), range.end());
            let mod_range = RangeInclusive::new(*new_start, *new_end);
            merged_ranges.push(mod_range);
            last_end = *new_end;
        }

        #[cfg(test)]
        #[cfg(debug_assertions)]
        println!("merged_intervals after '{:?}': {:#?}", range, merged_ranges);
    }

    Ok(merged_ranges)
}

fn find_fresh_ids(parsed: &ParsedContent) -> anyhow::Result<Vec<usize>> {
    let mut fresh_ids = Vec::new();

    // merge intervals problem
    let merged_intervals = merge_intervals(&parsed.ranges)?;

    #[cfg(test)]
    println!("merged ranges: {:#?}", merged_intervals);

    // binary search merge ranges for each ID to see if contained
    for id in &parsed.ids {
        let partition_idx = merged_intervals.partition_point(|range| range.end() < id);
        if partition_idx < merged_intervals.len() && merged_intervals[partition_idx].contains(id) {
            fresh_ids.push(*id);
        }
    }

    Ok(fresh_ids)
}

fn main() -> anyhow::Result<()> {
    let contents = std::fs::read_to_string("./data/day-05-input.txt")?;

    /* Part 1 */
    let parsed = parse_contents(&contents)?;

    let fresh_ids = find_fresh_ids(&parsed)?;
    println!("Part 1: {}", fresh_ids.len());

    /* Part 2 */
    let merged_intervals = merge_intervals(&parsed.ranges)?;
    let total_fresh_ids: usize = merged_intervals.iter().map(|int| int.clone().count()).sum();
    println!("Part 2: {}", total_fresh_ids);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_merge_intervals() {
        let ranges: Vec<RangeInclusive<usize>> = vec![
            RangeInclusive::new(3, 5),
            RangeInclusive::new(10, 14),
            RangeInclusive::new(16, 20),
            RangeInclusive::new(12, 18),
        ];
        let merged = merge_intervals(&ranges).unwrap();
        let expected = vec![RangeInclusive::new(3, 5), RangeInclusive::new(10, 20)];
        assert_eq!(merged, expected);
    }

    #[test]
    fn test_part1_example() {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        let parsed = parse_contents(input).unwrap();
        let fresh_ids = find_fresh_ids(&parsed).unwrap();
        assert_eq!(fresh_ids.len(), 3)
    }
}
