use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_file() -> String {
    let file = File::open("input/day15.txt").unwrap();
    BufReader::new(file).lines().exactly_one().unwrap().unwrap()
}

fn hash(s: &str) -> u64 {
    s.bytes().fold(0u64, |current_value, b| {
        ((current_value + b as u64) * 17) % 256
    })
}

fn part1(input: String) -> u64 {
    input.split(",").map(|v| hash(v)).sum()
}

fn part2(input: String) -> usize {
    let mut boxes: Vec<Vec<(&str, usize)>> = vec![Vec::new(); 256];

    for cmd in input.split(",") {
        match cmd.split_once(&['-', '='][..]) {
            Some((label, "")) => boxes[hash(label) as usize].retain(|(l, _)| l[..] != label[..]),
            Some((label, len_str)) => {
                let focal_len = len_str.parse::<usize>().unwrap();
                let the_box = &mut boxes[hash(label) as usize];
                if let Some((index, _)) = the_box
                    .iter()
                    .find_position(|(other_label, _)| other_label[0..] == label[0..])
                {
                    the_box[index] = (label, focal_len);
                } else {
                    the_box.push((label, focal_len));
                }
            }
            _ => panic!(),
        }
    }

    let mut res = 0;
    for (box_num, the_box) in boxes.iter().enumerate() {
        for (slot_num, &(_, focal_len)) in the_box.iter().enumerate() {
            res += (box_num + 1) * (slot_num + 1) * focal_len;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, read_file};

    const EXAMPLE1: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE1.to_string()), 1320);
    }

    #[test]
    fn test_part1() {
        let res = part1(read_file());
        println!("{}", res);
        assert_eq!(res, 510273);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(part2(EXAMPLE1.to_string()), 145);
    }

    #[test]
    fn test_part2() {
        let res = part2(read_file());
        println!("{}", res);
        assert_eq!(res, 212449);
    }
}
