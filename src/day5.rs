use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day5)]
fn generator(input: &str) -> Vec<i16> {
    let mut passes = input.lines().map(|pass| seat_id(pass)).collect::<Vec<i16>>();
    passes.sort();
    passes
}

fn splitter(pgm: &str, low: char, high: char, from: i16, to: i16) -> i16 {
    let (mut a, mut b) = (from, to);
    for c in pgm.chars() {
        if c == low {
            b = a + (b-a)/2
        } else if c == high {
            a = a + (b-a)/2 + 1
        } else {
            panic!("Unexpected char {}", c)
        };
    }
    assert_eq!(a, b);
    a
}

fn seat_id(pgm: &str) -> i16 {
    (splitter(&pgm[0..7], 'F', 'B', 0, 127) * 8) + 
    splitter(&pgm[7..10], 'L', 'R', 0, 7)
}

#[aoc(day5, part1)]
fn solve_part1(seats: &Vec<i16>) -> i16 {
    *seats.iter().max().unwrap()
}

#[aoc(day5, part2)]
fn solve_part2(seats: &Vec<i16>) -> i16 {
    for seat in seats {
        if seats.contains(&(seat + 2)) && !seats.contains(&(seat + 1)){
           return seat.clone() + 1
        }
    }
    panic!("Seat not found");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_splitter() {
        assert_eq!(splitter("FBFBBFF", 'F', 'B', 0, 127), 44);
        assert_eq!(splitter("RLR", 'L', 'R', 0, 7), 5);
    }

    #[test]
    fn test_seat_id() {
        assert_eq!(seat_id("BFFFBBFRRR"), 567);
        assert_eq!(seat_id("FFFBBBFRRR"), 119);
        assert_eq!(seat_id("BBFFBBFRLL"), 820);
    }
}