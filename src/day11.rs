use aoc_runner_derive::aoc;
use std::fmt;

#[derive(Debug, PartialEq, Copy, Clone)]
enum GridValue {
    Floor,
    Empty,
    Occupied,
}

enum SeatingAlgo {
    Near,
    Far,
}

struct SeatMap {
    grid: Grid,
    width: usize,
    height: usize,
    _seating_algo: SeatingAlgo,
}

impl SeatMap {
    pub fn new(raw_map: &str, seating_algo: SeatingAlgo) -> Self {
        let grid: Grid = raw_map
            .lines()
            .map(|line| line.trim())
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        'L' => GridValue::Empty,
                        '.' => GridValue::Floor,
                        _ => panic!("Unexpected: {}", c),
                    })
                    .collect()
            })
            .collect();

        let height = grid.len();
        let width = grid[0].len();
        Self {
            grid,
            width,
            height,
            _seating_algo: seating_algo,
        }
    }
    fn get(&self, x: usize, y: usize) -> GridValue {
        self.grid[y][x]
    }

    fn advance(&mut self) -> bool {
        let old_grid = self.grid.clone();
        for y in 0..self.height {
            for x in 0..self.width {
                let current = old_grid[y][x];
                if current == GridValue::Floor {
                    continue;
                }
                // check adjacent squares
                let mut adjacent_occupied = 0;
                for xx in x as isize - 1..=x as isize + 1 {
                    for yy in y as isize - 1..=y as isize + 1 {
                        if (xx == x as isize && yy == y as isize)
                            || xx < 0
                            || xx as usize >= self.width
                            || yy < 0
                            || yy as usize >= self.height
                        {
                            //
                        } else if old_grid[yy as usize][xx as usize] == GridValue::Occupied {
                            adjacent_occupied += 1;
                        }
                    }
                }
                if current == GridValue::Occupied && adjacent_occupied >= 4 {
                    // -> empty
                    self.grid[y][x] = GridValue::Empty;
                } else if current == GridValue::Empty && adjacent_occupied == 0 {
                    // -> occupied
                    self.grid[y][x] = GridValue::Occupied;
                }
            }
        }
        !old_grid
            .iter()
            .flat_map(|v| v)
            .zip(self.grid.iter().flat_map(|v| v))
            .all(|(v1, v2)| v1 == v2)
    }

    fn occupied_seats(&self) -> usize {
        self.grid
            .iter()
            .flat_map(|v| v)
            .filter(|v| **v == GridValue::Occupied)
            .count()
    }

    fn pretty(&self) -> String {
        let mut out = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                if x == 0 && y > 0 {
                    out.push('\n');
                }
                let c = match self.get(x, y) {
                    GridValue::Floor => '.',
                    GridValue::Empty => 'L',
                    GridValue::Occupied => '#',
                };
                out.push(c);
            }
        }
        out
    }
}

impl fmt::Debug for SeatMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.pretty())
    }
}

type Grid = Vec<Vec<GridValue>>;

// #[aoc_generator(day11)]
// fn parse_input(inp: &str) -> SeatMap {
//     SeatMap::new(inp)
// }

#[aoc(day11, part1)]
fn solve_part1(inp: &str) -> usize {
    let mut map = SeatMap::new(inp, SeatingAlgo::Near);
    loop {
        let changed = map.advance();
        if !changed {
            break;
        }
    }
    map.occupied_seats()
}

fn solve_part2(inp: &str) -> usize {
    let mut map = SeatMap::new(inp, SeatingAlgo::Far);
    loop {
        let changed = map.advance();
        if !changed {
            break;
        }
    }
    map.occupied_seats()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parser() {
        let input = "L.L
.L.";
        let seat_map = SeatMap::new(input, SeatingAlgo::Near);
        assert_eq!(
            seat_map.grid,
            vec![
                vec![GridValue::Empty, GridValue::Floor, GridValue::Empty],
                vec![GridValue::Floor, GridValue::Empty, GridValue::Floor]
            ]
        )
    }
    #[test]
    fn test_advance() {
        let input = "LLL
LLL
LLL";
        let mut map = SeatMap::new(input, SeatingAlgo::Near);
        map.advance();
        assert_eq!(
            map.pretty(),
            "###
###
###"
        );
        println!("{:?}", map.pretty());
        println!("----");
        map.advance();
        println!("{:?}", map.pretty());
        assert_eq!(
            map.pretty(),
            "#L#
LLL
#L#"
        );
    }

    #[test]
    fn test_sample_small() {
        let input = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
        let mut map = SeatMap::new(input, SeatingAlgo::Near);
        map.advance();
        let round1 = "#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##";
        assert_eq!(map.pretty(), round1);
        map.advance();
        let round2 = "#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##";
        assert_eq!(map.pretty(), round2);
    }

    #[test]
    fn test_sample() {
        let input = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
        assert_eq!(solve_part1(input), 37);
    }

    #[test]
    fn test_sample_part2() {
        let input = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
        assert_eq!(solve_part2(input), 26);
    }
}
