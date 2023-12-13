use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

// use std::str::FromStr;
fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day13.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap())
}

fn parse_input(input: impl Iterator<Item = String>) -> Vec<Vec<Vec<char>>> {
    let mut res = Vec::new();
    let mut curr = Vec::new();
    for line in input {
        if line.is_empty() {
            res.push(curr);
            curr = Vec::new();
        } else {
            curr.push(line.chars().collect_vec());
        }
    }
    if !curr.is_empty() {
        res.push(curr);
    }
    res
}

fn result_for_diffs(grids: &Vec<Vec<Vec<char>>>, count: usize) -> usize {
    grids
        .iter()
        .map(|grid| {
            if let Some(col) =
                (0..grid[0].len() - 1).find(|&col| vertical_diffs(&grid, col) == count)
            {
                col + 1
            } else if let Some(row) =
                (0..grid.len() - 1).find(|&row| horizontal_diffs(&grid, row) == count)
            {
                100 * (row + 1)
            } else {
                0
            }
        })
        .sum()
}

fn vertical_diffs(grid: &Vec<Vec<char>>, col: usize) -> usize {
    let n_cols = grid[0].len();
    let n_rows = grid.len();
    (0..(col + 1).min(n_cols - col - 1))
        .flat_map(|d| (0..n_rows).filter(move |&r| grid[r][col - d] != grid[r][col + d + 1]))
        .count()
}

fn horizontal_diffs(grid: &Vec<Vec<char>>, row: usize) -> usize {
    let n_cols = grid[0].len();
    let n_rows = grid.len();
    (0..(row + 1).min(n_rows - row - 1))
        .flat_map(|d| (0..n_cols).filter(move |&c| grid[row - d][c] != grid[row + d + 1][c]))
        .count()
}

fn part1(input: impl Iterator<Item = String>) -> usize {
    let grids = parse_input(input);
    result_for_diffs(&grids, 0)
}

fn part2(input: impl Iterator<Item = String>) -> usize {
    let grids = parse_input(input);
    result_for_diffs(&grids, 1)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, read_file};

    const EXAMPLE1: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE1.lines().map(|v| v.to_string())), 405);
    }

    #[test]
    fn test_part1() {
        let res = part1(read_file());
        println!("{}", res);
        assert_eq!(res, 33975);
        // 32663
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE1.lines().map(|v| v.to_string())), 400);
    }

    #[test]
    fn test_part2() {
        let res = part2(read_file());
        println!("{}", res);
        assert_eq!(res, 29083);
    }
}
