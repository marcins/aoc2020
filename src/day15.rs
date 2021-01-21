use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[aoc_generator(day15)]
fn input_parse(inp: &str) -> Vec<i32> {
    inp.split(",").map(|v| v.parse::<i32>().unwrap()).collect()
}

enum Ref {
    First(usize),
    Later(usize, usize),
}

fn update_entry_for_turn(map: &mut HashMap<i32, Ref>, turn: usize, num: i32) {
    if map.contains_key(&num) {
        let new_entry = match map.get(&num).unwrap() {
            Ref::First(prev_turn) => Ref::Later(*prev_turn, turn),
            Ref::Later(_, prev_turn) => Ref::Later(*prev_turn, turn),
        };
        map.insert(num, new_entry);
    } else {
        map.insert(num, Ref::First(turn));
    }
}

fn play_game(starting_numbers: &[i32], turns: usize) -> i32 {
    let mut last_spoken: HashMap<i32, Ref> = HashMap::new();

    for (idx, num) in starting_numbers.iter().enumerate() {
        update_entry_for_turn(&mut last_spoken, idx + 1, *num);
    }
    let mut turn: usize = starting_numbers.len() + 1;
    let mut last_spoken_number = *starting_numbers.last().unwrap();
    while turn <= turns {
        match last_spoken.get(&last_spoken_number) {
            Some(last_turn_spoken) => {
                last_spoken_number = match last_turn_spoken {
                    Ref::First(_) => 0,
                    Ref::Later(a, b) => (b - a) as i32,
                }
            }
            None => {
                last_spoken_number = 0;
            }
        }
        update_entry_for_turn(&mut last_spoken, turn, last_spoken_number);
        turn += 1;
    }
    last_spoken_number
}

#[aoc(day15, part1)]
fn solve_part1(inp: &[i32]) -> i32 {
    play_game(inp, 2020)
}

#[aoc(day15, part2)]
fn solve_part2(inp: &[i32]) -> i32 {
    play_game(inp, 30000000)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let inp = vec![0, 3, 6];
        assert_eq!(play_game(&inp, 10), 0);
        assert_eq!(play_game(&inp, 2020), 436);
    }

    #[test]
    fn test_part2() {
        // Warning: this is slow to run in non-release mode :)
        let inp = vec![0, 3, 6];
        assert_eq!(play_game(&inp, 30000000), 175594);
    }
}
