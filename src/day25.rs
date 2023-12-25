use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;
use rustworkx_core::petgraph;

// use std::str::FromStr;
fn read_file() -> impl Iterator<Item = String> {
    let file = File::open("input/day25.txt").unwrap();
    BufReader::new(file).lines().map(|s| s.unwrap())
}

fn parse_input(input: impl Iterator<Item = String>) -> HashMap<String, Vec<String>> {
    let mut res = HashMap::new();
    for line in input {
        let (lhs, rhs) = line.split_once(": ").unwrap();
        let rhs = rhs.split_whitespace().map(|s| s.to_string()).collect_vec();
        res.insert(lhs.to_string(), rhs);
    }
    res
}

fn part1(input: impl Iterator<Item = String>) -> u64 {
    let graph = parse_input(input);
    // let mut edges = Vec::new();
    // for (rhs, lhs) in graph {
    //     for n in lhs {
    //         edges.push((rhs.clone(), n.clone()));
    //     }
    // }

    println!("graph G {{");
    let mut counter = 0;
    for (rhs, lhs) in graph {
        for n in lhs {
            // println!("{rhs} -- {n} [label={rhs}_{n}]");
            println!("g.add_edge('{rhs}', '{n}')");
            counter += 1;
        }
    }
    println!("}}");
    0
}

#[cfg(test)]
mod tests {
    use super::{part1, read_file};

    const EXAMPLE1: &str = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr
";

    #[test]
    fn test_part1_example() {
        assert_eq!(part1(EXAMPLE1.lines().map(|v| v.to_string())), 0);
    }

    #[test]
    fn test_part1() {
        let res = part1(read_file());
        println!("{}", res);
        // assert_eq!(res, 0);
    }
}
