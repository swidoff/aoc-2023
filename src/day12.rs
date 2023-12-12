use itertools::Itertools;
use std::fs::File;
use std::io::{stdout, BufRead, BufReader, Write};
use std::iter;

use std::str::FromStr;
fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day12.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap())
}

struct Group {
    line: Vec<char>,
    groups: Vec<usize>,
}

fn parse_input(input: impl Iterator<Item = String>) -> Vec<Group> {
    input
        .map(|line| {
            let (line, groups) = line.split_once(" ").unwrap();
            let line = line
                .chars()
                .map(|c| if c == '.' { ' ' } else { c })
                .collect_vec();
            let groups = groups
                .split(",")
                .map(|s| usize::from_str(s).unwrap())
                .collect_vec();
            Group { line, groups }
        })
        .collect_vec()
}

fn list_arrangements(group: &Group) -> Vec<Vec<char>> {
    let num_unknown = group.line.iter().filter(|&&c| c == '?').count();
    let num_known = group.line.iter().filter(|&&c| c == '#').count();
    let num_required: usize = group.groups.iter().sum();
    let num_to_replace = num_required - num_known;

    let mut res = Vec::new();
    for replacements in (0..num_unknown).combinations(num_to_replace) {
        let mut new_line = group.line.clone();

        let mut source_i = 0;
        let mut replacement_i = 0;
        for j in 0..new_line.len() {
            if new_line[j] == '?' {
                if replacement_i < replacements.len() && replacements[replacement_i] == source_i {
                    new_line[j] = '#';
                    replacement_i += 1;
                } else {
                    new_line[j] = ' ';
                }
                source_i += 1;
            }
        }

        let counts = String::from_iter(&new_line)
            .split_whitespace()
            .map(|g| g.len())
            .collect_vec();
        if counts == group.groups {
            res.push(new_line);
        }
    }
    res
}

fn part1(input: impl Iterator<Item = String>) -> usize {
    let groups = parse_input(input);
    groups.iter().map(|g| list_arrangements(g).len()).sum()
}

fn part2(input: impl Iterator<Item = String>) -> usize {
    let groups = parse_input(input);
    groups
        .iter()
        .enumerate()
        .map(|(i, g1)| {
            let g2 = Group {
                line: g1
                    .line
                    .iter()
                    .map(|&c| c)
                    .chain(iter::once('?'))
                    .chain(g1.line.iter().map(|&c| c))
                    .collect_vec(),
                groups: g1.groups.repeat(2),
            };

            let arr1 = list_arrangements(g1).len();
            let arr2 = list_arrangements(&g2).len();

            let possible_answer = arr2.pow(4) / arr1.pow(3);
            println!("{} {} {} {}", i, arr1, arr2, possible_answer);
            _ = stdout().flush();
            possible_answer
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
        // assert_eq!(res, 0);
    }
}
