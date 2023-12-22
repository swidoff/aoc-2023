use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

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

fn count_plots(input: impl Iterator<Item = String> + Sized, max_steps: usize) -> i64 {
    let (rocks, start, dim) = parse_input(input);
    let mut seen = HashSet::new();
    let mut q = VecDeque::new();
    q.push_back((start, 0));

    let mut steps_count = Vec::new();
    for _ in 0..=max_steps {
        steps_count.push(0);
    }

    while let Some((pos @ (row, col), steps)) = q.pop_front() {
        if steps > max_steps {
            continue;
        }
        if seen.contains(&pos) {
            continue;
        } else {
            seen.insert(pos.clone());
            steps_count[steps] += 1;
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

    steps_count
        .iter()
        .enumerate()
        .filter(|(steps, _count)| steps % 2 == max_steps % 2)
        .map(|(_steps, &count)| count)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{count_plots, read_file};

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
        // Had to get a hint on this one. I was on the way to curve fitting, but couldn't
        // find a clean way to do it. One of the nicest solutions uses this lagrange interpolation
        // equation to find the coefficients of the second order polynomial that you can see
        // when you plot the counts as a function of steps.

        // This equation takes three points on the curve. We choose x=[65, 65 + 131, 65 + 131*2],
        // because the number of steps has the property 65 = 26501365 % 131. Where 131 is the
        // dimension of the input and 65 is half that (minus the starting point).
        //
        // So 65 is the intercept and 131 and 131*2 are the points on the curve.

        // The formula is:
        // f(x) = [(x – x1)(x – x2)/(x0 – x1)(x0 – x2)]×y0
        //      + [(x – x0)(x – x2)/(x1 – x0)(x1 – x2)]×y1
        //      + [(x – x0)(x – x1)/(x2 – x0)(x2 – x1)]×y2

        // If x0 = 0, x1, = 1 and x=2 is 2, then:
        // (x – x1)(x – x2)/(x0 – x1)(x0 – x2) = (x - 1)(x - 2)/2 = (x^2 - 3x + 2)/2
        // (x – x0)(x – x2)/(x1 – x0)(x1 – x2) = x*(x - 2)/1*(1-2) = -(x^2 - 2x)
        // (x – x0)(x – x1)/(x2 – x0)(x2 – x1) = x*(x-1)/2 = (x^2 - x)/2

        // f(x) = (x^2 - 3x + 2)*y0/2 - (x^2 - 2x)*y1 + (x^2 - x)*y2/2
        //      = x^2*y0/2 - 3x*y0/2+ 2*y0/2 - x^2*y1 + 2x*y1 + x^2*y2/2 - x*y2/2
        //      = (x^2*y0/2 - x^2*y1 + x^2*y2/2) + (-3x*y0/2 + 2x*y1 - x*y2/2) + (2*y0/2)
        //      = (y0/2 - y1 + y2/2)*x^2 + (-3*y0/2 + 2*y1 - y2/2)*x + y0
        //
        // So for the formula y = a*x^2 + b*x + c
        // a = y0/2 - y1 + y2/2
        // b = -3*y0/2 + 2*y1 - y2/2
        // c = y0

        let y0 = count_plots(read_file(), 65) as f64;
        let y1 = count_plots(read_file(), 65 + 131) as f64;
        let y2 = count_plots(read_file(), 65 + 2 * 131) as f64;

        let a = y0 / 2. - y1 + y2 / 2.;
        let b = -3. * y0 / 2. + 2. * y1 - y2 / 2.;
        let c = y0;

        // We fit the curve with x=[0, 1, 2], but the x values we actually have are
        // 65, 65 + 131 and 65 + 131*2.
        //
        // We'll adjust our target x by performing the same transformation to arrive at an x in
        // the same units as our interpolation: multiples of 131 offset by 65.
        let target = (26_501_365. - 65.) / 131.;
        let res = a * target * target + b * target + c;

        println!("{}", res);
        assert_eq!(res as u64, 637087163925555);
        // 637046188465030 too low
        // 637039890461554
        // 637080865517233
        // 637046188465029.0 too low
        // 637087163925555
    }
}
