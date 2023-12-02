use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day02.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap())
}

fn max_drawn_per_color(game: &String) -> [u32; 3] {
    let (_game, draws) = game.split(": ").collect_tuple().unwrap();
    draws
        .split("; ")
        .flat_map(|draw| draw.split(", "))
        .map(|pair| {
            let (num, color) = pair.split(" ").collect_tuple().unwrap();
            let num = u32::from_str(num).unwrap();
            (num, color)
        })
        .fold([0, 0, 0], |[mr, mg, mb], (num, color)| match color {
            "red" => [mr.max(num), mg, mb],
            "green" => [mr, mg.max(num), mb],
            "blue" => [mr, mg, mb.max(num)],
            _ => panic!(),
        })
}

fn part1(input: impl Iterator<Item = String>) -> usize {
    input
        .map(|s| max_drawn_per_color(&s))
        .enumerate()
        .filter_map(|(i, [max_r, max_g, max_b])| {
            if max_r <= 12 && max_g <= 13 && max_b <= 14 {
                Some(i + 1)
            } else {
                None
            }
        })
        .sum()
}

fn part2(input: impl Iterator<Item = String>) -> u32 {
    input
        .map(|s| max_drawn_per_color(&s))
        .map(|d| d.iter().sum::<u32>())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, read_file};

    const EXAMPLE1: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE1.lines().map(|v| v.to_string())), 8);
    }

    #[test]
    fn test_part1() {
        let res = part1(read_file());
        println!("{}", res);
        assert_eq!(res, 2285);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE1.lines().map(|v| v.to_string())), 2286);
    }

    #[test]
    fn test_part2() {
        let res = part2(read_file());
        println!("{}", res);
        assert_eq!(res, 77021);
    }
}
