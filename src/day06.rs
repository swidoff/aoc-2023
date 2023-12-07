fn part1(races: &Vec<(u64, u64)>) -> u64 {
    let mut res = 1;
    for &(time, distance) in races.iter() {
        let ways = solve_race(time, distance);
        if ways > 0 {
            res *= ways;
        }
    }
    res
}

fn part2(time: u64, distance: u64) -> u64 {
    return solve_race(time, distance);
}

fn solve_race(time: u64, distance: u64) -> u64 {
    // Solve the inequality for t: t*(time - t) > d => -t^2 + time*t - d > 0
    // Use quadratic equation to find the roots. The difference between the roots is the
    // region where the inequality is true. If the roots are integers, those are the points where
    // the equation exactly equals zero, so we need to nudge them a bit in the right direction.
    let t = time as f64;
    let d = distance as f64;
    let mut r1 = (t - (t.powf(2.) - 4. * d).sqrt()) / 2.;
    let mut r2 = (t + (t.powf(2.) - 4. * d).sqrt()) / 2.;
    if r1.ceil() == r1 {
        r1 += 0.01;
    }
    if r2.floor() == r2 {
        r2 -= 0.01;
    }
    (r2.floor() - r1.ceil() + 1.) as u64
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn test_part1_example() {
        let races = vec![(7, 9), (15, 40), (30, 200)];
        assert_eq!(part1(&races), 288);
    }

    #[test]
    fn test_part1() {
        // Time:        53     89     76     98
        // Distance:   313   1090   1214   1201
        let input = vec![(53, 313), (89, 1090), (76, 1214), (98, 1201)];
        let res = part1(&input);
        println!("{}", res);
        assert_eq!(res, 5133600);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(71530, 940200), 71503);
    }

    #[test]
    fn test_part2() {
        let res = part2(53897698, 313109012141201);
        println!("{}", res);
        assert_eq!(res, 40651271);
    }
}
