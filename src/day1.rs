use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn gen(input: &str) -> Vec<i32> {
    input.lines().map(|l| l.parse::<i32>().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &Vec<i32>) -> i32 {
    let mut i = 0;
    while i < input.len() - 1 {
        let mut j = i + 1;
        while j < input.len() {
            if input[i] + input[j] == 2020 {
                print!("{} x {}", input[i], input[j]);
                return input[i] * input[j];
            }
            j = j + 1;
        }
        i = i + 1;
    }
    panic!("Not found")
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &Vec<i32>) -> i32 {
    let mut i = 0;
    while i < input.len() - 1 {
        let mut j = i + 1;
        while j < input.len() {
            let mut k = j + 1;
            while k < input.len() {
                if input[i] + input[j] + input[k] == 2020 {
                    println!("{} x {} x {}", input[i], input[j], input[k]);
                    return input[i] * input[j] * input[k];
                }
                k = k + 1;
            }
            j = j + 1;
        }
        i = i + 1;
    }
    panic!("Not found")
}
