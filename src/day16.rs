use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::ops::Range;

enum ParserState {
    Fields,
    Yours,
    Nearby,
}

struct Puzzle {
    fields: HashMap<String, (Range<i32>, Range<i32>)>,
    your_ticket: Vec<i32>,
    nearby_tickets: Vec<Vec<i32>>,
}

#[aoc_generator(day16)]
fn input_parse(inp: &str) -> Puzzle {
    lazy_static! {
        static ref RE_FIELD_INF: Regex =
            Regex::new(r"([^:]+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
    }
    let mut parser_state = ParserState::Fields;
    let mut fields: HashMap<String, (Range<i32>, Range<i32>)> = HashMap::new();
    let mut your_ticket = vec![];
    let mut nearby_tickets = vec![];

    for line in inp.lines() {
        match line {
            "your ticket:" => {
                parser_state = ParserState::Yours;
            }
            "nearby tickets:" => {
                parser_state = ParserState::Nearby;
            }
            "" => (),
            _ => match parser_state {
                ParserState::Fields => {
                    let caps = RE_FIELD_INF.captures(line).unwrap();
                    let cap_to_i32 =
                        |num| -> i32 { caps.get(num).unwrap().as_str().parse::<i32>().unwrap() };

                    let name = caps.get(1).unwrap().as_str();
                    let range1 = cap_to_i32(2)..cap_to_i32(3) + 1;
                    let range2 = cap_to_i32(4)..cap_to_i32(5) + 1;
                    fields.insert(name.to_owned(), (range1, range2));
                }
                ParserState::Nearby => nearby_tickets
                    .push(line.split(",").map(|v| v.parse::<i32>().unwrap()).collect()),
                ParserState::Yours => {
                    your_ticket = line.split(",").map(|v| v.parse::<i32>().unwrap()).collect()
                }
            },
        }
    }

    Puzzle {
        fields,
        your_ticket,
        nearby_tickets,
    }
}

type Mapping<'a> = HashMap<&'a str, Vec<usize>>;

fn find_mapping(puzzle: &Puzzle) -> Mapping {
    let mut mapping: Mapping = HashMap::new();
    let tickets = valid_tickets(puzzle);
    // find fields that each column could be and create a mapping from field name to list of columns
    for col in 0..puzzle.your_ticket.len() {
        let matching_fields = puzzle.fields.iter().filter(|(_, (r1, r2))| {
            tickets
                .iter()
                .all(|ticket| r1.contains(&ticket[col]) || r2.contains(&ticket[col]))
        });
        for (field_name, _) in matching_fields {
            let entry = mapping.entry(field_name).or_insert(vec![]);
            entry.push(col);
        }
    }

    // find any "unique" colums, remove them from any other lists where they're present,
    // repeat until we're done
    //
    // Note this doesn't uniquely resolve everything in the final puzzle, but it resolves everything
    // with "departure" in it which is all we care about for the soution.. :)
    //
    // There's probably also a more elegant way of writing this, but it works and runs in <1ms
    loop {
        let uniques: Vec<usize> = mapping
            .iter()
            .filter(|(_, cols)| cols.len() == 1)
            .map(|(_, cols)| cols[0])
            .collect();
        let mut unchanged = true;
        for unique in uniques {
            for (_, cols) in mapping.iter_mut() {
                if cols.len() > 0 && !(cols.len() == 1 && cols[0] == unique) {
                    match cols.iter().position(|v| *v == unique) {
                        Some(idx) => {
                            unchanged = false;
                            cols.remove(idx);
                        }
                        None => (),
                    }
                }
            }
        }
        if unchanged {
            break;
        }
    }
    mapping
}

#[aoc(day16, part1)]
fn solve_part1(inp: &Puzzle) -> i32 {
    inp.nearby_tickets
        .iter()
        .map(|ticket| -> i32 {
            ticket
                .iter()
                .filter(|ticket_value| {
                    inp.fields
                        .values()
                        .all(|(r1, r2)| !r1.contains(ticket_value) && !r2.contains(ticket_value))
                })
                .sum()
        })
        .sum()
}

fn valid_tickets(puzzle: &Puzzle) -> Vec<&Vec<i32>> {
    puzzle
        .nearby_tickets
        .iter()
        .filter(|ticket| {
            ticket.iter().all(|ticket_value| {
                puzzle
                    .fields
                    .values()
                    .any(|(r1, r2)| r1.contains(ticket_value) || r2.contains(ticket_value))
            })
        })
        .collect()
}

#[aoc(day16, part2)]
fn solve_part2(puzzle: &Puzzle) -> u64 {
    let mapping = find_mapping(puzzle);
    let mut solution = 1;
    for (field_name, cols) in mapping {
        if field_name.starts_with("departure") {
            // we're taking advantage of the fact that while `find_mapping` isn't exhaustive, it's
            // good enough for the columns we care about for the puzzle (that may be intentional)
            if cols.len() > 1 {
                panic!("Sorry, {} still has too many options", field_name);
            }
            solution *= puzzle.your_ticket[cols[0]] as u64;
        }
    }
    solution
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parser() {
        let inp = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";
        let puzzle = input_parse(inp);
        assert_eq!(puzzle.fields.get("class").unwrap(), &(1..4, 5..8));
        assert_eq!(puzzle.fields.get("seat").unwrap(), &(13..41, 45..51));
        assert_eq!(puzzle.your_ticket, vec![7, 1, 14]);
        assert_eq!(
            puzzle.nearby_tickets,
            vec![
                vec![7, 3, 47],
                vec![40, 4, 50],
                vec![55, 2, 20],
                vec![38, 6, 12]
            ]
        );
    }

    #[test]
    fn test_part1() {
        let inp = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";
        assert_eq!(solve_part1(&input_parse(inp)), 71);
    }

    #[test]
    fn test_find_mapping() {
        let inp = "class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";
        let puzzle = input_parse(inp);
        let mapping = find_mapping(&puzzle);
        dbg!(&mapping);
        // assert_eq!(true, false);
        assert_eq!(*mapping.get("row").unwrap(), vec![0]);
        assert_eq!(*mapping.get("class").unwrap(), vec![1]);
        assert_eq!(*mapping.get("seat").unwrap(), vec![2]);
    }
}
