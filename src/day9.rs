use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day9)]
fn parse_input(input: &str) -> Vec<u64> {
    input.lines().map(|v| v.parse::<u64>().unwrap()).collect()
}

fn has_sum(numbers: &[u64], target: u64) -> bool {
    for i in 0..numbers.len() {
        for j in 1..numbers.len() {
            if i == j {
                continue;
            }
            if numbers[i] + numbers[j] == target {
                return true;
            }
        }
    }
    false
}

fn find_rule_breaker(numbers: &[u64], preamble_size: usize) -> u64 {
    *numbers[preamble_size..]
        .iter()
        .enumerate()
        .find(|(i, n)| !has_sum(&numbers[*i..*i + preamble_size], **n))
        .unwrap()
        .1
}

fn find_weakness(numbers: &[u64], target: u64) -> u64 {
    // remove any numbers bigger than the target
    let subset: Vec<&u64> = numbers.iter().filter(|v| **v < target).collect();

    for i in 0..subset.len() {
        for j in i..subset.len() {
            let set = &subset[i..j];
            if set.iter().map(|v| **v).sum::<u64>() == target {
                return **(set.iter().max().unwrap()) + **(set.iter().min().unwrap());
            }
        }
    }
    panic!("Solution not found");
}

#[aoc(day9, part1)]
fn solve_part1(numbers: &[u64]) -> u64 {
    find_rule_breaker(numbers, 25)
}

#[aoc(day9, part2)]
fn solve_part2(numbers: &[u64]) -> u64 {
    find_weakness(&numbers, find_rule_breaker(&numbers, 25))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rule_breaker() {
        let input = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
        assert_eq!(find_rule_breaker(&parse_input(input), 5), 127);
    }

    #[test]
    fn test_weakness_finder() {
        let input = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
        let numbers = parse_input(input);
        assert_eq!(find_weakness(&numbers, find_rule_breaker(&numbers, 5)), 62);
    }
}
