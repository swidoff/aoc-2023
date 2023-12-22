use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

// use std::str::FromStr;
fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day22.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap())
}

fn parse_input(input: impl Iterator<Item = String>) -> Vec<[usize; 6]> {
    input
        .map(|line| {
            let (x1, y1, z1, x2, y2, z2) = line
                .split(&[',', '~'][..])
                .map(|s| s.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap();
            [x1, x2, y1, y2, z1, z2]
        })
        .collect_vec()
}

fn support_graph(pieces: &mut Vec<[usize; 6]>) -> (Vec<Vec<usize>>, Vec<usize>) {
    pieces.sort_by_key(|v| v[4]);

    let mut max_z = 1;
    let mut dropped: Vec<(usize, [usize; 6])> = Vec::new();
    let mut supporting = Vec::new();
    for _ in 0..pieces.len() {
        supporting.push(Vec::new())
    }

    for (i, &[x1, x2, y1, y2, z1, z2]) in pieces.iter().enumerate() {
        let mut new_z = 0;
        for z in (1..=max_z).rev() {
            let intersecting_pieces = dropped
                .iter()
                .filter_map(|&(j, [ox1, ox2, oy1, oy2, oz1, oz2])| {
                    if x1.max(ox1) <= x2.min(ox2)
                        && y1.max(oy1) <= y2.min(oy2)
                        && z >= oz1
                        && z <= oz2
                    {
                        Some(j)
                    } else {
                        None
                    }
                })
                .collect_vec();

            if !intersecting_pieces.is_empty() {
                new_z = z + 1;
                supporting[i].extend(intersecting_pieces);
                break;
            }
            new_z = z;
        }
        let z_len = z2 - z1;
        dropped.push((i, [x1, x2, y1, y2, new_z, new_z + z_len]));
        max_z = max_z.max(new_z + z_len);
    }

    let required = (0..pieces.len())
        .filter(|&i| !supporting.iter().any(|v| v.len() == 1 && v[0] == i))
        .collect_vec();

    (supporting, required)
}

fn part1(input: impl Iterator<Item = String>) -> usize {
    let mut pieces = parse_input(input);
    let (_, required) = support_graph(&mut pieces);
    required.len()
}

fn falling_count(target: usize, supporting: &Vec<Vec<usize>>) -> usize {
    let mut falling = HashSet::new();
    falling.insert(target);

    for _ in 0..supporting.len() {
        let new_falling: HashSet<usize> = supporting
            .iter()
            .enumerate()
            .filter_map(|(i, d)| {
                if !d.is_empty() && d.iter().all(|j| falling.contains(j)) {
                    Some(i)
                } else {
                    None
                }
            })
            .collect();
        falling.extend(new_falling);
    }
    falling.len() - 1
}

fn part2(input: impl Iterator<Item = String>) -> usize {
    let mut pieces = parse_input(input);
    let (supporting, required) = support_graph(&mut pieces);

    let mut res = 0;
    for i in 0..pieces.len() {
        if !required.contains(&i) {
            res += falling_count(i, &supporting);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, read_file};

    const EXAMPLE1: &str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE1.lines().map(|v| v.to_string())), 5);
    }

    #[test]
    fn test_part1() {
        let res = part1(read_file());
        println!("{}", res);
        assert_eq!(res, 430);
        // 435 too high
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE1.lines().map(|v| v.to_string())), 7);
    }

    #[test]
    fn test_part2() {
        let res = part2(read_file());
        println!("{}", res);
        // assert_eq!(res, 0);
        //82782 too high
        //60558 too high
    }
}
