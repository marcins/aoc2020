use aoc_runner_derive::aoc;
use rayon::prelude::*;
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

type Grid = Vec<GridValue>;

struct SeatMap {
    grid: Grid,
    width: usize,
    height: usize,
    seating_algo: SeatingAlgo,
    coords: Vec<(usize, usize)>,
}

impl SeatMap {
    pub fn new(raw_map: &str, seating_algo: SeatingAlgo) -> Self {
        let grid: Grid = raw_map
            .lines()
            .map(|line| line.trim())
            .flat_map(|line| {
                line.chars()
                    .map(|c| match c {
                        'L' => GridValue::Empty,
                        '.' => GridValue::Floor,
                        _ => panic!("Unexpected: {}", c),
                    })
                    .collect::<Vec<GridValue>>()
            })
            .collect();

        let height = raw_map.lines().count();
        let width = raw_map.lines().nth(0).unwrap().len();
        let mut coords = Vec::new();
        for y in 0..height {
            for x in 0..width {
                coords.push((x, y));
            }
        }

        Self {
            grid,
            width,
            height,
            seating_algo: seating_algo,
            coords,
        }
    }
    fn get(&self, x: usize, y: usize) -> GridValue {
        self.grid[(y * self.width) + x]
    }

    fn find_occupied_seat(
        &self,
        x: usize,
        y: usize,
        x_step: isize,
        y_step: isize,
        range: usize,
    ) -> bool {
        let mut xx = x as isize;
        let mut yy = y as isize;
        let mut c = 0;
        while c < range {
            xx += x_step;
            yy += y_step;

            if xx < 0 || xx as usize >= self.width || yy < 0 || yy as usize >= self.height {
                return false;
            }

            match self.get(xx as usize, yy as usize) {
                GridValue::Occupied => return true,
                GridValue::Empty => return false,
                _ => (),
            };
            c += 1;
        }
        return false;
    }

    fn adjacent_occupied(&self, x: usize, y: usize, range: usize) -> usize {
        let steps = vec![
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];
        steps
            .iter()
            .filter(|(x_step, y_step)| {
                self.find_occupied_seat(x, y, *x_step as isize, *y_step as isize, range)
            })
            .count()
    }

    fn advance(&mut self) -> bool {
        let new_grid: Vec<GridValue> = self
            .coords
            .par_iter()
            .map(|(x, y)| {
                let current = self.get(*x, *y);
                if current == GridValue::Floor {
                    return current;
                }

                let (range, occupancy_threshold) = match self.seating_algo {
                    SeatingAlgo::Near => (1, 4),
                    SeatingAlgo::Far => (usize::MAX, 5),
                };

                let adjacent_occupied = self.adjacent_occupied(*x, *y, range);

                if current == GridValue::Occupied && adjacent_occupied >= occupancy_threshold {
                    // -> empty
                    GridValue::Empty
                } else if current == GridValue::Empty && adjacent_occupied == 0 {
                    // -> occupied
                    GridValue::Occupied
                } else {
                    current
                }
            })
            .collect();

        let changed = !new_grid
            .iter()
            .zip(self.grid.iter())
            .all(|(v1, v2)| v1 == v2);

        if changed {
            self.grid = new_grid;
        }
        changed
    }

    fn occupied_seats(&self) -> usize {
        self.grid
            .iter()
            .filter(|v| **v == GridValue::Occupied)
            .count()
    }

    fn pretty(&self) -> String {
        let mut out = String::new();
        for (x, y) in self.coords.iter() {
            if *x == 0 && *y > 0 {
                out.push('\n');
            }
            let c = match self.get(*x, *y) {
                GridValue::Floor => '.',
                GridValue::Empty => 'L',
                GridValue::Occupied => '#',
            };
            out.push(c);
        }
        out
    }
}

impl fmt::Debug for SeatMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.pretty())
    }
}

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

#[aoc(day11, part2)]
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
                GridValue::Empty,
                GridValue::Floor,
                GridValue::Empty,
                GridValue::Floor,
                GridValue::Empty,
                GridValue::Floor
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
