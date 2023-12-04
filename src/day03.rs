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
                    let value = ElementType::Number(number);
                    let start_col = col - digits;
                    let end_col = col - 1;
                    for c in start_col..(end_col + 1) {
                        locations.insert((row, c), value);
                    }
                    elements.push(Element { value, row, start_col, end_col });
                    number = 0;
                    digits = 0;
                }
                if char != '.' {
                    let value = ElementType::Symbol(char);
                    locations.insert((row, col), value);
                    elements.push(Element { value, row, start_col: col, end_col: col });
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
    let start_col = if e.start_col > 0 { e.start_col - 1 } else { e.start_col };
    let mut res = HashSet::new();
    for row in start_row..(e.row + 2) {
        for col in start_col..(e.end_col + 2) {
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
        // Filter to numbers and extract the value
        .filter_map(|n| match n.value {
            ElementType::Number(v) => Some((n, v)),
            ElementType::Symbol(_) => None,
        })
        // Check that the number is adjacent to a symbol.
        .filter(|(n, _v)| {
            find_adjacent(n, &locations).iter().any(|&v| match v {
                ElementType::Symbol(_) => true,
                _ => false,
            })
        })
        // Sum the values.
        .map(|(_n, v)| v)
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
                    ElementType::Symbol(_) => (0, 0), // Product will now only be zero.
                })
        })
        // Keep only results with 2 numeric neighbors and no symbol neighbors (prod == 0)
        .filter_map(|(prod, count)| if prod > 0 && count == 2 { Some(prod) } else { None })
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
