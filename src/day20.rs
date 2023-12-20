use crate::day20::Module::{Broadcaster, Conjunction};
use crate::util;
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day20.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap())
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Pulse {
    Low = 0,
    High = 1,
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum Module {
    Broadcaster,
    FlipFlip { on: bool },
    Conjunction { inputs: HashMap<String, bool> },
}

fn parse_input(
    input: impl Iterator<Item = String>,
) -> (HashMap<String, Vec<String>>, HashMap<String, Module>) {
    let mut connections = HashMap::new();
    let mut modules = HashMap::new();
    for line in input {
        let (module, dest) = line.split_once(" -> ").unwrap();
        let (module, module_name) = match &module[0..1] {
            "%" => (Module::FlipFlip { on: false }, module[1..].to_string()),
            "&" => (
                Conjunction { inputs: HashMap::new() },
                module[1..].to_string(),
            ),
            _ => (Broadcaster, module.to_string()),
        };
        let dest = dest.split(", ").map(|s| s.to_string()).collect_vec();
        connections.insert(module_name.clone(), dest);
        modules.insert(module_name, module);
    }
    for (source, destinations) in connections.iter() {
        for dest in destinations {
            if let Some(Conjunction { inputs, .. }) = modules.get_mut(dest) {
                inputs.insert(source.clone(), false);
            }
        }
    }

    (connections, modules)
}

fn push_button(
    connections: &HashMap<String, Vec<String>>,
    modules: &HashMap<String, Module>,
) -> (
    HashMap<String, u64>,
    HashMap<String, u64>,
    HashMap<String, Module>,
) {
    let mut low_count = HashMap::new();
    let mut high_count = HashMap::new();
    let mut new_modules = modules.clone();

    let button = "button".to_string();
    let broadcaster = "broadcaster".to_string();
    let mut q = VecDeque::new();
    q.push_back((&broadcaster, Pulse::Low, &button));

    while let Some((name, pulse, from)) = q.pop_front() {
        if pulse == Pulse::High {
            high_count.insert(name.clone(), high_count.get(name).unwrap_or(&0) + 1);
        } else {
            low_count.insert(name.clone(), low_count.get(name).unwrap_or(&0) + 1);
        }
        let new_pulse = match new_modules.get_mut(name) {
            Some(Broadcaster) => Some(pulse),
            Some(Module::FlipFlip { on }) => {
                if let Pulse::Low = pulse {
                    if *on {
                        *on = false;
                        Some(Pulse::Low)
                    } else {
                        *on = true;
                        Some(Pulse::High)
                    }
                } else {
                    None
                }
            }
            Some(Conjunction { inputs }) => {
                inputs.insert(from.clone(), pulse == Pulse::High);
                if inputs.values().all(|&b| b) {
                    Some(Pulse::Low)
                } else {
                    Some(Pulse::High)
                }
            }
            None => None,
        };

        if let Some(new_pulse) = new_pulse {
            for dest in connections.get(name).unwrap() {
                q.push_back((dest, new_pulse, name));
            }
        }
    }

    (low_count, high_count, new_modules)
}

fn part1(input: impl Iterator<Item = String>) -> u64 {
    let (connections, modules) = parse_input(input);
    let mut modules = modules;
    let mut low_count = 0;
    let mut high_count = 0;
    for _ in 0..1000 {
        let (l, h, m) = push_button(&connections, &modules);
        modules = m;
        low_count += l.values().sum::<u64>();
        high_count += h.values().sum::<u64>();
    }
    low_count * high_count
}

fn count_until_single_low(
    connections: &HashMap<String, Vec<String>>,
    modules: &HashMap<String, Module>,
    target: &str,
) -> Option<u64> {
    let target = target.to_string();
    let mut modules = modules.clone();
    for i in 1..=10_000 {
        let (l, _h, m) = push_button(&connections, &modules);
        if let Some(1) = l.get(&target) {
            return Some(i);
        }
        modules = m;
    }
    None
}

fn part2(input: impl Iterator<Item = String>) -> u64 {
    // By inspection, four sub-graphs are connected by a conjunction.
    // When all fire a high signal at once, then rx will get low signal.
    // Luckily, they all fire a high signal in a repeating cycle.
    let (connections, modules) = parse_input(input);
    let cycle1 = count_until_single_low(&connections, &modules, "rr").unwrap();
    let cycle2 = count_until_single_low(&connections, &modules, "js").unwrap();
    let cycle3 = count_until_single_low(&connections, &modules, "bs").unwrap();
    let cycle4 = count_until_single_low(&connections, &modules, "zb").unwrap();
    util::lcm(cycle1, util::lcm(cycle2, util::lcm(cycle3, cycle4)))
}

#[cfg(test)]
mod tests {
    use super::{part1, part2, read_file};

    const EXAMPLE1: &str = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
";

    const EXAMPLE2: &str = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE1.lines().map(|v| v.to_string())), 32000000);
        assert_eq!(part1(EXAMPLE2.lines().map(|v| v.to_string())), 11687500);
    }

    #[test]
    fn test_part1() {
        let res = part1(read_file());
        println!("{}", res);
        assert_eq!(res, 684125385);
    }

    #[test]
    fn test_part2() {
        let res = part2(read_file());
        println!("{}", res);
        assert_eq!(res, 225872806380073);
    }
}
