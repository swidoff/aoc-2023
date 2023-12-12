use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter;
use std::str::FromStr;

use itertools::Itertools;
use memoize::memoize;

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

#[memoize]
fn count_arrangements(line: String, groups: Vec<usize>) -> usize {
    let mut count = 0;
    let line = line.trim_start_matches(".");
    let group_size = groups[0];

    if line.len() >= group_size && is_complete(line, group_size, groups.len() == 1) {
        if groups.len() > 1 {
            if line.len() > group_size + 1 {
                count +=
                    count_arrangements(line[group_size + 1..].to_owned(), groups[1..].to_owned());
            }
        } else {
            count += 1;
        }
    }

    let min_length = groups.iter().sum::<usize>() + groups.len() - 1;
    if line.len() > min_length && line[0..].starts_with("?") {
        count += count_arrangements(line[1..].to_string(), groups);
    }
    count
}

fn is_complete(line: &str, group_size: usize, last_group: bool) -> bool {
    line.len() >= group_size
        && line.chars().take(group_size).all(|c| c == '#' || c == '?')
        && (line.len() == group_size
            || line[group_size..].starts_with(".")
            || line[group_size..].starts_with("?"))
        && (!last_group || !line.chars().dropping(group_size).any(|c| c == '#'))
}

fn part1(input: impl Iterator<Item = String>) -> usize {
    parse_input(input)
        .into_iter()
        .map(|(line, groups)| count_arrangements(line, groups))
        .sum()
}

fn part2(input: impl Iterator<Item = String>) -> usize {
    parse_input(input)
        .into_iter()
        .map(|(line, groups)| {
            let line = iter::repeat(line).take(5).join("?");
            let groups = groups.repeat(5);
            count_arrangements(line, groups)
        })
        .sum()
}
#[cfg(test)]
mod tests {
    use super::{count_arrangements, part1, part2, read_file};

    const EXAMPLE1: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";

    #[test]
    fn test_part1_example() {
        assert_eq!(count_arrangements(".???????#?".to_string(), vec![1, 4]), 7);
        assert_eq!(part1(EXAMPLE1.lines().map(|v| v.to_string())), 21);
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
