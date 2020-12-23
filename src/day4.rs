use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: String
}

#[aoc_generator(day4)]
fn input_parse(input: &str) -> Vec<HashMap<String, String>> {
    let mut passports = Vec::new();
    let mut passport: HashMap<String, String> = HashMap::new();
    for line in input.lines() {
        if line.trim() == "" {
                passports.push(passport);
                passport  = HashMap::new();
        } else {
            for kv in line.split(" ") {
                let pair: Vec<&str> = kv.split(":").collect();
                // println!("{:?}", pair);
                passport.insert(pair[0].to_string(), pair[1].to_string());            
            }
        }
    }
    passports
}


#[aoc(day4, part1)]
fn solve_part1(passports: &Vec<HashMap<String, String>>) -> usize {
    println!("{:#?} passports", passports.len());
    passports.iter().filter(|p| {
       p.keys().len() == 8 || (p.keys().len() == 7 && !p.contains_key("cid"))
    }).count()
}

// #[aoc(day3, part2)]
// fn solve_part2(map: &Map) -> i64 {
//     traverse(map, 1, 1) *
//     traverse(map, 3, 1) *
//     traverse(map, 5, 1) *
//     traverse(map, 7, 1) *
//     traverse(map, 1, 2)
// }

#[cfg(test)]
mod tests {
    use super::*;

    
//     #[test]
//     fn parse() {
//         let inp = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
// byr:1937 iyr:2017 cid:147 hgt:183cm

// iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
// hcl:#cfa07d byr:1929

// hcl:#ae17e1 iyr:2013
// eyr:2024
// ecl:brn pid:760753108 byr:1931
// hgt:179cm

// hcl:#cfa07d eyr:2025 pid:166559648
// iyr:2011 ecl:brn hgt:59in";
//         assert_eq!(input_parse(&inp), [("ecl", "gry")]
//     }

//     #[test]
//     fn part2() {
//         let inp = "..##.......
// #...#...#..
// .#....#..#.
// ..#.#...#.#
// .#...##..#.
// ..#.##.....
// .#.#.#....#
// .#........#
// #.##...#...
// #...##....#
// .#..#...#.#";
//         assert_eq!(solve_part2(&input_parse(&inp)), 336)
//     }
}