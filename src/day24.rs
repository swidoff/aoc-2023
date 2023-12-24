use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

// use std::str::FromStr;
fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day24.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap())
}

fn parse_input(input: impl Iterator<Item = String>) -> Vec<Vec<f64>> {
    input
        .map(|line| {
            line.split(&[',', '@'])
                .map(|s| s.parse::<i64>().unwrap() as f64)
                .collect_vec()
        })
        .collect_vec()
}

fn lines_intersect_2d(l1: &Vec<f64>, l2: &Vec<f64>, min_pos: f64, max_pos: f64) -> bool {
    // y = mx - (m*x1-y1), a = m, b = m*x1-y1
    let x1 = l1[0];
    let y1 = l1[1];
    let run1 = l1[3];
    let rise1 = l1[4];
    let m1 = rise1 / run1;
    let a = m1;
    let c = y1 - m1 * x1;

    let x2 = l2[0];
    let y2 = l2[1];
    let run2 = l2[3];
    let rise2 = l2[4];
    let m2 = rise2 / run2;
    let b = m2;
    let d = y2 - m2 * x2;

    if a == b {
        false
    } else {
        let x = (d - c) / (a - b);
        let y = a * x + c;
        min_pos <= x
            && x <= max_pos
            && min_pos <= y
            && y <= max_pos
            && (x - x1).signum() == run1.signum()
            && (x - x1).abs() >= run1.abs()
            && (x - x2).signum() == run2.signum()
            && (x - x2).abs() >= run2.abs()
            && (y - y1).signum() == rise1.signum()
            && (y - y1).abs() >= rise1.abs()
            && (y - y2).signum() == rise2.signum()
            && (y - y2).abs() >= rise2.abs()
    }
}

fn part1(input: impl Iterator<Item = String>, min_pos: f64, max_pos: f64) -> u64 {
    let lines = parse_input(input);
    let mut count = 0;
    for i in 0..lines.len() {
        let l1 = &lines[i];
        for j in (i + 1)..lines.len() {
            let l2 = &lines[j];
            if lines_intersect_2d(l1, l2, min_pos, max_pos) {
                count += 1;
            }
        }
    }
    count
}

fn part2(_input: impl Iterator<Item = String>) -> u64 {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, read_file};

    const EXAMPLE1: &str = "19,13,30@-2,1,-2
18,19,22@-1,-1,-2
20,25,34@-2,-2,-4
12,31,28@-1,-2,-1
20,19,15@1,-5,-3
";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE1.lines().map(|v| v.to_string()), 7., 27.), 2);
    }

    #[test]
    fn test_part1() {
        let res = part1(read_file(), 200000000000000., 400000000000000.);
        println!("{}", res);
        // assert_eq!(res, 0);
    }

    const EXAMPLE2: &str = "
";

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE2.lines().map(|v| v.to_string())), 0);
    }

    #[test]
    fn test_part2() {
        let res = part2(read_file());
        println!("{}", res);
        // assert_eq!(res, 0);
    }
}
