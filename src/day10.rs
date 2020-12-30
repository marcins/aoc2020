use aoc_runner_derive::{aoc, aoc_generator};
use petgraph::algo;
use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::HashMap;

#[aoc_generator(day10)]
fn parse_input(input: &str) -> Vec<u64> {
    let mut joltages: Vec<u64> = input.lines().map(|v| v.parse::<u64>().unwrap()).collect();
    joltages.sort();
    joltages.insert(0, 0);
    joltages.push(joltages[joltages.len() - 1] + 3);
    joltages
}

#[aoc(day10, part1)]
fn solve_part1(joltages: &[u64]) -> u64 {
    let (ones, threes) = joltages
        .windows(2)
        .fold((0, 0), |acc, pair| match pair[1] - pair[0] {
            1 => (acc.0 + 1, acc.1),
            3 => (acc.0, acc.1 + 1),
            _ => acc,
        });
    ones * threes
}

struct Adapters {
    graph: DiGraph<u64, ()>,
    root_nx: NodeIndex,
    device_nx: NodeIndex,
}

fn build_graph(joltages: &[u64]) -> Adapters {
    let mut graph = DiGraph::<u64, ()>::new();
    let mut node_map = HashMap::<u64, NodeIndex>::new();
    let edges: Vec<(u64, u64)> = Vec::new();
    // Nodes are the adapters
    for joltage in joltages.iter() {
        let nx = graph.add_node(*joltage);
        node_map.insert(*joltage, nx);
    }

    let root_nx = node_map.get(&joltages[0]).unwrap();
    let device_nx = node_map.get(&joltages[joltages.len() - 1]).unwrap();

    // created directed edges from each node to each node with joltage within 3
    for (v, nx) in node_map.iter() {
        for v2 in joltages.iter() {
            if *v2 <= *v {
                continue;
            }
            if *v2 > *v + 3 {
                continue;
            }
            graph.add_edge(*nx, *node_map.get(v2).unwrap(), ());
        }
    }
    Adapters {
        graph,
        root_nx: *root_nx,
        device_nx: *device_nx,
    }
}

#[aoc(day10, part2)]
fn solve_part2(joltages: &[u64]) -> usize {
    let adapters = build_graph(joltages);
    // This works for the examples but is impractical for the real thing, need a
    // better solution..
    //
    // The thing to consider is that adapters have to be within 3 - eg. 1 can connect to 2, 3, or 4
    // This means if we're at a 1, and have a 2, 3 and 4 then have:
    //  1-4, 1-2-4, 1-2-3-4, 1-3-4 - 4 combos
    //
    algo::all_simple_paths::<Vec<NodeIndex>, &DiGraph<u64, ()>>(
        &adapters.graph,
        adapters.root_nx,
        adapters.device_nx,
        1,
        None,
    )
    // .map(|p| {
    //     dbg!(&p);
    //     p
    // })
    .count()
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample_small() {
        let input = "16
10
15
5
1
11
7
19
6
12
4";
        assert_eq!(solve_part1(&parse_input(input)), 35);
    }

    #[test]
    fn test_sample_small_part2() {
        let input = "16
10
15
5
1
11
7
19
6
12
4";
        assert_eq!(solve_part2(&parse_input(input)), 8);
    }

    #[test]
    fn test_sample_large_part2() {
        let input = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
        assert_eq!(solve_part2(&parse_input(input)), 19208);
    }
}
