use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day13)]
fn input_parse(inp: &str) -> (i32, Vec<i32>) {
    (
        inp.lines().nth(0).unwrap().parse::<i32>().unwrap(),
        inp.lines()
            .nth(1)
            .unwrap()
            .split(",")
            .filter(|v| *v != "x")
            .map(|v| v.parse::<i32>().unwrap())
            .collect(),
    )
}

#[aoc(day13, part1)]
fn solve_part1(inp: &(i32, Vec<i32>)) -> i32 {
    let (time, ids) = inp;
    let mut times: Vec<(i32, i32)> = ids
        .iter()
        .map(|id| {
            let mins = id - (time % id);
            (*id, mins)
        })
        .collect();

    times.sort_by(|a, b| a.1.cmp(&b.1));
    times[0].0 * times[0].1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parser() {
        let inp = "939
7,13,x,x,59,x,31,19";
        assert_eq!(input_parse(inp), (939, vec![7, 13, 59, 31, 19]));
    }

    #[test]
    fn test_part1() {
        let inp = "939
7,13,x,x,59,x,31,19";
        assert_eq!(solve_part1(&input_parse(inp)), 295);
    }
}
