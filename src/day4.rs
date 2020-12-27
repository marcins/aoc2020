use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

#[aoc_generator(day4)]
fn input_parse(input: &str) -> Vec<HashMap<String, String>> {
    let mut passports = Vec::new();
    let mut passport: HashMap<String, String> = HashMap::new();
    for line in input.lines() {
        if line.trim() == "" {
            passports.push(passport);
            passport = HashMap::new();
        } else {
            for kv in line.split(" ") {
                let pair: Vec<&str> = kv.split(":").collect();
                // println!("{:?}", pair);
                passport.insert(pair[0].to_string(), pair[1].to_string());
            }
        }
    }
    passports.push(passport);
    passports
}

fn validate_hgt(hgt: Option<&String>) -> bool {
    lazy_static! {
        static ref RE_HGT: Regex = Regex::new("(\\d+)(cm|in)").unwrap();
    }
    if hgt.is_none() {
        return false;
    }
    let caps = RE_HGT.captures(hgt.unwrap());
    match caps {
        Some(caps) => {
            let units = caps.get(2).unwrap().as_str();
            let value = caps.get(1).unwrap().as_str().parse::<i32>();
            match value {
                Err(_) => false,
                Ok(value) => match units {
                    "in" => value >= 59 && value <= 76,
                    "cm" => value >= 150 && value <= 193,
                    _ => false,
                },
            }
        }
        _ => false,
    }
}

fn validate_hcl(hcl: Option<&String>) -> bool {
    lazy_static! {
        static ref RE_HCL: Regex = Regex::new("#[a-f0-9]{6}").unwrap();
    }
    match hcl {
        None => false,
        Some(v) => RE_HCL.is_match(v),
    }
}

fn validate_pid(pid: Option<&String>) -> bool {
    lazy_static! {
        static ref RE_PID: Regex = Regex::new("^\\d{9}$").unwrap();
    }
    match pid {
        None => false,
        Some(v) => RE_PID.is_match(v),
    }
}

fn validate_numstr(value: Option<&String>, min: i32, max: i32) -> bool {
    let parsed = value.unwrap_or(&String::from("0")).parse::<i32>().unwrap();
    parsed >= min && parsed <= max
}

fn validate(passport: &HashMap<String, String>) -> bool {
    validate_numstr(passport.get("byr"), 1920, 2002)
        && validate_numstr(passport.get("iyr"), 2010, 2020)
        && validate_numstr(passport.get("eyr"), 2020, 2030)
        && validate_hgt(passport.get("hgt"))
        && validate_hcl(passport.get("hcl"))
        && vec![
            String::from("amb"),
            String::from("blu"),
            String::from("brn"),
            String::from("gry"),
            String::from("grn"),
            String::from("hzl"),
            String::from("oth"),
        ]
        .contains(passport.get("ecl").unwrap_or(&String::from("")))
        && validate_pid(passport.get("pid"))
}

#[aoc(day4, part1)]
fn solve_part1(passports: &Vec<HashMap<String, String>>) -> usize {
    println!("{:#?} passports", passports.len());
    passports
        .iter()
        .filter(|p| p.keys().len() == 8 || (p.keys().len() == 7 && !p.contains_key("cid")))
        .count()
}

#[aoc(day4, part2)]
fn solve_part2(passports: &Vec<HashMap<String, String>>) -> usize {
    passports
        .iter()
        .filter(|p| p.keys().len() == 8 || (p.keys().len() == 7 && !p.contains_key("cid")))
        .filter(|p| validate(p))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_hgt() {
        assert_eq!(validate_hgt(Option::from(&String::from("169cm"))), true);
        assert_eq!(validate_hgt(Option::from(&String::from("69in"))), true);
        assert_eq!(validate_hgt(Option::from(&String::from("169in"))), false);
        assert_eq!(validate_hgt(Option::from(&String::from("69cm"))), false);
        assert_eq!(validate_hgt(Option::from(&String::from("garbage"))), false);
    }

    #[test]
    fn test_validate_hcl() {
        assert_eq!(validate_hcl(Option::None), false);
        assert_eq!(validate_hcl(Option::from(&String::from("#abcdef"))), true);
        assert_eq!(validate_hcl(Option::from(&String::from("#00ccff"))), true);
        assert_eq!(validate_hcl(Option::from(&String::from("#996699"))), true);
        assert_eq!(validate_hcl(Option::from(&String::from("#abcde"))), false);
    }

    #[test]
    fn test_invalid_passports() {
        let inp = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";
        assert_eq!(solve_part2(&input_parse(inp)), 0);
    }

    #[test]
    fn test_valid_passports() {
        let inp = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
        assert_eq!(solve_part2(&input_parse(inp)), 4);
    }
}
