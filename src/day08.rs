use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter;

// use std::str::FromStr;
fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day08.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap())
}

fn parse_input(
    input: impl Iterator<Item = String>,
) -> (Vec<char>, HashMap<String, (String, String)>) {
    let mut instructions = Vec::new();
    let mut nodes = HashMap::new();
    for line in input {
        if instructions.is_empty() {
            instructions.extend(line.chars());
        } else if !line.is_empty() {
            let (node, pairs) = line.split_once(" = ").unwrap();
            let (n1, n2) = pairs[1..pairs.len() - 1].split_once(", ").unwrap();
            nodes.insert(node.to_string(), (n1.to_string(), n2.to_string()));
        }
    }
    (instructions, nodes)
}

fn part1(input: impl Iterator<Item = String>) -> u64 {
    let (instructions, nodes) = parse_input(input);
    let node = "AAA".to_string();
    steps_to_z(&node, &instructions, &nodes)
}

fn steps_to_z(
    n: &String,
    instructions: &Vec<char>,
    nodes: &HashMap<String, (String, String)>,
) -> u64 {
    let mut node = n;
    let mut count = 0;
    for &c in instructions.iter().cycle() {
        let (left, right) = nodes.get(node).unwrap();
        match c {
            'L' => node = left,
            'R' => node = right,
            _ => panic!(),
        }
        count += 1;

        if node.ends_with("Z") {
            break;
        }
    }
    count
}

fn part2(input: impl Iterator<Item = String>) -> u64 {
    let (instructions, nodes) = parse_input(input);
    let mut cycle = nodes
        .keys()
        .filter(|&k| k.ends_with("A"))
        .map(|node| steps_to_z(node, &instructions, &nodes))
        .collect_vec();
    let mut turns = cycle.clone();

    loop {
        if let Ok(&v) = turns.iter().all_equal_value() {
            if v > 0 {
                break;
            }
        }

        let i = turns.iter().position_min().unwrap();
        turns[i] += cycle[i];
    }
    turns[0]
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, read_file};

    const EXAMPLE1: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE1.lines().map(|v| v.to_string())), 6);
    }

    #[test]
    fn test_part1() {
        let res = part1(read_file());
        println!("{}", res);
        assert_eq!(res, 19783);
    }

    const EXAMPLE2: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE2.lines().map(|v| v.to_string())), 6);
    }

    #[test]
    fn test_part2() {
        let res = part2(read_file());
        println!("{}", res);
        assert_eq!(res, 9177460370549);
    }
}
