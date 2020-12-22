use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
pub struct Policy {
    low: usize,
    high: usize,
    letter: char,
    pwd: String
}

#[aoc_generator(day2)]
pub fn gen(input: &str) -> Vec<Policy> {
    input.lines().map(|l| {
        let parts: Vec<&str> = l.split(" ").collect();
        let nums: Vec<usize> = parts[0].split("-").map(|v| v.parse().unwrap()).collect();
        let letter: char = parts[1].chars().nth(0).unwrap();
        let pwd = parts[2].into();

        Policy {
            low: nums[0],
            high: nums[1],
            letter,
            pwd
        }
    }).collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &Vec<Policy>) -> usize {
    input.iter().filter(|v| {
        let count = v.pwd.chars().filter(|c| *c == v.letter).count();
        count >= v.low && count <= v.high
    }).count()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &Vec<Policy>) -> usize {
    input.iter().filter(|v| {
        let low = v.pwd.chars().nth(v.low - 1).unwrap();
        let high = v.pwd.chars().nth(v.high - 1).unwrap();
        (low == v.letter && high != v.letter) || (low != v.letter && high == v.letter)
    }).count()
}
