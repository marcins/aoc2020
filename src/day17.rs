use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;
use std::fmt::Write;

#[derive(Debug, PartialEq, Hash, Eq, Clone, Copy)]
struct Coord(i32, i32, i32);

struct State {
    active: HashSet<Coord>,
    bounds_min: Coord,
    bounds_max: Coord,
}

impl State {
    fn map<F>(self: &Self, f: F) -> HashSet<Coord>
    where
        F: Fn(Coord, bool) -> bool,
    {
        let mut new_active = HashSet::new();
        for z in self.bounds_min.2..self.bounds_max.2 {
            for y in self.bounds_min.1..self.bounds_max.1 {
                for x in self.bounds_min.0..self.bounds_max.0 {
                    let c = Coord(x, y, z);
                    let new_value = f(c, self.active.contains(&c));
                    if new_value {
                        new_active.insert(c);
                    }
                }
            }
        }
        new_active
    }

    fn expand_bounds(self: &Self) -> (Coord, Coord) {
        let Coord(min_x, min_y, min_z) = self.bounds_min;
        let Coord(max_x, max_y, max_z) = self.bounds_max;
        (
            Coord(min_x - 1, min_y - 1, min_z - 1),
            Coord(max_x + 1, max_y + 1, max_z + 1),
        )
    }
}

impl std::fmt::Debug for State {
    fn fmt(self: &Self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        for z in self.bounds_min.2..self.bounds_max.2 {
            out.push_str(&format!("\nz {}", z));
            for y in self.bounds_min.1..self.bounds_max.1 {
                out.write_char('\n')?;
                for x in self.bounds_min.0..self.bounds_max.0 {
                    match self.active.contains(&Coord(x, y, z)) {
                        true => out.write_char('#')?,
                        false => out.write_char('.')?,
                    };
                }
            }
        }
        out.write_char('\n')?;
        f.write_str(&out)
    }
}

#[aoc_generator(day16)]
fn input_parse(inp: &str) -> State {
    let mut active: HashSet<Coord> = HashSet::new();
    let mut x = 1;
    let mut y = 1;
    let mut max_x = 0;
    let z = 1;
    for line in inp.lines() {
        max_x = x;
        x = 1;
        for c in line.chars() {
            match c {
                '#' => {
                    active.insert(Coord(x, y, z));
                }
                '.' => (),
                _ => panic!("Unexpected char: {}", c),
            };
            x += 1;
        }
        y += 1;
    }
    State {
        active,
        bounds_min: Coord(0, 0, 0),
        bounds_max: Coord(max_x + 1, y + 1, 3),
    }
}

fn step(state: &State) -> State {
    let new_active = state.map(|coord, active| {
        let Coord(x, y, z) = coord;
        let mut adj_active = 0;
        for zz in z - 1..=z + 1 {
            for yy in y - 1..=y + 1 {
                for xx in x - 1..=x + 1 {
                    if xx == x && yy == y && zz == z {
                        continue;
                    }
                    if state.active.contains(&Coord(xx, yy, zz)) {
                        adj_active += 1;
                    }
                }
            }
        }
        match active {
            true if adj_active == 2 || adj_active == 3 => true,
            false if adj_active == 3 => true,
            _ => false,
        }
    });

    let (new_min, new_max) = state.expand_bounds();

    State {
        active: new_active,
        bounds_min: new_min,
        bounds_max: new_max,
    }
}

#[aoc(day17, part1)]
fn solve_part1(inp: &str) -> usize {
    let mut state = input_parse(inp);
    for _ in 0..6 {
        // dbg!(&state);
        state = step(&state);
    }
    // dbg!(&state);
    state.active.len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let inp = ".#.
..#
###";
        let mut state = input_parse(inp);
        state = step(&state);
        for _ in 0..5 {
            state = step(&state);
        }
        assert_eq!(state.active.len(), 112);
    }
}
