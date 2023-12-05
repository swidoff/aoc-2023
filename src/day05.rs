use itertools::Itertools;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day05.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap())
}

/// Convert the input to ranges (source_start, source_end, dest_start, dest_end) for convenience.
///
fn parse_input(input: impl Iterator<Item = String>) -> (Vec<u64>, Vec<Vec<(u64, u64, u64, u64)>>) {
    let mut seeds = Vec::new();
    let mut maps = Vec::new();
    for line in input {
        if seeds.is_empty() {
            let (_, seed_str) = line.split_once(" ").unwrap();
            for seed in seed_str.split_whitespace() {
                seeds.push(u64::from_str(seed).unwrap())
            }
        } else if line.contains(":") {
            maps.push(Vec::new());
        } else if !line.is_empty() {
            let (end, start, range) = line
                .split_whitespace()
                .map(|s| u64::from_str(s).unwrap())
                .collect_tuple()
                .unwrap();
            maps.last_mut()
                .unwrap()
                .push((start, start + range - 1, end, end + range - 1));
        }
    }

    maps.iter_mut().for_each(|m| m.sort());
    (seeds, maps)
}

fn part1(input: impl Iterator<Item = String>) -> u64 {
    let (seeds, maps) = parse_input(input);
    let mut res = u64::MAX;

    for &seed in seeds.iter() {
        let location = map_seed(&maps, seed);
        res = res.min(location);
    }
    res
}

fn map_seed(maps: &Vec<Vec<(u64, u64, u64, u64)>>, seed: u64) -> u64 {
    let mut current = seed;
    for map in maps.iter() {
        let mut next = current;
        for &(source_start, source_end, dest_start, _dest_end) in map.iter() {
            if current >= source_start && current <= source_end {
                next = dest_start + (current - source_start);
                break;
            }
        }
        current = next;
    }
    current
}

fn part2(input: impl Iterator<Item = String>) -> u64 {
    let (seeds, maps) = parse_input(input);
    let mut locs = Vec::new();

    // Operate on ranges rather than individual locations.
    for (&seed_start, &seed_range) in seeds.iter().tuples() {
        locs.push((seed_start, seed_start + seed_range - 1));
    }

    for level in 0..maps.len() {
        // 1. Merge the ranges to collapse any overlap.
        // 2. Map the previous locations to the next level locations.
        let mut q = merge_ranges(&mut locs);
        locs = map_ranges(&mut q, maps.get(level).unwrap());
    }

    locs.iter().map(|&(start, _end)| start).min().unwrap()
}

/// Sort the ranges and merge consecutive ranges that overlap.
///
fn merge_ranges(locs: &mut Vec<(u64, u64)>) -> VecDeque<(u64, u64)> {
    locs.sort();

    let mut current = None;
    let mut q = VecDeque::new();
    for &(start, end) in locs.iter() {
        if let Some((current_start, current_end)) = current {
            if start >= current_start && start <= current_end {
                current = Some((current_start, end.max(current_end)))
            } else {
                q.push_back((current_start, current_end));
                current = Some((start, end))
            }
        } else {
            current = Some((start, end))
        }
    }
    if let Some((start, end)) = current {
        q.push_back((start, end))
    }
    q
}

fn map_ranges(
    ranges: &mut VecDeque<(u64, u64)>,
    map: &Vec<(u64, u64, u64, u64)>,
) -> Vec<(u64, u64)> {
    // There's probably a more clever way to do this with binary search so you don't have to
    // scan the rules linearly and calculate messy overlap, but not going to bother. This is fast
    // enough.
    let mut next_locs = Vec::new();
    while let Some((start, end)) = ranges.pop_front() {
        let mut overlap_found = false;
        for &(source_start, source_end, dest_start, _dest_end) in map.iter() {
            let overlap_start = start.max(source_start);
            let overlap_end = end.min(source_end);
            if overlap_start <= overlap_end {
                let new_start = dest_start + (overlap_start - source_start);
                let new_end = dest_start + (overlap_end - source_start);
                next_locs.push((new_start, new_end));

                if start < overlap_start {
                    ranges.push_front((start, overlap_start - 1));
                }
                if overlap_end < end {
                    ranges.push_front((overlap_end + 1, end));
                }
                overlap_found = true;
                break;
            }
        }
        if !overlap_found {
            next_locs.push((start, end))
        }
    }
    next_locs
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, read_file};

    const EXAMPLE1: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE1.lines().map(|v| v.to_string())), 35);
    }

    #[test]
    fn test_part1() {
        let res = part1(read_file());
        println!("{}", res);
        assert_eq!(res, 346433842);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE1.lines().map(|v| v.to_string())), 46);
    }

    #[test]
    fn test_part2() {
        let res = part2(read_file());
        println!("{}", res);
        assert_eq!(res, 60294664);
    }
}
