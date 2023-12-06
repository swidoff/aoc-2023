use std::fs::File;
use std::io::{BufRead, BufReader};

fn part1(races: &Vec<(u64, u64)>) -> usize {
    let mut res = 1;
    for &(time, distance) in races.iter() {
        let mut ways = 0;
        for t in 1..time {
            let final_distance = t * (time - t);
            if final_distance > distance {
                ways += 1;
            }
        }
        if ways > 0 {
            res *= ways;
        }
    }
    res
}

fn part2(time: u64, distance: u64) -> u64 {
    let mut ways = 0;
    for t in 1..time {
        let final_distance = t * (time - t);
        if final_distance > distance {
            ways += 1;
        }
    }
    ways
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

    const EXAMPLE2: &str = "
";

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
