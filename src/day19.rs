use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

// use std::str::FromStr;
fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day19.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap())
}

const X: usize = 0;
const M: usize = 1;
const A: usize = 2;
const S: usize = 3;

enum Destination {
    Workflow(String),
    Accept,
    Reject,
}

enum Op {
    Lt,
    Gt,
}

struct Workflow {
    rules: Vec<(usize, Op, u64, Destination)>,
    otherwise: Destination,
}

fn parse_input(input: impl Iterator<Item = String>) -> (HashMap<String, Workflow>, Vec<[u64; 4]>) {
    let mut workflows = HashMap::new();
    let mut parts = Vec::new();

    let mut parse_workflows = true;
    for line in input {
        if line.is_empty() {
            parse_workflows = false;
            continue;
        }
        if parse_workflows {
            let mut rules = Vec::new();
            let mut otherwise = Destination::Accept;
            let (name, rest) = line.split_once("{").unwrap();
            for rule in rest[..rest.len() - 1].split(",") {
                if let Some((condition, dest)) = rule.split_once(":") {
                    let prop = match &condition[0..1] {
                        "m" => M,
                        "s" => S,
                        "a" => A,
                        "x" => X,
                        _ => panic!(),
                    };
                    let op = match &condition[1..2] {
                        ">" => Op::Gt,
                        "<" => Op::Lt,
                        _ => panic!(),
                    };
                    let value = condition[2..].parse::<u64>().unwrap();
                    let dest = match dest {
                        "A" => Destination::Accept,
                        "R" => Destination::Reject,
                        x => Destination::Workflow(x.to_string()),
                    };
                    rules.push((prop, op, value, dest))
                } else {
                    otherwise = match rule {
                        "A" => Destination::Accept,
                        "R" => Destination::Reject,
                        x => Destination::Workflow(x.to_string()),
                    };
                }
            }
            workflows.insert(name.to_string(), Workflow { rules, otherwise });
        } else {
            let mut part = [0u64; 4];
            for (i, prop) in line[1..line.len() - 1].split(",").enumerate() {
                let (_, value) = prop.split_once("=").unwrap();
                part[i] = value.parse::<u64>().unwrap();
            }
            parts.push(part);
        }
    }
    (workflows, parts)
}

fn is_accepted(workflows: &HashMap<String, Workflow>, part: &[u64; 4]) -> bool {
    let mut workflow_name = &"in".to_string();
    loop {
        let mut workflow_dest = None;
        let workflow = workflows.get(workflow_name).unwrap();
        for (prop, op, value, dest) in &workflow.rules {
            if match op {
                Op::Gt => part[*prop] > *value,
                Op::Lt => part[*prop] < *value,
            } {
                workflow_dest = Some(dest);
                break;
            }
        }
        if workflow_dest.is_none() {
            workflow_dest = Some(&workflow.otherwise);
        }
        match workflow_dest.unwrap() {
            Destination::Accept => return true,
            Destination::Reject => return false,
            Destination::Workflow(name) => workflow_name = name,
        }
    }
}

fn part1(input: impl Iterator<Item = String>) -> u64 {
    let (workflows, parts) = parse_input(input);
    parts
        .into_iter()
        .filter(|p| is_accepted(&workflows, p))
        .map(|p| p.iter().sum::<u64>())
        .sum()
}

fn count_combinations(
    dest: &Destination,
    workflows: &HashMap<String, Workflow>,
    ranges: [[u64; 2]; 4],
) -> u64 {
    let mut combinations = 0;
    let workflow_name;

    match dest {
        Destination::Accept => return ranges.iter().map(|[b, e]| e - b + 1).product::<u64>(),
        Destination::Reject => return 0,
        Destination::Workflow(name) => workflow_name = name,
    }

    let mut new_ranges = ranges.clone();
    let workflow = workflows.get(workflow_name).unwrap();

    for (prop, op, value, dest) in &workflow.rules {
        let mut next_ranges = new_ranges.clone();
        match op {
            Op::Gt => {
                next_ranges[*prop][0] = *value + 1;
                new_ranges[*prop][1] = *value;
            }
            Op::Lt => {
                next_ranges[*prop][1] = *value - 1;
                new_ranges[*prop][0] = *value;
            }
        }
        combinations += count_combinations(dest, workflows, next_ranges)
    }
    combinations + count_combinations(&workflow.otherwise, workflows, new_ranges)
}

fn part2(input: impl Iterator<Item = String>) -> u64 {
    let (workflows, _) = parse_input(input);
    count_combinations(
        &Destination::Workflow("in".to_string()),
        &workflows,
        [[1, 4000]; 4],
    )
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, read_file};

    const EXAMPLE1: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE1.lines().map(|v| v.to_string())), 19114);
    }

    #[test]
    fn test_part1() {
        let res = part1(read_file());
        println!("{}", res);
        assert_eq!(res, 263678);
        // 348875
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(
            part2(EXAMPLE1.lines().map(|v| v.to_string())),
            167409079868000
        );
    }

    #[test]
    fn test_part2() {
        let res = part2(read_file());
        println!("{}", res);
        assert_eq!(res, 125455345557345);
    }
}
