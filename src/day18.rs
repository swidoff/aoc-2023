use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;

fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day18.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap())
}

fn parse_input(input: impl Iterator<Item = String>) -> Vec<(char, i32, String)> {
    input
        .map(|line| {
            let (dir, num, color) = line.split_whitespace().collect_tuple().unwrap();
            let (dir,) = dir.chars().take(1).collect_tuple().unwrap();
            let num = num.parse::<i32>().unwrap();
            let color = color[2..2 + 6].to_string();
            (dir, num, color)
        })
        .collect_vec()
}

fn part1(input: impl Iterator<Item = String>) -> usize {
    let plan = parse_input(input);
    let mut r = 0;
    let mut c = 0;
    let mut min_r = 1_000_000;
    let mut max_r = 0;
    let mut min_c = 1_000_000;
    let mut max_c = 0;
    let mut edges = HashMap::new();

    for (dir, num, color) in plan.into_iter() {
        let (row_delta, col_delta) = match dir {
            'U' => (-1, 0),
            'R' => (0, 1),
            'D' => (1, 0),
            'L' => (0, -1),
            _ => panic!(),
        };
        for _ in 0..num {
            r += row_delta;
            c += col_delta;
            max_r = max_r.max(r);
            max_c = max_c.max(c);
            min_r = min_r.min(r);
            min_c = min_c.min(c);
            edges.insert((r, c), color.clone());
        }
    }

    let mut count = edges.len();
    for r in min_r..=max_r {
        let mut row = Vec::new();
        let mut seen_first_edge = false;
        for c in min_c..=max_c {
            if edges.contains_key(&(r, c)) {
                seen_first_edge = true;
                row.push('#');
                continue;
            }

            if !seen_first_edge {
                row.push('.');
                continue;
            }

            let mut edge_count = 0;
            let mut consecutive_edges = 0;
            for c1 in (c + 1..=max_c) {
                if edges.contains_key(&(r, c1)) {
                    if consecutive_edges == 0 {
                        edge_count += 1;
                    }
                    consecutive_edges += 1;
                } else {
                    consecutive_edges = 0;
                }
            }

            // let edge_count = (c + 1..=max_c)
            //     .filter_map(|c1| edges.get(&(r, c1)))
            //     .unique()
            //     .count();
            if edge_count % 2 == 1 {
                count += 1;
                row.push('*');
            } else {
                row.push('.');
            }
        }
        println!("{}", String::from_iter(row.iter()));
    }
    count
}

fn part2(input: impl Iterator<Item = String>) -> u64 {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, read_file};

    const EXAMPLE1: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE1.lines().map(|v| v.to_string())), 62);
    }

    #[test]
    fn test_part1() {
        let res = part1(read_file());
        println!("{}", res);
        // assert_eq!(res, 0);
        // 32509
        // 28205
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
