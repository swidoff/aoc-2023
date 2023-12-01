use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

use itertools::Itertools;

fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day1.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap())
}

fn part1(input: impl Iterator<Item = String>) -> u64 {
    input
        .map(|s| {
            let digits = s.chars().filter(|c| c.is_digit(10)).collect_vec();
            u64::from_str(format!("{}{}", digits[0], digits.last().unwrap()).as_str()).unwrap()
        })
        .sum()
}

const DIGITS: [(&str, u64); 18] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
];

fn digits_from_string(s: &String) -> Vec<u64> {
    let s = s.as_str();
    let mut res = Vec::new();
    for i in 0..s.len() {
        let suffix = &s[i..];
        for (word, digit) in DIGITS {
            if suffix.starts_with(word) {
                res.push(digit);
                break;
            }
        }
    }
    return res;
}

fn part2(input: impl Iterator<Item = String>) -> u64 {
    input
        .map(|s| {
            let digits = digits_from_string(&s);
            digits[0] * 10 + digits.last().unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, read_file};

    const EXAMPLE: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE.lines().map(|v| v.to_string())), 142);
    }

    #[test]
    fn test_part1() {
        let res = part1(read_file());
        println!("{}", res);
        assert_eq!(res, 53080);
    }

    const EXAMPLE2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE2.lines().map(|v| v.to_string())), 281);
    }

    #[test]
    fn test_part2() {
        let res = part2(read_file());
        println!("{}", res);
        assert_eq!(res, 53268);
    }
}
