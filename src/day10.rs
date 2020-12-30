use aoc_runner_derive::{aoc, aoc_generator};

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

#[aoc(day10, part2)]
fn solve_part2(joltages: &[u64]) -> u64 {
    // I needed some hints for this part - I originally had a graph based solution
    // that worked for small samples but was too computationally expensive for
    // the full problem.
    #[derive(Debug)]
    struct Result {
        run_count: usize,
        result: u64,
    };

    // Find gaps of > 1, keeping track of how many were in a run of 1s - these
    // runs cause a combo, we multiply all the combos together to get the answer
    let result = joltages.windows(2).fold(
        Result {
            run_count: 0,
            result: 1,
        },
        |mut acc, v| {
            acc.run_count += 1;
            if v[1] - v[0] > 1 {
                // if there's a run on joltages then these correspond to a particular
                // number of combos.
                let size = match acc.run_count {
                    0 => 1,
                    1 => 1,
                    2 => 1,
                    3 => 2,
                    4 => 4,
                    5 => 7,
                    _ => panic!("Unexpected run"),
                };
                acc.result *= size;
                acc.run_count = 0;
                return acc;
            }
            acc
        },
    );
    result.result
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
