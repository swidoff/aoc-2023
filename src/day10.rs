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
        '|' => Some([(row + 1, column), (row - 1, column)]),
        '-' => Some([(row, column + 1), (row, column - 1)]),
        'L' => Some([(row, column + 1), (row - 1, column)]),
        'J' => Some([(row, column - 1), (row - 1, column)]),
        '7' => Some([(row + 1, column), (row, column - 1)]),
        'F' => Some([(row, column + 1), (row + 1, column)]),
        c => panic!("{c}"),
    }
}

fn find_loop(grid: &Vec<Vec<char>>, start_loc: Coord, start_char: char) -> Vec<Coord> {
    // Find the start character.
    // Travel around the loop until you return to the same coord.
    let mut res = Vec::new();
    res.push(start_loc);

    let mut prior_loc = start_loc;
    let mut current_loc = targets_for_pipe(start_char, start_loc.0, start_loc.1).unwrap()[0];
    let mut current_char = grid[current_loc.0 as usize][current_loc.1 as usize];

    while current_loc != start_loc {
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

fn find_start_loc(grid: &Vec<Vec<char>>) -> (u64, u64) {
    let mut start_loc = (0, 0);
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == 'S' {
                start_loc = (row as u64, col as u64);
                break;
            }
        }
    }
    start_loc
}

fn part1(input: impl Iterator<Item = String>, start_char: char) -> u64 {
    let map = parse_input(input);
    let start_loc = find_start_loc(&map);
    find_loop(&map, start_loc, start_char).len() as u64 / 2
}

fn part2(input: impl Iterator<Item = String>, start_char: char) -> usize {
    let mut map = parse_input(input);
    let start_loc = find_start_loc(&map);
    let loop_pos1 = find_loop(&map, start_loc, start_char);
    map[start_loc.0 as usize][start_loc.1 as usize] = start_char;

    // Find the upper-left 'F' in the loop and move clockwise around the loop.
    let loop_start_idx = loop_pos1.iter().position_min().unwrap();
    let loop_start = loop_pos1[loop_start_idx];
    let loop_pos = find_loop(&map, loop_start, 'F');

    let mut dir = 'N';
    let mut west_inside = HashMap::new();

    for &pos in loop_pos.iter() {
        let mut char = map[pos.0 as usize][pos.1 as usize];
        if char == 'S' {
            char = start_char;
        }

        // Record boundary and change direction.
        match char {
            'F' => match dir {
                'W' => {
                    dir = 'S';
                    west_inside.insert(pos, true);
                }
                'N' => {
                    dir = 'E';
                    west_inside.insert(pos, false);
                }
                c => panic!("{c}"),
            },
            'J' => match dir {
                'S' => {
                    dir = 'W';
                    west_inside.insert(pos, true);
                }
                'E' => {
                    dir = 'N';
                    west_inside.insert(pos, true);
                }
                _ => panic!(),
            },
            'L' => match dir {
                'S' => {
                    dir = 'E';
                    west_inside.insert(pos, true);
                }
                'W' => {
                    dir = 'N';
                    west_inside.insert(pos, false);
                }
                _ => panic!(),
            },
            '7' => match dir {
                'E' => {
                    dir = 'S';
                    west_inside.insert(pos, true);
                }
                'N' => {
                    dir = 'W';
                    west_inside.insert(pos, false);
                }
                _ => panic!(),
            },
            '|' => match dir {
                'S' => {
                    west_inside.insert(pos, true);
                }
                'N' => {
                    west_inside.insert(pos, false);
                }
                _ => panic!(),
            },
            '-' => {
                west_inside.insert(pos, true);
            }
            c => panic!("{c}"),
        }
    }

    // Draw a ray to the west and find the first loop segment on the same row.
    // Check if west is inside or outside of the loop.
    let mut count = 0;
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            if !west_inside.contains_key(&(row as u64, col as u64)) {
                let loop_segment = loop_pos
                    .iter()
                    .filter(|&&(r, c)| row == r as usize && c as usize > col)
                    .min();
                match loop_segment {
                    Some(c) if *west_inside.get(c).unwrap() => count += 1,
                    _ => {}
                }
            }
        }
    }

    count
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

    const EXAMPLE3: &str = "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........
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
        assert_eq!(part2(EXAMPLE3.lines().map(|v| v.to_string()), 'F'), 4);
        assert_eq!(part2(EXAMPLE4.lines().map(|v| v.to_string()), 'F'), 8);
        assert_eq!(part2(EXAMPLE5.lines().map(|v| v.to_string()), '7'), 10);
    }

    #[test]
    fn test_part2() {
        let res = part2(read_file(), 'J');
        println!("{}", res);
        assert_eq!(res, 423);
    }
}
