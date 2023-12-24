use itertools::Itertools;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
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

fn to_graph(grid: &Vec<Vec<char>>) -> HashMap<(i32, i32), Vec<((i32, i32), usize)>> {
    // Convert the difficult-to-work-with grid into a graph. Returns a map of edges
    // (source -> destination) with associated number of steps
    let dim = grid.len() as i32;
    let mut seen = HashSet::new();
    let mut q = VecDeque::new();
    let mut res: HashMap<(i32, i32), Vec<((i32, i32), usize)>> = HashMap::new();
    let dir = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    q.push_back(((0, 1), (1, 1)));

    while let Some(key @ (intersection, start)) = q.pop_front() {
        if seen.contains(&key) {
            continue;
        }
        seen.insert(key);

        let mut steps = 1;
        let mut p = start;
        let mut prior_p = intersection;
        let mut next_steps;
        loop {
            next_steps = Vec::new();
            for (row_delta, col_delta) in dir {
                let new_r = p.0 + row_delta;
                let new_c = p.1 + col_delta;
                let new_p = (new_r, new_c);
                if new_r >= 0
                    && new_r < dim
                    && new_c >= 0
                    && new_c < dim
                    && grid[new_r as usize][new_c as usize] != '#'
                    && new_p != prior_p
                {
                    next_steps.push(new_p);
                }
            }
            if next_steps.len() != 1 {
                break;
            } else {
                steps += 1;
                prior_p = p;
                p = next_steps[0];
            }
        }

        if let Some(v) = res.get_mut(&intersection) {
            v.push((p, steps));
        } else {
            res.insert(intersection, vec![(p, steps)]);
        }
        for next in next_steps {
            q.push_back((p, next))
        }
    }
    res
}

fn longest_walk2(
    graph: &HashMap<(i32, i32), Vec<((i32, i32), usize)>>,
    start: (i32, i32),
    end: (i32, i32),
    path: &mut HashSet<(i32, i32)>,
) -> Option<usize> {
    // With a map of edges, it's now much more efficient to do a simple DFS and find the result.
    if path.contains(&start) {
        return None;
    }
    if start == end {
        return Some(0);
    }

    let mut res = None;
    path.insert(start);
    for &(next, dist) in graph.get(&start).unwrap() {
        if let Some(new_steps) = longest_walk2(graph, next, end, path) {
            res = Some(res.unwrap_or(0).max(dist + new_steps))
        }
    }
    path.remove(&start);
    res
}

fn part2(input: impl Iterator<Item = String>) -> usize {
    let grid = parse_input(input);
    let graph = to_graph(&grid);
    let dim = grid.len() as i32;

    // Print graph in grapviz dot notation.
    // println!("digraph G {{");
    // for ((r1, c1), edges) in graph.iter() {
    //     for ((r2, c2), steps) in edges {
    //         println!("\tn_{r1}_{c1} -> n_{r2}_{c2} [label={steps}]")
    //     }
    // }
    // println!("}}");
    //
    let mut path = HashSet::new();
    longest_walk2(&graph, (0, 1), (dim - 1, dim - 2), &mut path).unwrap()
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
        assert_eq!(res, 6802);
        // 5242 too low
    }
}
