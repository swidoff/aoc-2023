use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;
use std::str::FromStr;

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
            edges.insert((r, c), color.clone());
        }
    }

    let &(min_r, min_c) = edges.keys().min().unwrap();
    let mut seen = HashSet::new();
    let mut q = VecDeque::new();
    q.push_back((min_r + 1, min_c + 1));
    while let Some(p @ (r, c)) = q.pop_front() {
        if seen.contains(&p) {
            continue;
        }
        seen.insert(p);

        for new_p in [(r - 1, c), (r, c - 1), (r + 1, c), (r, c + 1)] {
            if !edges.contains_key(&new_p) {
                q.push_back(new_p);
            }
        }
    }

    edges.len() + seen.len()
}

fn parse_input2(input: impl Iterator<Item = String>) -> Vec<(char, i64)> {
    input
        .map(|line| {
            let (_dir, _num, color) = line.split_whitespace().collect_tuple().unwrap();
            let color = color[2..2 + 6].to_string();
            let num = i64::from_str_radix(&color[0..color.len() - 1], 16).unwrap();
            let dir = match i64::from_str(&color[color.len() - 1..]).unwrap() {
                0 => 'R',
                1 => 'D',
                2 => 'L',
                3 => 'U',
                _ => panic!(),
            };
            (dir, num)
        })
        .collect_vec()
}

fn part2(input: impl Iterator<Item = String>) -> i64 {
    // let plan = parse_input(input);
    let plan = parse_input2(input);
    let mut vertices = Vec::new();
    let mut x = 0;
    let mut y = 0;
    vertices.push((0, 0));
    let mut prior_inside = false;

    // In order to use the shoelace formula, we need to convert the row/column edge to x,y
    // vertices where the edges are in the inside of line segments describe by the vertices.
    // The walk around the edge is luckily clockwise, so inside is to the right.
    for (i, (dir, num)) in plan.iter().enumerate() {
        let next_dir = if i < plan.len() - 1 { plan[i + 1].0 } else { plan[0].0 };
        let next_inside = match (dir, next_dir) {
            ('L', 'U') => false,
            ('L', 'D') => true,
            ('D', 'L') => false,
            ('D', 'R') => true,
            ('R', 'U') => true,
            ('R', 'D') => false,
            ('U', 'L') => true,
            ('U', 'R') => false,
            _ => panic!(),
        };

        let (x_delta, y_delta) = match (dir, next_inside, prior_inside) {
            ('L', false, false) => (-*num - 1, 0),
            ('L', false, true) => (-*num, 0),
            ('L', true, false) => (-*num, 0),
            ('L', true, true) => (-*num + 1, 0),
            ('D', false, false) => (0, *num + 1),
            ('D', false, true) => (0, *num),
            ('D', true, false) => (0, *num),
            ('D', true, true) => (0, *num - 1),
            ('R', false, false) => (*num + 1, 0),
            ('R', false, true) => (*num, 0),
            ('R', true, false) => (*num, 0),
            ('R', true, true) => (*num - 1, 0),
            ('U', false, false) => (0, -*num - 1),
            ('U', false, true) => (0, -*num),
            ('U', true, false) => (0, -*num),
            ('U', true, true) => (0, -*num + 1),
            _ => panic!(),
        };

        x += x_delta;
        y += y_delta;
        prior_inside = next_inside;
        vertices.push((x, y));
    }

    // Compute area using the shoelace formula.
    let mut area = 0;
    for r @ ((x1, y1), (x2, y2)) in vertices.iter().tuple_windows() {
        let len = x1 * y2 - x2 * y1;
        println!("{r:?}, {len}");
        area += len
    }
    area / 2
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
        assert_eq!(res, 52055);
        // 32509
        // 28205
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE1.lines().map(|v| v.to_string())), 952408144115);
        // assert_eq!(part2(EXAMPLE1.lines().map(|v| v.to_string())), 62);
    }

    #[test]
    fn test_part2() {
        let res = part2(read_file());
        println!("{}", res);
        assert_eq!(res, 67622758357096);
    }
}

/* Notes
Working through the example getting the vertices right.

 0 1 2 3 4 5 6 7
  # # # # # # #
 1
  # . . . . . #
 2
  # # # . . . #
 3
  . . # . . . #
 4
  . . # . . . #
 5
  # # # . # # #
 6
  # . . . # . .
 7
  # # . . # # #
 8
  . # . . . . #
 9
  . # # # # # #
10

(0, 0), (7, 0) =>  0
(7, 0), (7, 6) => 42
(7, 6), (5, 6) => 42 - 30 = 12
(5, 6), (5, 7) => 35 - 30 = 5
(5, 7), (7, 7) => 35 - 49 = -14
(7, 7), (7, 10) => 70 - 49 = 21
(7, 10), (1, 10) => 70 - 10 = 60
(1, 10), (1, 8) => 8 - 10 = -2
(1, 8), (0, 8) => 8
(0, 8), (0, 5) => 0
(0, 5), (2, 5) => -10
(2, 5), (2, 3) => 6 - 10 = -4
(2, 3), (0, 3) => 6
(0, 3), (0, 0) => 0
*/
