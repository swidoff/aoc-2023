use itertools::Itertools;
use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter;
use std::str::FromStr;

fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day04.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap())
}

fn parse_winning_numbers(input: impl Iterator<Item = String>) -> impl Iterator<Item = usize> {
    fn parse_numbers(s: &str) -> HashSet<u32> {
        s.split_whitespace()
            .map(|s| u32::from_str(s).unwrap())
            .collect()
    }

    input.map(|line| {
        let (_, numbers) = line.split_once(": ").unwrap();
        let (winning_numbers, my_numbers) = numbers.split_once(" | ").unwrap();
        parse_numbers(winning_numbers)
            .intersection(&parse_numbers(my_numbers))
            .count()
    })
}

fn part1(input: impl Iterator<Item = String>) -> u32 {
    parse_winning_numbers(input)
        .filter(|&num_winning_numbers| num_winning_numbers > 0)
        .map(|num_winning_numbers| 2_u32.pow(num_winning_numbers as u32 - 1))
        .sum()
}

fn part2(input: impl Iterator<Item = String>) -> u32 {
    let cards = parse_winning_numbers(input).collect_vec();
    let mut counts = iter::repeat(0).take(cards.len()).collect_vec();
    let mut q = VecDeque::from((0..cards.len()).collect_vec());

    while let Some(i) = q.pop_front() {
        counts[i] += 1;
        for j in (i + 1)..(i + cards[i] + 1) {
            q.push_back(j);
        }
    }

    counts.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, read_file};

    const EXAMPLE1: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE1.lines().map(|v| v.to_string())), 13);
    }

    #[test]
    fn test_part1() {
        let res = part1(read_file());
        println!("{}", res);
        assert_eq!(res, 21105);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE1.lines().map(|v| v.to_string())), 30);
    }

    #[test]
    fn test_part2() {
        let res = part2(read_file());
        println!("{}", res);
        assert_eq!(res, 5329815);
    }
}
