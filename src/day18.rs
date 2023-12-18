use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

use itertools::Itertools;

fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day18.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap())
}

fn parse_input(input: impl Iterator<Item = String>) -> Vec<(char, i64)> {
    input
        .map(|line| {
            let (dir, num, color) = line.split_whitespace().collect_tuple().unwrap();
            let (dir,) = dir.chars().take(1).collect_tuple().unwrap();
            let num = num.parse::<i64>().unwrap();
            (dir, num)
        })
        .collect_vec()
}

fn area(plan: Vec<(char, i64)>) -> i64 {
    let mut vertices = Vec::new();
    let mut x = 0;
    let mut y = 0;
    vertices.push((0, 0));
    let mut prior_inside = false;

    // In order to use the shoelace formula, we need to convert the row/column edge to x,y
    // vertices where the edges are inside the line segments describe by the vertices.
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

        match (dir, next_inside, prior_inside) {
            ('L', false, false) => x += -*num - 1,
            ('L', true, true) => x += -*num + 1,
            ('L', _, _) => x += -*num,
            ('D', false, false) => y += *num + 1,
            ('D', true, true) => y += *num - 1,
            ('D', _, _) => y += *num,
            ('R', false, false) => x += *num + 1,
            ('R', true, true) => x += *num - 1,
            ('R', _, _) => x += *num,
            ('U', false, false) => y += -*num - 1,
            ('U', true, true) => y += -*num + 1,
            ('U', _, _) => y += -*num,
            _ => panic!(),
        };
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

fn part1(input: impl Iterator<Item = String>) -> i64 {
    let plan = parse_input(input);
    area(plan)
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
    area(parse_input2(input))
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
