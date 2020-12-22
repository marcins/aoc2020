use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
pub struct Policy {
    low: u16,
    high: u16,
    letter: char,
    pwd: String
}

#[aoc_generator(day2)]
pub fn gen(input: &str) -> Vec<Policy> {
    input.lines().map(|l| {
        let parts: Vec<&str> = l.split(" ").collect();
        let nums: Vec<u16> = parts[0].split("-").map(|v| v.parse().unwrap()).collect();
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
        let mut count = 0;
        for c in v.pwd.chars() {
            if c == v.letter {
                count += 1;
            }
        }
        count >= v.low && count <= v.high
    }).count()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &Vec<Policy>) -> usize {
    input.iter().filter(|v| {
        let low = v.pwd.chars().nth((v.low - 1) as usize).unwrap();
        let high = v.pwd.chars().nth((v.high - 1) as usize).unwrap();
        (low == v.letter && high != v.letter) || (low != v.letter && high == v.letter)
    }).count()
}


// #[aoc(day1, part2)]
// pub fn solve_part2(input: &Vec<i32>) -> i32 {
//     let mut i = 0;
//     while i < input.len() - 1 {
//         let mut j = i + 1;
//         while j < input.len() {
//             let mut k = j + 1;
//             while k < input.len() {
//                 if input[i] + input[j] + input[k] == 2020 {
//                     println!("{} x {} x {}", input[i], input[j], input[k]);
//                     return input[i] * input[j] * input[k];
//                 }
//                 k = k + 1;
//             }
//             j = j + 1;
//         }
//         i = i + 1;
//     }
//     panic!("Not found")
// }
