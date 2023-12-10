use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
// use std::str::FromStr;
fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day10.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap())
}

type Coord = (usize, usize);

struct Map {
    start: Coord,
    grid: HashMap<Coord, [Coord; 2]>,
    rows: usize,
    cols: usize,
    points: Vec<Coord>,
}

fn parse_input(input: impl Iterator<Item = String>) -> Map {
    let mut start = None;
    let mut grid = HashMap::new();
    let mut points = Vec::new();

    for (row, line) in input.enumerate() {
        let row = row + 1;
        for (column, c) in line.chars().enumerate() {
            let column = column + 1;
            let pos = (row, column);
            if c == 'S' {
                start = Some(pos);
            } else if c == '.' {
                points.push(pos);
            } else if let Some(target) = targets_for_pipe(c, row, column) {
                grid.insert(pos, target);
            }
        }
    }

    let start = start.unwrap();
    if let Some(start_target) = ['|', '-', 'L', 'J', '7', 'F']
        .into_iter()
        .filter_map(|c| targets_for_pipe(c, start.0, start.1))
        .find(|targets| targets.iter().all(|t| grid.contains_key(t)))
    {
        grid.insert(start, start_target);
    }

    let rows = grid.keys().map(|&(rows, _)| rows).max().unwrap();
    let cols = grid.keys().map(|&(_, cols)| cols).max().unwrap();
    Map { start, grid, rows, cols, points }
}

fn targets_for_pipe(c: char, row: usize, column: usize) -> Option<[Coord; 2]> {
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

fn find_loop(map: &Map) -> HashSet<Coord> {
    let grid = &map.grid;
    let start = map.start;
    let mut q = VecDeque::new();
    let mut seen = HashSet::new();

    q.push_back(start);

    while let Some(pos) = q.pop_front() {
        seen.insert(pos);

        if let Some(targets) = grid.get(&pos) {
            for &t in targets {
                if grid.contains_key(&t) && !seen.contains(&t) {
                    q.push_back(t);
                }
            }
        }
    }
    seen
}

fn part1(input: impl Iterator<Item = String>) -> u64 {
    let map = parse_input(input);
    find_loop(&map).len() as u64 / 2
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
        assert_eq!(part1(EXAMPLE1.lines().map(|v| v.to_string())), 4);
        assert_eq!(part1(EXAMPLE2.lines().map(|v| v.to_string())), 8);
    }

    #[test]
    fn test_part1() {
        let res = part1(read_file());
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
