use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Clone, Copy)]
enum OpCodes {
    Nop(isize),
    Acc(isize),
    Jmp(isize),
}

#[aoc_generator(day8)]
fn parse_input(inp: &str) -> Vec<OpCodes> {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"^(jmp|acc|nop) (\+|\-)(\d+)$").unwrap();
    }
    inp.lines()
        .map(|line| {
            let caps = REGEX.captures(line).unwrap();
            let opcode = caps.get(1).unwrap().as_str();
            let math = caps.get(2).unwrap().as_str();
            let mut value = caps.get(3).unwrap().as_str().parse::<isize>().unwrap();
            match math {
                "-" => value *= -1,
                _ => (),
            };
            match opcode {
                "nop" => OpCodes::Nop(value),
                "acc" => OpCodes::Acc(value),
                "jmp" => OpCodes::Jmp(value),
                _ => panic!("Unknown opcode {}", opcode),
            }
        })
        .collect()
}

#[derive(Debug)]
struct Computer {
    pc: usize,
    acc: isize,
}

#[derive(Debug, PartialEq)]
enum ExitCode {
    Normal,
    Looped,
}

impl Computer {
    fn new() -> Self {
        Computer { pc: 0, acc: 0 }
    }
    fn execute(&mut self, program: &[OpCodes]) -> ExitCode {
        let mut executed_lines = HashSet::<usize>::new();
        loop {
            if !executed_lines.insert(self.pc) {
                return ExitCode::Looped;
            } else if self.pc >= program.len() {
                return ExitCode::Normal;
            }
            match program[self.pc as usize] {
                OpCodes::Acc(v) => {
                    self.acc += v;
                    self.pc += 1;
                }
                OpCodes::Jmp(v) => self.pc = (self.pc as isize + v) as usize,
                OpCodes::Nop(_v) => {
                    self.pc += 1;
                }
            }
        }
    }
}

#[aoc(day8, part1)]
fn solve_part1(program: &[OpCodes]) -> isize {
    let mut computer = Computer::new();
    computer.execute(program);
    computer.acc
}

fn variant_eq(a: &OpCodes, b: &OpCodes) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}

#[aoc(day8, part2)]
fn solve_part2(program: &[OpCodes]) -> isize {
    for i in 0..program.len() {
        // If we were going to modify an Acc then skip this program
        if variant_eq(&program[i], &OpCodes::Acc(0)) {
            continue;
        }
        let mut computer = Computer::new();
        // Create a new program which is a copy of the current
        // program with the appropriate replacement
        let this_program: Vec<OpCodes> = program
            .iter()
            .enumerate()
            .map(|(index, opcode)| {
                if index != i {
                    *opcode
                } else {
                    match opcode {
                        OpCodes::Acc(v) => OpCodes::Acc(*v),
                        OpCodes::Jmp(v) => OpCodes::Nop(*v),
                        OpCodes::Nop(v) => OpCodes::Jmp(*v),
                    }
                }
            })
            .collect();

        let exit_code = computer.execute(&this_program);
        if exit_code == ExitCode::Normal {
            return computer.acc;
        }
    }
    panic!("Solution not found!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parser() {
        let input = "nop -123
acc +5
acc -5
jmp +37
jmp -69";
        assert_eq!(
            parse_input(input),
            vec![
                OpCodes::Nop(-123),
                OpCodes::Acc(5),
                OpCodes::Acc(-5),
                OpCodes::Jmp(37),
                OpCodes::Jmp(-69)
            ],
        );
    }

    #[test]
    fn test_execute() {
        let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        assert_eq!(solve_part1(&parse_input(input)), 5);
    }

    #[test]
    fn test_modify() {
        let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        assert_eq!(solve_part2(&parse_input(input)), 8);
    }
}
