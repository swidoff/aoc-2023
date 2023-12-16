use itertools::Itertools;
use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day16.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap())
}

fn parse_input(input: impl Iterator<Item = String>) -> Vec<Vec<char>> {
    input.map(|line| line.chars().collect_vec()).collect_vec()
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
struct State {
    row: i64,
    col: i64,
    dir_row: i64,
    dir_col: i64,
}

fn count_energized(grid: &Vec<Vec<char>>, initial: State) -> usize {
    let dim = grid.len() as i64;
    let mut energized = HashSet::new();
    let mut seen = HashSet::new();

    let mut q = VecDeque::new();
    q.push_back(initial);

    while let Some(state @ State { row, col, dir_row, dir_col }) = q.pop_front() {
        if seen.contains(&state) {
            continue;
        } else {
            seen.insert(state.clone());
            energized.insert((row, col));
        }

        let new_states = match grid[row as usize][col as usize] {
            '\\' if dir_row > 0 => vec![State { row, col: col + 1, dir_row: 0, dir_col: 1 }],
            '\\' if dir_row < 0 => vec![State { row, col: col - 1, dir_row: 0, dir_col: -1 }],
            '\\' if dir_col > 0 => vec![State { row: row + 1, col, dir_row: 1, dir_col: 0 }],
            '\\' if dir_col < 0 => vec![State { row: row - 1, col, dir_row: -1, dir_col: 0 }],
            '/' if dir_row > 0 => vec![State { row, col: col - 1, dir_row: 0, dir_col: -1 }],
            '/' if dir_row < 0 => vec![State { row, col: col + 1, dir_row: 0, dir_col: 1 }],
            '/' if dir_col > 0 => vec![State { row: row - 1, col, dir_row: -1, dir_col: 0 }],
            '/' if dir_col < 0 => vec![State { row: row + 1, col, dir_row: 1, dir_col: 0 }],
            '|' if dir_col != 0 => vec![
                State { row: row - 1, col, dir_row: -1, dir_col: 0 },
                State { row: row + 1, col, dir_row: 1, dir_col: 0 },
            ],
            '-' if dir_row != 0 => vec![
                State { row, col: col - 1, dir_row: 0, dir_col: -1 },
                State { row, col: col + 1, dir_row: 0, dir_col: 1 },
            ],
            _ => vec![State { row: row + dir_row, col: col + dir_col, ..state }],
        };
        for new_state in new_states {
            if new_state.row >= 0
                && new_state.row < dim
                && new_state.col >= 0
                && new_state.col < dim
            {
                q.push_back(new_state);
            }
        }
    }

    energized.len()
}

fn part1(input: impl Iterator<Item = String>) -> usize {
    let grid = parse_input(input);
    let initial = State { row: 0, col: 0, dir_row: 0, dir_col: 1 };
    count_energized(&grid, initial)
}

fn part2(input: impl Iterator<Item = String>) -> usize {
    let grid = parse_input(input);
    let dim = grid.len();
    let last = dim as i64 - 1;
    (0..dim)
        .flat_map(|i| {
            let last_i = (dim - i - 1) as i64;
            [
                State { row: i as i64, col: 0, dir_row: 0, dir_col: 1 },
                State { row: last_i, col: last, dir_row: 0, dir_col: -1 },
                State { row: 0, col: i as i64, dir_row: 1, dir_col: 0 },
                State { row: last, col: last_i, dir_row: -1, dir_col: 0 },
            ]
            .into_iter()
        })
        .map(|initial| count_energized(&grid, initial))
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, read_file};

    const EXAMPLE1: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE1.lines().map(|v| v.to_string())), 46);
    }

    #[test]
    fn test_part1() {
        let res = part1(read_file());
        println!("{}", res);
        assert_eq!(res, 8112);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE1.lines().map(|v| v.to_string())), 51);
    }

    #[test]
    fn test_part2() {
        let res = part2(read_file());
        println!("{}", res);
        assert_eq!(res, 8314);
    }
}
