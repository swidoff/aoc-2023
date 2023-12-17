use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day17.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap())
}

fn parse_input(input: impl Iterator<Item = String>) -> Vec<Vec<char>> {
    input.map(|line| line.chars().collect_vec()).collect_vec()
}

const NORTH: i8 = 0;
const EAST: i8 = 1;
const SOUTH: i8 = 2;
const WEST: i8 = 3;

fn min_heat_loss(grid: &mut Vec<Vec<char>>, min_steps_to_turn: i32, max_forward_steps: i32) -> u32 {
    let dim = grid.len() as i32;
    let mut distances = HashMap::new();
    let mut q = BinaryHeap::new();
    let mut res = 10_000_000;
    q.push(Reverse((0, 0, 0, 1, EAST)));
    q.push(Reverse((0, 0, 0, 1, SOUTH)));

    while let Some(Reverse((distance, row, col, steps, dir))) = q.pop() {
        if let Some(&old_distance) = distances.get(&(row, col, steps, dir)) {
            if old_distance <= distance {
                continue;
            }
        }
        distances.insert((row, col, steps, dir), distance);

        if row == dim - 1 && col == dim - 1 {
            res = res.min(distance);
            continue;
        }

        for (new_row, new_col, new_dir) in [
            (row + 1, col, SOUTH),
            (row, col + 1, EAST),
            (row - 1, col, NORTH),
            (row, col - 1, WEST),
        ] {
            if new_row >= 0
                && new_row < dim
                && new_col >= 0
                && new_col < dim
                && (new_dir != dir || steps < max_forward_steps)
                && (new_dir == dir || steps >= min_steps_to_turn)
                && ((new_dir + 2) % 4 != dir)
            {
                let new_steps = if dir == new_dir { steps + 1 } else { 1 };
                let new_dist = distance
                    + grid[new_row as usize][new_col as usize]
                        .to_digit(10)
                        .unwrap();
                q.push(Reverse((new_dist, new_row, new_col, new_steps, new_dir)));
            }
        }
    }
    res
}

fn part1(input: impl Iterator<Item = String>) -> u32 {
    let mut grid = parse_input(input);
    min_heat_loss(&mut grid, 0, 3)
}

fn part2(input: impl Iterator<Item = String>) -> u32 {
    let mut grid = parse_input(input);
    min_heat_loss(&mut grid, 4, 10)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, read_file};

    const EXAMPLE1: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE1.lines().map(|v| v.to_string())), 102);
    }

    #[test]
    fn test_part1() {
        let res = part1(read_file());
        println!("{}", res);
        assert_eq!(res, 959);
        // 948
        // 925 <-- low
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE1.lines().map(|v| v.to_string())), 94);
    }

    #[test]
    fn test_part2() {
        let res = part2(read_file());
        println!("{}", res);
        assert_eq!(res, 1135);
    }
}
