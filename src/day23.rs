use itertools::Itertools;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

// use std::str::FromStr;
fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day23.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap())
}

fn parse_input(input: impl Iterator<Item = String>) -> Vec<Vec<char>> {
    input.map(|line| line.chars().collect_vec()).collect_vec()
}

fn longest_walk(grid: Vec<Vec<char>>, ignore_slopes: bool) -> u64 {
    let dim = grid.len() as i32;
    let target = (dim - 1, dim - 2);
    let mut seen = HashMap::new();
    let mut q = BinaryHeap::new();
    let mut paths = HashMap::new();
    let mut counter = 0;

    q.push((0, (0i32, 1i32), counter));
    paths.insert(counter, HashSet::new());

    while let Some((steps, p @ (r, c), id)) = q.pop() {
        if let Some(&max_steps) = seen.get(&p) {
            if max_steps >= steps {
                continue;
            }
        }
        seen.insert(p, steps);

        let ch = grid[r as usize][c as usize];
        let mut path = paths.remove(&id).unwrap();
        path.insert(p);

        for (row_delta, col_delta, dir) in [(1, 0, 'v'), (0, 1, '>'), (-1, 0, '^'), (0, -1, '<')] {
            if !ignore_slopes && ch != '.' && ch != dir {
                continue;
            }

            let new_r = r + row_delta;
            let new_c = c + col_delta;
            if new_r >= 0
                && new_r < dim
                && new_c >= 0
                && new_c < dim
                && !path.contains(&(new_r, new_c))
                && grid[new_r as usize][new_c as usize] != '#'
            {
                counter += 1;
                paths.insert(counter, path.clone());
                q.push((steps + 1, (new_r, new_c), counter));
            }
        }
    }

    *seen.get(&target).unwrap()
}

fn part1(input: impl Iterator<Item = String>) -> u64 {
    let grid = parse_input(input);
    longest_walk(grid, false)
}

fn longest_walk2(
    grid: &Vec<Vec<char>>,
    r: i32,
    c: i32,
    path: &mut HashSet<(i32, i32)>,
) -> Option<usize> {
    let dim = grid.len() as i32;
    if (r, c) == (dim - 1, dim - 2) {
        return Some(path.len());
    }

    let dirs = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    for (row_delta, col_delta) in dirs {
        let new_r = r + row_delta;
        let new_c = c + col_delta;
        if new_r >= 0
            && new_r < dim
            && new_c >= 0
            && new_c < dim
            && !path.contains(&(new_r, new_c))
            && grid[new_r as usize][new_c as usize] != '#'
        {
            path.insert((new_r, new_c));
            let res = longest_walk2(grid, new_r, new_c, path);
            if res.is_some() {
                return res;
            } else {
                path.remove(&(new_r, new_c));
            }
        }
    }
    None
}

fn part2(input: impl Iterator<Item = String>) -> usize {
    let grid = parse_input(input);
    let mut path = HashSet::new();
    longest_walk2(&grid, 0, 1, &mut path).unwrap()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, read_file};

    const EXAMPLE1: &str = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#
";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE1.lines().map(|v| v.to_string())), 94);
    }

    #[test]
    fn test_part1() {
        let res = part1(read_file());
        // println!("{}", res);
        assert_eq!(res, 2186);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE1.lines().map(|v| v.to_string())), 154);
    }

    #[test]
    fn test_part2() {
        let res = part2(read_file());
        println!("{}", res);
        // assert_eq!(res, 0);
    }
}
