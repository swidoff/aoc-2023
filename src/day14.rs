use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

// use std::str::FromStr;
fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day14.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap())
}

fn parse_input(input: impl Iterator<Item = String>) -> Vec<Vec<char>> {
    input.map(|line| line.chars().collect_vec()).collect_vec()
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for line in grid {
        println!("{}", String::from_iter(line.iter()));
    }
    println!();
}

fn tilt_north(grid: &mut Vec<Vec<char>>) {
    for source_row in 1..grid.len() {
        for source_col in 0..grid[source_row].len() {
            if grid[source_row][source_col] != 'O' {
                continue;
            }

            let move_rows = (0..source_row)
                .rev()
                .take_while(|&r| grid[r][source_col] == '.')
                .count();
            if move_rows > 0 {
                grid[source_row - move_rows][source_col] = 'O';
                grid[source_row][source_col] = '.';
            }
        }
    }
}

fn tilt_west(grid: &mut Vec<Vec<char>>) {
    for source_col in 1..grid.len() {
        for source_row in 0..grid.len() {
            if grid[source_row][source_col] != 'O' {
                continue;
            }

            let move_cols = (0..source_col)
                .rev()
                .take_while(|&c| grid[source_row][c] == '.')
                .count();
            if move_cols > 0 {
                grid[source_row][source_col - move_cols] = 'O';
                grid[source_row][source_col] = '.';
            }
        }
    }
}

fn tilt_south(grid: &mut Vec<Vec<char>>) {
    for source_row in (0..grid.len() - 1).rev() {
        for source_col in 0..grid.len() {
            if grid[source_row][source_col] != 'O' {
                continue;
            }

            let move_rows = (source_row + 1..grid.len())
                .take_while(|&r| grid[r][source_col] == '.')
                .count();
            if move_rows > 0 {
                grid[source_row + move_rows][source_col] = 'O';
                grid[source_row][source_col] = '.';
            }
        }
    }
}

fn tilt_east(grid: &mut Vec<Vec<char>>) {
    for source_col in (0..grid.len() - 1).rev() {
        for source_row in 0..grid.len() {
            if grid[source_row][source_col] != 'O' {
                continue;
            }

            let move_cols = (source_col + 1..grid.len())
                .take_while(|&c| grid[source_row][c] == '.')
                .count();
            if move_cols > 0 {
                grid[source_row][source_col + move_cols] = 'O';
                grid[source_row][source_col] = '.';
            }
        }
    }
}

fn count_load(grid: &Vec<Vec<char>>) -> usize {
    let mut count = 0;
    let dim = grid.len();
    for r in 0..dim {
        for c in 0..dim {
            if grid[r][c] == 'O' {
                count += dim - r;
            }
        }
    }
    count
}

fn part1(input: impl Iterator<Item = String>) -> usize {
    let mut grid = parse_input(input);
    tilt_north(&mut grid);
    print_grid(&grid);
    count_load(&grid)
}

fn part2(input: impl Iterator<Item = String>, iterations: usize) -> Option<usize> {
    let mut load_vec = Vec::new();
    let mut load_map = HashMap::new();
    let mut grid = parse_input(input);
    for j in 0..iterations {
        tilt_north(&mut grid);
        tilt_west(&mut grid);
        tilt_south(&mut grid);
        tilt_east(&mut grid);

        let curr_load = count_load(&grid);
        // println!("{} {}:", j + 1, curr_load);

        if let Some(&i) = load_map.get(&grid) {
            let cycle_len = j - i;
            if cycle_len > 3 {
                let offset = i + 1;
                let index = (1_000_000_000 - offset) % cycle_len + offset - 1;
                // println!("Done: offset={offset}, cycle_len={cycle_len}");
                return Some(load_vec[index]);
            }
        }

        load_map.insert(grid.clone(), j);
        load_vec.push(curr_load);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, read_file};

    const EXAMPLE1: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE1.lines().map(|v| v.to_string())), 136);
    }

    #[test]
    fn test_part1() {
        let res = part1(read_file());
        println!("{}", res);
        assert_eq!(res, 109345);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(
            part2(EXAMPLE1.lines().map(|v| v.to_string()), 50).expect("No cycle"),
            64
        );
    }

    #[test]
    fn test_part2() {
        let res = part2(read_file(), 150).expect("No cycle");
        println!("{}", res);
        assert_eq!(res, 112452);
    }
}
