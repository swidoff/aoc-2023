use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter;
use std::str::FromStr;

use itertools::Itertools;

fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day12.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap())
}

fn parse_input(input: impl Iterator<Item = String>) -> Vec<(String, Vec<usize>)> {
    input
        .map(|line| {
            let (line, groups) = line.split_once(" ").unwrap();
            let groups = groups
                .split(",")
                .map(|s| usize::from_str(s).unwrap())
                .collect_vec();
            (line.to_string(), groups)
        })
        .collect_vec()
}

fn num_arrangements<'a>(
    line: &'a str,
    num_in_group: usize,
    groups: &'a [usize],
    cache: &mut HashMap<(&'a str, usize, &'a [usize]), usize>,
) -> usize {
    if line.is_empty() {
        return if num_in_group == 0 && groups.is_empty() { 1 } else { 0 };
    }

    let cache_key = (line, num_in_group, groups);
    if let Some(&count) = cache.get(&cache_key) {
        return count;
    }

    let mut count = 0;
    let c = &line[0..1];

    if c == "." || c == "?" {
        if num_in_group > 0 {
            if num_in_group == groups[0] {
                count += num_arrangements(&line[1..], 0, &groups[1..], cache);
            }
        } else {
            count += num_arrangements(&line[1..], num_in_group, groups, cache);
        }
    }
    if c == "#" || c == "?" {
        if !groups.is_empty() && num_in_group < groups[0] {
            count += num_arrangements(&line[1..], num_in_group + 1, groups, cache);
        }
    }
    cache.insert(cache_key, count);
    count
}

fn part1(input: impl Iterator<Item = String>) -> usize {
    parse_input(input)
        .into_iter()
        .map(|(line, groups)| {
            let mut cache = HashMap::new();
            num_arrangements(
                (line + ".").as_str(),
                0,
                &groups[0..groups.len()],
                &mut cache,
            )
        })
        .sum()
}

fn part2(input: impl Iterator<Item = String>) -> usize {
    parse_input(input)
        .into_iter()
        .map(|(line, groups)| {
            let mut cache = HashMap::new();
            let line = iter::repeat(line).take(5).join("?") + ".";
            let groups = groups.repeat(5);
            num_arrangements(line.as_str(), 0, &groups[0..groups.len()], &mut cache)
        })
        .sum()
}
#[cfg(test)]
mod tests {
    use super::{part1, part2, read_file};

    const EXAMPLE1: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";

    #[test]
    fn test_part1_example() {
        // assert_eq!(
        //     num_arrangements("???.###.".to_string(), 0, vec![1, 1, 3]),
        //     1
        // );
        // assert_eq!(
        //     num_arrangements(".??..??...?##.".to_string(), 0, vec![1, 1, 3]),
        //     4
        // );
        // assert_eq!(
        //     num_arrangements("?#?#?#?#?#?#?#?.".to_string(), 0, vec![1, 3, 1, 6]),
        //     1
        // );
        // assert_eq!(
        //     num_arrangements("????.#...#....".to_string(), 0, vec![4, 1, 1]),
        //     1
        // );
        // assert_eq!(
        //     num_arrangements("????.######..#####.".to_string(), 0, vec![1, 6, 5]),
        //     4
        // );
        // assert_eq!(
        //     num_arrangements("?###????????.".to_string(), 0, vec![3, 2, 1]),
        //     10
        // );
        assert_eq!(part1(EXAMPLE1.lines().map(|v| v.to_string())), 21);
        // assert_eq!(
        //     num_arrangements(".???????#?.".to_string(), 0, vec![1, 4]),
        //     7
        // );
    }

    #[test]
    fn test_part1() {
        let res = part1(read_file());
        println!("{}", res);
        assert_eq!(res, 6981);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE1.lines().map(|v| v.to_string())), 525152);
    }

    #[test]
    fn test_part2() {
        let res = part2(read_file());
        println!("{}", res);
        assert_eq!(res, 4546215031609);
    }
}
