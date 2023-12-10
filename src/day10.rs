use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

// use std::str::FromStr;
fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day10.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap())
}

type Coord = (u64, u64);

fn parse_input(input: impl Iterator<Item = String>) -> Vec<Vec<char>> {
    input.map(|line| line.chars().collect_vec()).collect_vec()
}

fn targets_for_pipe(c: char, row: u64, column: u64) -> Option<[Coord; 2]> {
    match c {
        '|' => Some([(row - 1, column), (row + 1, column)]),
        '-' => Some([(row, column - 1), (row, column + 1)]),
        'L' => Some([(row - 1, column), (row, column + 1)]),
        'J' => Some([(row - 1, column), (row, column - 1)]),
        '7' => Some([(row + 1, column), (row, column - 1)]),
        'F' => Some([(row + 1, column), (row, column + 1)]),
        _ => None,
    }
}

fn find_loop(grid: &Vec<Vec<char>>, start_char: char) -> Vec<Coord> {
    // Find the start character.
    let mut start_loc = (0, 0);
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == 'S' {
                start_loc = (row as u64, col as u64);
                break;
            }
        }
    }

    // Travel around the loop until you return to the same coord.
    let mut res = Vec::new();
    res.push(start_loc);

    let mut prior_loc = start_loc;
    let mut current_loc = targets_for_pipe(start_char, start_loc.0, start_loc.1).unwrap()[0];
    let mut current_char = grid[current_loc.0 as usize][current_loc.1 as usize];

    while current_char != 'S' {
        let (row, col) = current_loc;
        res.push(current_loc);

        let next_loc = *targets_for_pipe(current_char, row, col)
            .unwrap()
            .iter()
            .find(|&&c| c != prior_loc)
            .unwrap();
        prior_loc = current_loc;
        current_loc = next_loc;
        current_char = grid[next_loc.0 as usize][next_loc.1 as usize];
    }

    res
}

fn part1(input: impl Iterator<Item = String>, start_char: char) -> u64 {
    let map = parse_input(input);
    find_loop(&map, start_char).len() as u64 / 2
}

fn part2(input: impl Iterator<Item = String>) -> usize {
    // let map = parse_input(input);
    // let loop_pos = find_loop(&map);
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, read_file};

    const EXAMPLE1: &str = ".....
.S-7.
.|.|.
.L-J.
.....
";

    const EXAMPLE2: &str = "
..F7.
.FJ|.
SJ.L7
|F--J
LJ...
";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE1.lines().map(|v| v.to_string()), 'F'), 4);
        assert_eq!(part1(EXAMPLE2.lines().map(|v| v.to_string()), 'F'), 8);
    }

    #[test]
    fn test_part1() {
        let res = part1(read_file(), 'J');
        println!("{}", res);
        assert_eq!(res, 6697);
    }

    const EXAMPLE3: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
";

    const EXAMPLE4: &str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
";

    const EXAMPLE5: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
";

    #[test]
    fn test_part2_example() {
        // assert_eq!(part2(EXAMPLE3.lines().map(|v| v.to_string())), 4);
        assert_eq!(part2(EXAMPLE4.lines().map(|v| v.to_string())), 8);
        assert_eq!(part2(EXAMPLE5.lines().map(|v| v.to_string())), 10);
    }

    #[test]
    fn test_part2() {
        let res = part2(read_file());
        println!("{}", res);
        // assert_eq!(res, 0);
    }
}
