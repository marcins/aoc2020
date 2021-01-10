use aoc_runner_derive::{aoc, aoc_generator};

type Map = Vec<Vec<u8>>;

#[aoc_generator(day3)]
fn input_parse(input: &str) -> Map {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => 0,
                    '#' => 1,
                    _ => panic!("Unexpected character: {}", c),
                })
                .collect()
        })
        .collect()
}

fn get(map: &Map, x: usize, y: usize) -> i8 {
    if y >= map.len() {
        return -1;
    }
    map[y][x % map[0].len()] as i8
}

fn traverse(map: &Map, step_x: usize, step_y: usize) -> i64 {
    let mut x = 0;
    let mut y = 0;
    let mut trees = 0;
    loop {
        let v = get(&map, x, y);
        if v == 1 {
            trees += 1;
        } else if v == -1 {
            return trees;
        }
        x += step_x;
        y += step_y;
    }
}

#[aoc(day3, part1)]
fn solve_part1(map: &Map) -> i64 {
    traverse(map, 3, 1)
}

#[aoc(day3, part2)]
fn solve_part2(map: &Map) -> i64 {
    traverse(map, 1, 1)
        * traverse(map, 3, 1)
        * traverse(map, 5, 1)
        * traverse(map, 7, 1)
        * traverse(map, 1, 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        assert_eq!(
            input_parse(".#.\n#.#\n"),
            vec![vec![0, 1, 0], vec![1, 0, 1]]
        );
    }

    #[test]
    fn lookup() {
        let sample: Map = vec![vec![0, 1, 0], vec![1, 0, 1]];
        assert_eq!(get(&sample, 0, 0), 0);
        assert_eq!(get(&sample, 0, 100), -1);
        assert_eq!(get(&sample, 4, 0), 1);
    }

    #[test]
    fn part1() {
        let inp = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
        assert_eq!(solve_part1(&input_parse(&inp)), 7)
    }

    #[test]
    fn part2() {
        let inp = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
        assert_eq!(solve_part2(&input_parse(&inp)), 336)
    }
}
