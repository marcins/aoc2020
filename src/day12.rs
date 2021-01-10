use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, PartialEq)]
enum Command {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

#[derive(Debug, PartialEq)]
struct Instruction {
    command: Command,
    value: i32,
}

#[derive(Debug)]
struct Ferry {
    direction: i16,
    x: i32,
    y: i32,
    wpt_x: i32,
    wpt_y: i32,
}

impl Ferry {
    fn new() -> Self {
        Ferry {
            direction: 90,
            x: 0,
            y: 0,
            wpt_x: 10,
            wpt_y: -1,
        }
    }

    fn apply_simple(self: &mut Self, instructions: &[Instruction]) {
        for instruction in instructions.iter() {
            match instruction.command {
                Command::North => self.y -= instruction.value,
                Command::East => self.x += instruction.value,
                Command::South => self.y += instruction.value,
                Command::West => self.x -= instruction.value,
                Command::Left => self.direction = (self.direction - instruction.value as i16) % 360,
                Command::Right => {
                    self.direction = (self.direction + instruction.value as i16) % 360
                }
                Command::Forward => match self.direction {
                    0 => self.y -= instruction.value,
                    90 => self.x += instruction.value,
                    180 => self.y += instruction.value,
                    270 => self.x -= instruction.value,
                    d => panic!("Unsupported direction: {}", d),
                },
            }
            if self.direction < 0 {
                self.direction += 360;
            }
        }
    }

    fn rotate(x: i32, y: i32, theta: i32) -> (i32, i32) {
        let theta = if theta < 0 { theta + 360 } else { theta };
        let theta_rad = theta as f64 * (std::f64::consts::PI / 180.0);

        // Forgot all my maths, so found this:
        // https://stackoverflow.com/questions/3162643/proper-trigonometry-for-rotating-a-point-around-the-origin
        let (xx, yy) = (
            (x as f64 * theta_rad.cos() - y as f64 * theta_rad.sin()),
            x as f64 * theta_rad.sin() + y as f64 * theta_rad.cos(),
        );
        (xx.round() as i32, yy.round() as i32)
    }

    fn apply_wpt(self: &mut Self, instructions: &[Instruction]) {
        for instruction in instructions.iter() {
            match instruction.command {
                Command::North => self.wpt_y -= instruction.value,
                Command::East => self.wpt_x += instruction.value,
                Command::South => self.wpt_y += instruction.value,
                Command::West => self.wpt_x -= instruction.value,
                Command::Left => {
                    let (x, y) = Ferry::rotate(self.wpt_x, self.wpt_y, -instruction.value);
                    self.wpt_x = x;
                    self.wpt_y = y;
                }
                Command::Right => {
                    let (x, y) = Ferry::rotate(self.wpt_x, self.wpt_y, instruction.value);
                    self.wpt_x = x;
                    self.wpt_y = y;
                }
                Command::Forward => {
                    self.x += self.wpt_x * instruction.value;
                    self.y += self.wpt_y * instruction.value;
                }
            }
            if self.direction < 0 {
                self.direction += 360;
            }
        }
    }
}

#[aoc_generator(day12)]
fn input_parse(inp: &str) -> Vec<Instruction> {
    inp.lines()
        .map(|line| {
            let command = match line.chars().nth(0).unwrap() {
                'N' => Command::North,
                'S' => Command::South,
                'E' => Command::East,
                'W' => Command::West,
                'L' => Command::Left,
                'R' => Command::Right,
                'F' => Command::Forward,
                c => panic!("Unknown command {}", c),
            };
            let value = line[1..].parse::<i32>().unwrap();
            Instruction { command, value }
        })
        .collect()
}

#[aoc(day12, part1)]
fn solve_part1(inp: &Vec<Instruction>) -> i32 {
    let mut ferry = Ferry::new();
    ferry.apply_simple(inp);
    ferry.x.abs() + ferry.y.abs()
}

#[aoc(day12, part2)]
fn solve_part2(inp: &Vec<Instruction>) -> i32 {
    let mut ferry = Ferry::new();
    ferry.apply_wpt(inp);
    ferry.x.abs() + ferry.y.abs()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rotate() {
        assert_eq!(Ferry::rotate(10, -4, 90), (4, 10));
        assert_eq!(Ferry::rotate(10, -4, 180), (-10, 4));
        assert_eq!(Ferry::rotate(10, -4, 270), (-4, -10));
        assert_eq!(Ferry::rotate(10, -4, -270), (4, 10));
        assert_eq!(Ferry::rotate(10, -4, -180), (-10, 4));
    }

    #[test]
    fn test_parser() {
        let inp = "F10
N3
F7
R90
F11";
        assert_eq!(
            input_parse(inp),
            vec![
                Instruction {
                    command: Command::Forward,
                    value: 10
                },
                Instruction {
                    command: Command::North,
                    value: 3
                },
                Instruction {
                    command: Command::Forward,
                    value: 7
                },
                Instruction {
                    command: Command::Right,
                    value: 90
                },
                Instruction {
                    command: Command::Forward,
                    value: 11
                }
            ]
        );
    }

    #[test]
    fn test_apply() {
        let inp = "F10
N3
F7
R90
F11";
        assert_eq!(solve_part1(&input_parse(inp)), 25);
    }

    #[test]
    fn test_apply_wpt() {
        let inp = "F10
N3
F7
R90
F11";
        assert_eq!(solve_part2(&input_parse(inp)), 286);
    }
}
