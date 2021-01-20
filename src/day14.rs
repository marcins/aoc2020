use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq)]
enum Command {
    Mask(String),
    Mem(i64, i64),
}

#[aoc_generator(day14)]
fn input_parse(inp: &str) -> Vec<Command> {
    lazy_static! {
        static ref RE_MASK: Regex = Regex::new(r"mask = ([X01]+)").unwrap();
        static ref RE_MEM: Regex = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
    }

    inp.lines()
        .map(|line| {
            let maybe_mask = RE_MASK.captures(line);
            if let Some(caps) = maybe_mask {
                return Command::Mask(caps.get(1).unwrap().as_str().to_owned());
            }
            let maybe_mem = RE_MEM.captures(line);
            if let Some(caps) = maybe_mem {
                return Command::Mem(
                    caps.get(1).unwrap().as_str().parse::<i64>().unwrap(),
                    caps.get(2).unwrap().as_str().parse::<i64>().unwrap(),
                );
            }
            panic!("Unexpected");
        })
        .collect()
}

fn apply_mask(mask: &str, value: i64) -> i64 {
    let mut new_value = value;
    for (i, char) in mask.chars().enumerate() {
        if char == 'X' {
            continue;
        }
        let exp = (2 as i64).pow((mask.len() - i - 1) as u32);

        if char == '1' {
            // dbg!(i, char, new_value, exp, new_value | exp);
            new_value |= exp;
        } else if char == '0' {
            // dbg!(i, char, new_value, exp, new_value ^ exp);
            new_value &= !exp;
        }
    }
    new_value
}

fn get_masks(mask: &str) -> Vec<String> {
    let mut masks: HashSet<usize> = HashSet::new();
    let len = mask.len();
    let floating_size = mask.chars().filter(|c| *c == 'X').count();
    for i in 0..(2 as usize).pow(floating_size as u32) {
        // distribute the bits of i into the floater
        let mut updated_mask = 0;
        let mut ii = 0;
        for (idx, c) in mask.chars().rev().enumerate() {
            match c {
                'X' => {
                    updated_mask |= ((i >> ii) & 1) << idx;
                    ii += 1;
                }
                v => updated_mask |= (v.to_digit(10).unwrap() as usize) << idx,
            };
            // updated_mask <<= 1;
        }
        masks.insert(updated_mask);
    }
    masks
        .iter()
        .map(|mask| match len {
            3 => format!("{:03b}", mask),
            36 => format!("{:036b}", mask),
            l => panic!("Unexpected mask length: {}", l),
        })
        .collect()
}

#[aoc(day14, part1)]
fn solve_part1(instructions: &[Command]) -> i64 {
    let mut memory: HashMap<i64, i64> = HashMap::new();
    let mut current_mask: &String = &String::from("");
    for command in instructions.iter() {
        match command {
            Command::Mask(mask) => current_mask = mask,
            Command::Mem(addr, value) => {
                memory.insert(*addr, apply_mask(current_mask, *value));
            }
        };
    }
    memory.values().sum()
}

#[aoc(day14, part2)]
fn solve_part2(instructions: &[Command]) -> i64 {
    let mut memory: HashMap<i64, i64> = HashMap::new();
    let mut current_mask: &String = &String::from("");
    for command in instructions.iter() {
        match command {
            Command::Mask(mask) => current_mask = mask,
            Command::Mem(addr, value) => {
                let binary_addr = format!("{:036b}", addr);
                // Takes the address and mask, and creates a base mask by applying the mask to the address
                // leaving Xs for floating bits
                let base_mask = current_mask
                    .chars()
                    .zip(binary_addr.chars())
                    .enumerate()
                    .map(|(_i, (mask_char, addr_char))| match mask_char {
                        '0' => addr_char,
                        _ => mask_char,
                    })
                    .collect::<String>();

                // produces masks to be used by "resolving" the floating bits
                let current_masks = get_masks(&base_mask);
                for mask in &current_masks {
                    // err.. this might not be required since there shouldn't be any floaters left?
                    let new_addr = apply_mask(&mask, *addr);
                    memory.insert(new_addr, *value);
                }
            }
        };
    }
    memory.values().sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_mask() {
        assert_eq!(apply_mask("1", 0), 1);
        assert_eq!(apply_mask("0", 1), 0);
        assert_eq!(apply_mask("010", 0b101), 0b010);
        assert_eq!(apply_mask("X1X", 0b101), 0b111);
        assert_eq!(apply_mask("0XX", 0b101), 0b001);
        assert_eq!(apply_mask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X", 11), 73);
        assert_eq!(apply_mask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X", 101), 101);
        assert_eq!(apply_mask("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X", 0), 64);
    }

    #[test]
    fn test_get_masks() {
        let mut masks = get_masks("XXX");
        masks.sort();
        assert_eq!(
            masks,
            vec!["000", "001", "010", "011", "100", "101", "110", "111"]
        )
    }

    #[test]
    fn test_get_masks_complex() {
        let mut masks = get_masks("X10");
        masks.sort();
        assert_eq!(masks, vec!["010", "110"])
    }

    #[test]
    fn test_get_masks_complex_sample() {
        let mut masks = get_masks("000000000000000000000000000000X1001X");
        masks.sort();
        assert_eq!(
            masks,
            vec![
                "000000000000000000000000000000010010",
                "000000000000000000000000000000010011",
                "000000000000000000000000000000110010",
                "000000000000000000000000000000110011"
            ]
        )
    }

    #[test]
    fn test_parser() {
        let inp = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";
        assert_eq!(
            input_parse(inp),
            vec![
                Command::Mask(String::from("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X")),
                Command::Mem(8, 11),
                Command::Mem(7, 101),
                Command::Mem(8, 0)
            ]
        );
    }

    #[test]
    fn test_part1() {
        let inp = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";
        assert_eq!(solve_part1(&input_parse(inp)), 165);
    }

    #[test]
    fn test_part2() {
        let inp = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";
        assert_eq!(solve_part2(&input_parse(inp)), 208);
    }
}
