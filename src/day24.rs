use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;
use ndarray::prelude::*;
use ndarray_linalg::Solve;

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

fn part2(input: impl Iterator<Item = String>, i: usize, j: usize, k: usize) -> f64 {
    let hail = parse_input(input);
    let (a0x, a0y, a0z, avx, avy, avz) = (
        hail[i][0], hail[i][1], hail[i][2], hail[i][3], hail[i][4], hail[i][5],
    );
    let (b0x, b0y, b0z, bvx, bvy, bvz) = (
        hail[j][0], hail[j][1], hail[j][2], hail[j][3], hail[j][4], hail[j][5],
    );
    let (c0x, c0y, c0z, cvx, cvy, cvz) = (
        hail[k][0], hail[k][1], hail[k][2], hail[k][3], hail[k][4], hail[k][5],
    );

    /*
    a0x + avx*t = Px + Qx*t
    a0y + avy*t = Py + Qy*t
    a0z + avz*t = Pz + Qz*t

    avx*t - Qx*t = Px - a0x
    t = (Px - a0x) / (avx - Qx)
    t = (Py - a0y) / (avy - Qy)
    t = (Pz - a0z) / (avz - Qz)

    (Px - a0x) / (avx - Qx) = (Py - a0y) / (avy - Qy) = (Pz - a0z) / (avz - Qz)
    (Px - b0x) / (bvx - Qx) =  (Py - b0y) / (bvy - Qy) = (Pz - b0z) / (bvz - Qz)
    (Px - c0x) / (cvx - Qx) =  (Py - c0y) / (cvy - Qy) = (Pz - c0z) / (cvz - Qz)

    (Px - a0x) * (avy - Qy) = (Py - a0y) * (avx - Qx)
    Px*avy - Px*Qy - a0x*vy + a0x*Qy = Py*avx - Py*Qx - a0y*avx + a0y*Qx

    Px*Qy - Py*Qx = Px*avy - Py*avx + a0y*avx - a0x*avy + a0x*Qy - a0y*Qx
    Px*Qy - Py*Qx = Px*bvy - Py*bvx + a0y*bvx - a0x*bvy + b0x*Qy - b0y*Qx
    Px*Qy - Py*Qx = Px*cvy - Py*cvx + a0y*cvx - a0x*cvy + c0x*Qy - c0y*Qx

    Pz*Qx - Px*Qx = Pz*avx - Px*avz + a0x*avz - a0z*avx + a0z*Qz - a0y*Qy
    Pz*Qx - Px*Qx = Pz*bvx - Px*bvz + a0x*bvz - a0z*bvx + b0z*Qz - b0y*Qy
    Pz*Qx - Px*Qx = Pz*cvx - Px*cvz + a0x*cvz - a0z*cvx + c0z*Qz - c0y*Qy

    Py*Qz - Pz*Qy = Py*avz - Pz*avy + a0z*avy - a0y*avz + a0y*Qz - a0z*Qy
    Py*Qz - Pz*Qy = Py*bvz - Pz*bvy + a0z*bvy - a0y*bvz + b0y*Qz - b0z*Qy
    Py*Qz - Pz*Qy = Py*cvz - Pz*cvy + a0z*cvy - a0y*cvz + c0y*Qz - c0z*Qy

    (avy - bvy)*Px + (avx - bvx)*Py +                + (a0y - b0y)*Qx + (a0x - b0x)*Qy                  = (b0y*bvx - b0x*bvy) + (a0y*avx - a0x*avy)
    (avy - cvy)*Px + (avx - cvx)*Py +                + (a0y - c0y)*Qx + (a0x - c0x)*Qy                  = (c0y*cvx - c0x*cvy) + (a0y*avx - a0x*avy)
    (avz - bvz)*Px +                + (avx - bvx)*Pz + (a0z - b0z)*Qx +                + (a0x - b0x)*Qz = (b0x*bvz - b0z*bvx) + (a0x*avz - a0y*avz)
    (avz - cvz)*Px +                + (avx - cvx)*Pz + (a0z - b0z)*Qx +                + (a0x - c0x)*Qz = (c0x*cvz - c0z*cvx) + (a0x*avz - a0y*avz)
                   + (avz - bvz)*Py + (avy - bvy)*Pz +                + (a0z - b0z)*Qy + (a0y - c0y)*Qz = (b0z*bvy - b0y*bvz) + (a0z*avy - a0y*avz)
                   + (avz - cvz)*Py + (avy - cvy)*Pz +                + (a0z - c0z)*Qy + (a0y - c0y)*Qz = (c0z*cvy - c0y*cvz) + (a0z*avy - a0y*avz)
     */
    let a = array![
        [avy - bvy, bvx - avx, 0.0, b0y - a0y, a0x - b0x, 0.0],
        [avy - cvy, cvx - avx, 0.0, c0y - a0y, a0x - c0x, 0.0],
        [bvz - avz, 0.0, avx - bvx, a0z - b0z, 0.0, b0x - a0x],
        [cvz - avz, 0.0, avx - cvx, a0z - c0z, 0.0, c0x - a0x],
        [0.0, avz - bvz, bvy - avy, 0.0, b0z - a0z, a0y - b0y],
        [0.0, avz - cvz, cvy - avy, 0.0, c0z - a0z, a0y - c0y],
    ];
    let b = array![
        (b0y * bvx - b0x * bvy) - (a0y * avx - a0x * avy),
        (c0y * cvx - c0x * cvy) - (a0y * avx - a0x * avy),
        (b0x * bvz - b0z * bvx) - (a0x * avz - a0z * avx),
        (c0x * cvz - c0z * cvx) - (a0x * avz - a0z * avx),
        (b0z * bvy - b0y * bvz) - (a0z * avy - a0y * avz),
        (c0z * cvy - c0y * cvz) - (a0z * avy - a0y * avz),
    ];
    let res = a.solve(&b).unwrap();
    println!("{res}");
    res[0] + res[1] + res[2]
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
        assert_eq!(res, 16050);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE1.lines().map(|v| v.to_string()), 0, 1, 2), 47.);
    }

    #[test]
    fn test_part2() {
        let res = part2(read_file(), 1, 10, 20);
        println!("{}", res);
        assert_eq!(res, 669042940632377.);
        // 669042940632372
        // 669042940632377
    }
}
