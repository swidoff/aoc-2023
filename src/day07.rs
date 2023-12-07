use itertools::Itertools;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day07.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap())
}

struct Hand {
    cards: Vec<u32>,
    type_rank: u32,
}

fn cmp_hand(hand1: &Hand, hand2: &Hand) -> Ordering {
    match hand1.type_rank.cmp(&hand2.type_rank) {
        Ordering::Equal => hand1
            .cards
            .iter()
            .zip(&hand2.cards)
            .find_map(|(&c1, c2)| match c1.cmp(c2) {
                Ordering::Equal => None,
                o => Some(o),
            })
            .unwrap(),
        o => o,
    }
}

fn type_rank(cards: &Vec<u32>) -> u32 {
    let mut counts = [0u32; 14];
    for &c in cards {
        counts[c as usize] += 1;
    }
    if counts[0] > 0 {
        // Jokers
        if let Some(i) = counts.iter().dropping(1).position_max() {
            counts[i + 1] += counts[0];
            counts[0] = 0
        }
    }

    let (top1, top2) = counts
        .into_iter()
        .dropping(1)
        .sorted()
        .rev()
        .take(2)
        .collect_tuple()
        .unwrap();
    match (top1, top2) {
        (5, 0) => 7,
        (4, 1) => 6,
        (3, 2) => 5,
        (3, 1) => 4,
        (2, 2) => 3,
        (2, 1) => 2,
        _ => 1,
    }
}

fn parse_input(input: impl Iterator<Item = String>, jokers: bool) -> Vec<(Hand, u64)> {
    input
        .map(|line| {
            let (cards, bid) = line.split_once(" ").unwrap();
            let cards = cards
                .chars()
                .map(|c| match c {
                    'A' => 13,
                    'K' => 12,
                    'Q' => 11,
                    'J' => {
                        if jokers {
                            0
                        } else {
                            10
                        }
                    }
                    'T' => 9,
                    d => d.to_digit(10).unwrap() - 1,
                })
                .collect_vec();

            let type_rank = type_rank(&cards);
            (Hand { cards, type_rank }, u64::from_str(bid).unwrap())
        })
        .collect_vec()
}

fn score_hands(hands: &mut Vec<(Hand, u64)>) -> u64 {
    hands
        .iter()
        .sorted_by(|(h1, _), (h2, _)| cmp_hand(h1, h2))
        .enumerate()
        .map(|(r, (_hand, bid))| (r as u64 + 1) * *bid)
        .sum()
}

fn part1(input: impl Iterator<Item = String>) -> u64 {
    score_hands(&mut parse_input(input, false))
}

fn part2(input: impl Iterator<Item = String>) -> u64 {
    score_hands(&mut parse_input(input, true))
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, read_file};

    const EXAMPLE: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE.lines().map(|v| v.to_string())), 6440);
    }

    #[test]
    fn test_part1() {
        let res = part1(read_file());
        println!("{}", res);
        assert_eq!(res, 249204891);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE.lines().map(|v| v.to_string())), 5905);
    }

    #[test]
    fn test_part2() {
        let res = part2(read_file());
        println!("{}", res);
        assert_eq!(res, 249666369);
        // 249997770 High
        // 249932626 High
    }
}
