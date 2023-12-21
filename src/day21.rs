use crate::util;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

// use std::str::FromStr;
fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day21.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap())
}

fn parse_input(input: impl Iterator<Item = String>) -> (HashSet<(i64, i64)>, (i64, i64), i64) {
    let mut rocks = HashSet::new();
    let mut start = (0, 0);
    let mut dim = 0;

    for (row, line) in input.enumerate() {
        for (col, c) in line.chars().enumerate() {
            match c {
                'S' => start = (row as i64, col as i64),
                '#' => _ = rocks.insert((row as i64, col as i64)),
                _ => {}
            }
        }
        dim = row as i64
    }

    (rocks, start, dim)
}

fn count_plots(input: impl Iterator<Item = String> + Sized, max_steps: usize) -> usize {
    let (rocks, start, dim) = parse_input(input);
    let mut seen = HashMap::new();
    let mut q = VecDeque::new();
    q.push_back((start, 0));

    while let Some((pos @ (row, col), steps)) = q.pop_front() {
        if steps > max_steps {
            continue;
        }
        if seen.contains_key(&pos) {
            continue;
        } else if steps > 0 {
            seen.insert(pos.clone(), steps);
        }

        for [delta_r, delta_c] in [[-1, 0], [0, -1], [1, 0], [0, 1]] {
            let new_row = row + delta_r;
            let new_col = col + delta_c;
            let map_row = if new_row > 0 {
                new_row % (dim + 1)
            } else {
                (dim + 1) - new_row.abs() % (dim + 1)
            };
            let map_col = if new_col > 0 {
                new_col % (dim + 1)
            } else {
                (dim + 1) - new_col.abs() % (dim + 1)
            };
            // println!("{new_row} {new_col}: {map_row} {map_col}");
            if !rocks.contains(&(map_row, map_col)) {
                q.push_back(((new_row, new_col), steps + 1));
            }
        }
    }

    // for r in 0..=dim {
    //     for c in 0..=dim {
    //         if let Some(&steps) = seen.get(&(r, c)) {
    //             if steps % 2 == 0 {
    //                 print!("O");
    //                 continue;
    //             }
    //         }
    //         if rocks.contains(&(r, c)) {
    //             print!("#");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!();
    // }

    seen.values().filter(|&&steps| steps % 2 == 0).count()
}

fn part1(input: impl Iterator<Item = String>, max_steps: usize) -> usize {
    count_plots(input, max_steps)
}

fn part2(_input: impl Iterator<Item = String>) -> u64 {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::{count_plots, part1, part2, read_file};

    const EXAMPLE1: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
";

    #[test]
    fn test_part1_example() {
        assert_eq!(count_plots(EXAMPLE1.lines().map(|v| v.to_string()), 6), 16);
    }

    #[test]
    fn test_part1() {
        let res = count_plots(read_file(), 64);
        println!("{}", res);
        assert_eq!(res, 3830);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(count_plots(EXAMPLE1.lines().map(|v| v.to_string()), 10), 50);
        assert_eq!(
            count_plots(EXAMPLE1.lines().map(|v| v.to_string()), 50),
            1594
        );
        assert_eq!(
            count_plots(EXAMPLE1.lines().map(|v| v.to_string()), 100),
            6536
        );
        assert_eq!(
            count_plots(EXAMPLE1.lines().map(|v| v.to_string()), 500),
            167004
        );
        assert_eq!(
            count_plots(EXAMPLE1.lines().map(|v| v.to_string()), 1000),
            668697
        );
        assert_eq!(
            count_plots(EXAMPLE1.lines().map(|v| v.to_string()), 5000),
            16733044
        );
    }

    #[test]
    fn test_part2() {
        let res = part2(read_file());
        println!("{}", res);
        // assert_eq!(res, 0);
    }
}
