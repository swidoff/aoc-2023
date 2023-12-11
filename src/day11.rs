use itertools::Itertools;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

// use std::str::FromStr;
fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day11.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap())
}

type Coord = (i64, i64);

fn parse_input(input: impl Iterator<Item = String>, multiplier: i64) -> Vec<Coord> {
    let grid = input.map(|line| line.chars().collect_vec()).collect_vec();
    let dim = grid.len();
    let empty_rows: HashSet<usize> = (0..dim)
        .filter(|&r| grid[r].iter().all(|&c| c == '.'))
        .collect();
    let empty_cols: HashSet<usize> = (0..dim)
        .filter(|&c| (0..dim).all(|r| grid[r][c] == '.'))
        .collect();

    let mut res = Vec::new();
    let mut row = 0;
    for i in 0..dim {
        let mut col = 0;
        for j in 0..dim {
            if grid[i][j] == '#' {
                res.push((row, col));
            }

            if empty_cols.contains(&j) {
                col += multiplier;
            } else {
                col += 1;
            }
        }

        if empty_rows.contains(&i) {
            row += multiplier;
        } else {
            row += 1;
        }
    }
    res
}

fn sum_of_distances(galaxies: Vec<Coord>) -> i64 {
    let mut res = 0;
    for (i, &(r1, c1)) in galaxies.iter().enumerate() {
        for &(r2, c2) in galaxies.iter().dropping(i + 1) {
            let distance = (r2 - r1).abs() + (c2 - c1).abs();
            res += distance;
        }
    }

    res
}

fn part1(input: impl Iterator<Item = String>) -> i64 {
    let galaxies = parse_input(input, 2);
    sum_of_distances(galaxies)
}

fn part2(input: impl Iterator<Item = String>, multiplier: i64) -> i64 {
    let galaxies = parse_input(input, multiplier);
    sum_of_distances(galaxies)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, read_file};

    const EXAMPLE1: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE1.lines().map(|v| v.to_string())), 374);
    }

    #[test]
    fn test_part1() {
        let res = part1(read_file());
        println!("{}", res);
        assert_eq!(res, 9957702);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE1.lines().map(|v| v.to_string()), 10), 1030);
        assert_eq!(part2(EXAMPLE1.lines().map(|v| v.to_string()), 100), 8410);
    }

    #[test]
    fn test_part2() {
        let res = part2(read_file(), 1_000_000);
        println!("{}", res);
        assert_eq!(res, 512240933238);
    }
}
