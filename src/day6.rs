use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, HashSet};

#[aoc_generator(day6)]
fn input_parse(inp: &str) -> Vec<Vec<Vec<char>>> {
    let mut result = Vec::new();
    let mut curr = Vec::new();
    for line in inp.lines() {
        if line == "" {
            result.push(curr);
            curr = Vec::new();
        } else {
            curr.push(line.clone().chars().collect());
        }
    }
    result.push(curr);
    result
}

#[aoc(day6, part1)]
fn solve_part1(groups: &Vec<Vec<Vec<char>>>) -> usize {
    groups
        .iter()
        .map(|group| {
            let mut set = HashSet::<char>::new();
            for line in group {
                for c in line.iter() {
                    set.insert(*c);
                }
            }
            set.len()
        })
        .sum()
}

#[aoc(day6, part2)]
fn solve_part2(groups: &Vec<Vec<Vec<char>>>) -> usize {
    groups
        .iter()
        .map(|group| {
            let mut map = HashMap::<char, usize>::new();
            for line in group.iter() {
                for c in line.iter() {
                    let v = map.entry(*c).or_insert(0);
                    *v += 1;
                }
            }
            map.values().filter(|v| **v == group.len()).count()
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let group = "abc

a
b
c

ab
ac

a
a
a
a

b";
        assert_eq!(solve_part1(&input_parse(group)), 11);
    }

    #[test]
    fn test_part2() {
        let group = "abc

a
b
c

ab
ac

a
a
a
a

b";
        assert_eq!(solve_part2(&input_parse(group)), 6);
    }
}
