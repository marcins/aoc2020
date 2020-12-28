use aoc_runner_derive::aoc;
use lazy_static::lazy_static;
use petgraph::dot::{Config, Dot};
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::Dfs;
use petgraph::Direction;
use regex::Regex;
use std::collections::HashMap;

type Edges<'a> = Vec<(&'a str, &'a str, usize)>;

// cargo-aoc has an issue with lifetimes, so we can't use this as an [#aoc_generator] function
// Takes the wordy input from AoC and creates a vector of edge definitions (source bag, target bag, number)
fn input_parse(inp: &str) -> Edges {
    lazy_static! {
        static ref RE_BAG: Regex = Regex::new(r"((\d+) (.*?)|no other) bags?").unwrap();
    }
    let parsed: Edges = inp
        .lines()
        .flat_map(|line| {
            let parts: Vec<&str> = line.split(" bags contain ").collect();
            let src = parts[0];
            let dst: Edges = parts[1]
                .split(", ")
                .filter_map(|bag_str| {
                    let caps = RE_BAG.captures(bag_str).unwrap();
                    if caps.get(1).unwrap().as_str() == "no other" {
                        None
                    } else {
                        let count = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
                        let bag = caps.get(3).unwrap().as_str();
                        Option::from((src, bag, count))
                    }
                })
                .collect();
            dst
        })
        .collect();
    parsed
}

fn edges_to_graph<'a>(edges: &Edges<'a>) -> DiGraph<&'a str, usize> {
    let mut nodes = HashMap::<&str, NodeIndex>::new();
    let mut graph = DiGraph::<&str, usize>::new();
    for edge in edges {
        let (src, dst, count) = edge;
        if !nodes.contains_key(src) {
            let node = graph.add_node(src);
            nodes.insert(src, node);
        }
        if !nodes.contains_key(dst) {
            let node = graph.add_node(dst);
            nodes.insert(dst, node);
        }
        graph.add_edge(*nodes.get(src).unwrap(), *nodes.get(dst).unwrap(), *count);
    }
    graph
}

#[aoc(day7, part1)]
fn solve_part1(input: &str) -> usize {
    // Workaround for cargo-aoc issue with lifetimes
    let mut graph = edges_to_graph(&input_parse(input));

    // Uncomment to get a .dot output for debugging
    // println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));

    const TARGET: &str = "shiny gold";
    // Find the target node
    let target_node = graph.node_indices().find(|i| graph[*i] == TARGET).unwrap();

    // Reverse the graph so we walk "up" from the target
    graph.reverse();

    // Find all paths out from target
    let mut dfs = Dfs::new(&graph, target_node);
    let mut count = 0;
    while let Some(nx) = dfs.next(&graph) {
        // add up any nodes we pass through as these all "contain" our target bag
        if nx != target_node {
            count += 1;
        }
    }
    count
}

fn sum_subgraph(
    graph: &DiGraph<&str, usize>,
    seen: &mut HashMap<NodeIndex, usize>,
    start_node: NodeIndex,
) -> usize {
    // This might be possible as a Dfs or other built in search?
    // What we want to do is:
    // - from the starting node find all the contained bags
    // - multiply the number of contained bags (edge weight) by the sum of the sub-graph starting at the neighbour node
    let mut neighbors = graph
        .neighbors_directed(start_node, Direction::Outgoing)
        .detach();
    let mut total = 1;
    while let Some(node) = neighbors.next_node(&graph) {
        // For this node, the number of bags is the edge weight
        let num_bags = graph
            .edge_weight(graph.find_edge(start_node, node).unwrap())
            .unwrap();

        // if we've already visited the target node then we have it's total bag size
        let sum = if seen.contains_key(&node) {
            0
        } else {
            // recursively call with the target node
            sum_subgraph(&graph, seen, node)
        };
        // Upsert the subgraph sum into the "seen" cache
        let bag_sum = seen.entry(node).or_insert(sum);
        // The total is the number of bags + the sub-graph total (&* is to convert mutable ref to immutable)
        total += num_bags * &*bag_sum;
    }
    total
}

#[aoc(day7, part2)]
fn solve_part2(input: &str) -> usize {
    // Workaround for cargo-aoc issue with lifetimes
    let graph = edges_to_graph(&input_parse(input));
    // println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));

    const TARGET: &str = "shiny gold";
    // Find the target node
    let target_node = graph.node_indices().find(|i| graph[*i] == TARGET).unwrap();

    // maintain a cache sub-sums in case we see a node more than once
    let mut seen: HashMap<NodeIndex, usize> = HashMap::new();

    // add up the total number of bags in the subgraph starting from this node (recursive)
    sum_subgraph(&graph, &mut seen, target_node) - 1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
        // input_parse(input);
        assert_eq!(solve_part1(input), 4);
    }

    #[test]
    fn test_part2() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
        // input_parse(input);
        assert_eq!(solve_part2(input), 32);
    }

    #[test]
    fn test_part2_other() {
        let input = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";
        // input_parse(input);
        assert_eq!(solve_part2(input), 126);
    }
}
