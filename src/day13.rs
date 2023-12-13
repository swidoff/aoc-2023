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

fn is_vertical_fold(grid: &Vec<Vec<char>>, col: usize) -> bool {
    let n_cols = grid[0].len();
    let n_rows = grid.len();
    let mut i = col as i64;
    let mut j = col + 1;
    while i >= 0 && j < n_cols {
        if !(0..n_rows).all(|r| grid[r][i as usize] == grid[r][j]) {
            return false;
        }
        i -= 1;
        j += 1;
    }
    true
}

fn is_horizontal_fold(grid: &Vec<Vec<char>>, row: usize) -> bool {
    let n_cols = grid[0].len();
    let n_rows = grid.len();
    let mut i = row as i64;
    let mut j = row + 1;
    while i >= 0 && j < grid.len() {
        if !(0..n_cols).all(|c| grid[i as usize][c] == grid[j][c]) {
            return false;
        }
        i -= 1;
        j += 1;
    }
    true
}

fn part1(input: impl Iterator<Item = String>) -> usize {
    let grids = parse_input(input);
    let mut res = 0;
    for grid in grids {
        let n_cols = grid[0].len();
        let n_rows = grid.len();
        for col in 0..n_cols - 1 {
            if is_vertical_fold(&grid, col) {
                println! {"col: {col}"}
                res += col + 1;
                continue;
            }
        }
        for row in 0..n_rows - 1 {
            if is_horizontal_fold(&grid, row) {
                println! {"row: {row}"}
                res += 100 * (row + 1);
                continue;
            }
        }
    }
    res
}

fn count_vertical_fold_diffs(grid: &Vec<Vec<char>>, col: usize) -> usize {
    let n_cols = grid[0].len();
    let n_rows = grid.len();
    let mut i = col as i64;
    let mut j = col + 1;
    let mut count = 0;

    while i >= 0 && j < n_cols {
        count += (0..n_rows)
            .filter(|&r| grid[r][i as usize] != grid[r][j])
            .count();
        i -= 1;
        j += 1;
    }
    count
}

fn count_horizontal_fold_diffs(grid: &Vec<Vec<char>>, row: usize) -> usize {
    let n_cols = grid[0].len();
    let n_rows = grid.len();
    let mut i = row as i64;
    let mut j = row + 1;
    let mut count = 0;

    while i >= 0 && j < grid.len() {
        count += (0..n_cols)
            .filter(|&c| grid[i as usize][c] != grid[j][c])
            .count();
        i -= 1;
        j += 1;
    }
    count
}

fn part2(input: impl Iterator<Item = String>) -> usize {
    let grids = parse_input(input);
    let mut res = 0;
    for grid in grids {
        let n_cols = grid[0].len();
        let n_rows = grid.len();
        for col in 0..n_cols - 1 {
            let count = count_vertical_fold_diffs(&grid, col);
            println!("col: {col}, count: {count}");
            if count == 1 {
                res += col + 1;
                continue;
            }
        }
        for row in 0..n_rows - 1 {
            let count = count_horizontal_fold_diffs(&grid, row);
            println!("row: {row}, count: {count}");
            if count == 1 {
                res += 100 * (row + 1);
                continue;
            }
        }
    }
    println!();
    res
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
