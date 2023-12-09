use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day09.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap())
}

fn parse_input(input: impl Iterator<Item = String>) -> Vec<Vec<i64>> {
    input
        .map(|l| {
            l.split_whitespace()
                .map(|v| i64::from_str(v).unwrap())
                .collect_vec()
        })
        .collect_vec()
}

fn next_value(v: Vec<i64>) -> i64 {
    let seqs = seq_differences(v);
    seqs.iter().rev().map(|v| v.last().unwrap()).sum()
}

fn previous_value(v: Vec<i64>) -> i64 {
    let seqs = seq_differences(v);
    seqs.iter()
        .rev()
        .map(|v| v.first().unwrap())
        .fold(0, |curr, &n| n - curr)
}

fn seq_differences(v: Vec<i64>) -> Vec<Vec<i64>> {
    let mut seqs = Vec::new();
    seqs.push(v);

    while seqs.last().unwrap().iter().all_equal_value().is_err() {
        let new_seq = seqs
            .last()
            .unwrap()
            .iter()
            .tuple_windows()
            .map(|(&v1, &v2)| v2 - v1)
            .collect_vec();
        seqs.push(new_seq);
    }
    seqs
}

fn part1(input: impl Iterator<Item = String>) -> i64 {
    let seqs = parse_input(input);
    seqs.into_iter().map(|v| next_value(v)).sum()
}

fn part2(input: impl Iterator<Item = String>) -> i64 {
    let seqs = parse_input(input);
    seqs.into_iter().map(|v| previous_value(v)).sum()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, read_file};

    const EXAMPLE1: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE1.lines().map(|v| v.to_string())), 114);
    }

    #[test]
    fn test_part1() {
        let res = part1(read_file());
        println!("{}", res);
        assert_eq!(res, 1708206096);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE1.lines().map(|v| v.to_string())), 2);
    }

    #[test]
    fn test_part2() {
        let res = part2(read_file());
        println!("{}", res);
        assert_eq!(res, 1050);
    }
}
