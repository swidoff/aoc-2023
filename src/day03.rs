use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

// use std::str::FromStr;
fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day03.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap())
}

struct Number {
    value: u64,
    row: usize,
    start_col: usize,
    end_col: usize,
}

fn parse_schematic(
    input: impl Iterator<Item = String>,
) -> (Vec<Number>, HashMap<(usize, usize), char>) {
    let mut numbers = Vec::new();
    let mut symbols = HashMap::new();

    for (row, line) in input.enumerate() {
        let mut number = 0u64;
        let mut digits = 0;
        for (col, char) in line.chars().enumerate() {
            if let Some(c) = char.to_digit(10) {
                number = number * 10 + c as u64;
                digits += 1
            } else {
                if digits > 0 {
                    numbers.push(Number {
                        value: number,
                        row,
                        start_col: col - digits,
                        end_col: col - 1,
                    });
                    number = 0;
                    digits = 0;
                }
                if char != '.' {
                    symbols.insert((row, col), char);
                }
            }
        }
        if digits > 0 {
            let col = line.len();
            numbers.push(Number {
                value: number,
                row,
                start_col: col - digits,
                end_col: col - 1,
            });
        }
    }

    (numbers, symbols)
}

fn is_adjacent_to_symbol(number: &Number, symbols: &HashMap<(usize, usize), char>) -> bool {
    let start_row = if number.row > 0 {
        number.row - 1
    } else {
        number.row
    };
    let start_col = if number.start_col > 0 {
        number.start_col - 1
    } else {
        number.start_col
    };
    let end_row = number.row + 1;
    let end_col = number.end_col + 1;
    for row in start_row..(end_row + 1) {
        for col in start_col..(end_col + 1) {
            if symbols.contains_key(&(row, col)) {
                println!("True: {}", { number.value });
                return true;
            }
        }
    }
    println!("False: {}", { number.value });
    return false;
}

fn part1(input: impl Iterator<Item = String>) -> u64 {
    let (numbers, symbols) = parse_schematic(input);
    numbers
        .iter()
        .filter_map(|n| {
            if is_adjacent_to_symbol(n, &symbols) {
                Some(n.value)
            } else {
                None
            }
        })
        .sum()
}

fn part2(_input: impl Iterator<Item = String>) -> u64 {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, read_file};

    const EXAMPLE1: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE1.lines().map(|v| v.to_string())), 4361);
    }

    #[test]
    fn test_part1() {
        let res = part1(read_file());
        println!("{}", res);
        assert_eq!(res, 525911);
        // 521979
    }

    const EXAMPLE2: &str = "
";

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE2.lines().map(|v| v.to_string())), 0);
    }

    #[test]
    fn test_part2() {
        let res = part2(read_file());
        println!("{}", res);
        // assert_eq!(res, 0);
    }
}
