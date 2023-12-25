use rustworkx_core::connectivity::stoer_wagner_min_cut;
use rustworkx_core::petgraph::graph::{NodeIndex, UnGraph};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;

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

fn node_for_label(
    rhs: &String,
    g: &mut UnGraph<(), ()>,
    nodes: &mut HashMap<String, NodeIndex>,
) -> NodeIndex {
    if let Some(&node) = nodes.get(rhs) {
        node
    } else {
        let node = g.add_node(());
        nodes.insert(rhs.clone(), node);
        node
    }
}

fn part1(input: impl Iterator<Item = String>) -> usize {
    // Turns out there's an algorithm to find the min edge cut of a connected graph required
    // to partition the graph into to parts. Call that wonderful algorithm to confirm that the
    // min edge cut is indeed 3. Then compute the size of the two partitions.
    //
    // How we were expected to write this ourselves, I have no idea. It's a pretty complex algorithm.

    let graph = parse_input(input);

    let mut edges = Vec::new();
    let mut nodes = HashMap::new();
    let mut g: UnGraph<(), ()> = UnGraph::new_undirected();
    for (rhs, lhs) in graph {
        let rhs_node = node_for_label(&rhs, &mut g, &mut nodes);
        for n in lhs {
            let lhs_node = node_for_label(&n, &mut g, &mut nodes);
            edges.push((rhs_node, lhs_node));
        }
    }
    g.extend_with_edges(edges);

    let min_cut_res: rustworkx_core::Result<Option<(usize, Vec<_>)>> =
        stoer_wagner_min_cut(&g, |_| Ok(1));
    let (min_cut, partition) = min_cut_res.unwrap().unwrap();
    assert_eq!(min_cut, 3);
    let p1 = partition.len();
    let p2 = nodes.len() - p1;
    p1 * p2
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
        assert_eq!(part1(EXAMPLE1.lines().map(|v| v.to_string())), 54);
    }

    #[test]
    fn test_part1() {
        let res = part1(read_file());
        println!("{}", res);
        assert_eq!(res, 520380);
    }
}
