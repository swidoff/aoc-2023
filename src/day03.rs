use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter;

fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day03.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap())
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum ElementType {
    Number(u64),
    Symbol(char),
}

#[derive(Debug)]
struct Element {
    value: ElementType,
    row: usize,
    start_col: usize,
    end_col: usize,
}

fn parse_schematic(
    input: impl Iterator<Item = String>,
) -> (Vec<Element>, HashMap<(usize, usize), ElementType>) {
    let mut elements = Vec::new();
    let mut locations = HashMap::new();

    for (row, line) in input.enumerate() {
        let mut number = 0u64;
        let mut digits = 0;

        for (col, char) in line.chars().chain(iter::once('.')).enumerate() {
            if let Some(c) = char.to_digit(10) {
                number = number * 10 + c as u64;
                digits += 1
            } else {
                if digits > 0 {
                    let start_col = col - digits;
                    let end_col = col - 1;
                    let n = Element {
                        value: ElementType::Number(number),
                        row,
                        start_col,
                        end_col,
                    };
                    for c in start_col..(end_col + 1) {
                        locations.insert((row, c), n.value);
                    }
                    elements.push(n);

                    number = 0;
                    digits = 0;
                }
                if char != '.' {
                    let s = Element {
                        value: ElementType::Symbol(char),
                        row,
                        start_col: col,
                        end_col: col,
                    };
                    locations.insert((row, col), s.value);
                    elements.push(s);
                }
            }
        }
    }

    (elements, locations)
}

fn find_adjacent<'a>(
    e: &Element,
    locations: &'a HashMap<(usize, usize), ElementType>,
) -> HashSet<&'a ElementType> {
    let start_row = if e.row > 0 { e.row - 1 } else { e.row };
    let start_col = if e.start_col > 0 {
        e.start_col - 1
    } else {
        e.start_col
    };
    let end_row = e.row + 1;
    let end_col = e.end_col + 1;
    let mut res = HashSet::new();
    for row in start_row..(end_row + 1) {
        for col in start_col..(end_col + 1) {
            if row == e.row && col >= e.start_col && col <= e.end_col {
                continue;
            } else if let Some(adjacent) = locations.get(&(row, col)) {
                res.insert(adjacent);
            }
        }
    }
    return res;
}

fn part1(input: impl Iterator<Item = String>) -> u64 {
    let (elements, locations) = parse_schematic(input);
    elements
        .iter()
        .filter_map(|n| match n.value {
            ElementType::Number(value) => {
                let has_adjacent_symbol = find_adjacent(n, &locations).iter().any(|&v| match v {
                    ElementType::Symbol(_) => true,
                    _ => false,
                });
                if has_adjacent_symbol {
                    Some(value)
                } else {
                    None
                }
            }
            _ => None,
        })
        .sum()
}

fn part2(input: impl Iterator<Item = String>) -> u64 {
    let (elements, locations) = parse_schematic(input);
    elements
        .iter()
        // Find all the gears
        .filter(|&n| match n.value {
            ElementType::Symbol('*') => true,
            _ => false,
        })
        // Count number neighbors and take their product
        .map(|n| {
            find_adjacent(n, &locations)
                .iter()
                .fold((1, 0), |(prod, count), &adj| match adj {
                    ElementType::Number(v) => (prod * v, count + 1),
                    ElementType::Symbol(_) => (0, 0),
                })
        })
        // Keep only results with 2 numeric neighbors
        .filter_map(|(prod, count)| {
            if prod > 0 && count == 2 {
                Some(prod)
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, read_file};

    const EXAMPLE: &str = "467..114..
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
        assert_eq!(part1(EXAMPLE.lines().map(|v| v.to_string())), 4361);
    }

    #[test]
    fn test_part1() {
        let res = part1(read_file());
        println!("{}", res);
        assert_eq!(res, 525911);
        // 521979
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE.lines().map(|v| v.to_string())), 467835);
    }

    #[test]
    fn test_part2() {
        let res = part2(read_file());
        println!("{}", res);
        assert_eq!(res, 75805607);
    }
}
