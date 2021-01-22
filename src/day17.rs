use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Hash, Eq)]
struct Coord(i32, i32, i32);

#[derive(Debug)]
enum Cube {
    Inactive,
    Active,
}

type State = HashMap<Coord, Cube>;

#[aoc_generator(day16)]
fn input_parse(inp: &str) -> State {
    let mut state: State = HashMap::new();
    let mut y = 0;
    let mut x = 0;
    let z = 0;
    for line in inp.lines() {
        x = 0;
        for c in line.chars() {
            let value = match c {
                '#' => Cube::Active,
                '.' => Cube::Inactive,
                _ => panic!("Unexpected char: {}", c),
            };
            state.insert(Coord(x, y, z), value);
            x += 1;
        }
        y += 1;
    }
    state
}

// fn step(state: &State) -> State {
//     let mut updates_
// }

// #[aoc(day17, part1)]
// fn solve_part1(inp: ()) -> () {
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parser() {
        let inp = ".#.
..#
###";
        let state = input_parse(inp);
        dbg!(state);
        assert_eq!(true, false);
    }
}
