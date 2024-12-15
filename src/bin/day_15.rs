use std::collections::VecDeque;

use aoc2024::{
    collections::grid::{Direction, Grid, Position},
    sample,
};
use tracing::{debug, info, trace};

fn main() {
    aoc2024::run(part1, Some(part2));
}

type Swap = ((usize, usize), (usize, usize));

struct Warehouse {
    grid: Grid<char>,
    moves: VecDeque<Direction>,
    wide: bool,
}

impl Warehouse {
    fn parse(input: &str, wide: bool) -> Self {
        let mut lines = input.lines();

        let grid: Grid<char> = lines
            .by_ref()
            .take_while(|l| !l.is_empty())
            .map(|l| {
                l.chars().flat_map(|c| {
                    if wide {
                        match c {
                            '#' => vec!['#', '#'],
                            'O' => vec!['[', ']'],
                            '.' => vec!['.', '.'],
                            '@' => vec!['@', '.'],
                            x => panic!("Unrecognized block: {}", x),
                        }
                    } else {
                        vec![c]
                    }
                })
            })
            .collect();

        let moves = lines
            .flat_map(|l| l.chars())
            .map(|c| match c {
                '^' => Direction::North,
                '>' => Direction::East,
                'v' => Direction::South,
                '<' => Direction::West,
                x => panic!("Unrecognized move: {}", x),
            })
            .collect();

        Self { grid, moves, wide }
    }

    fn get_bot_position(&self) -> Position<char> {
        self.grid
            .enumerate()
            .find_map(|(r, c, v)| match v {
                '@' => self.grid.position(r, c),
                _ => None,
            })
            .expect("Failed to find robot position.")
    }

    fn count_boxes(&self) -> usize {
        self.grid
            .enumerate()
            .filter(|(_, _, v)| matches!(v, 'O' | '['))
            .count()
    }

    fn shift(&mut self, r: usize, c: usize, dir: Direction) -> (usize, usize) {
        // Empty nodes don't need to be shifted. Walls cannot be shifted.
        let val = *self.grid.get(r, c);
        if val == '.' || val == '#' {
            return (r, c);
        }

        let pos = self.grid.position(r, c).unwrap();
        let neighbor = pos.get_neighbor(dir);
        let (r, c) = pos.into();

        if let Some(n) = neighbor {
            let (rn, cn) = n.into();
            self.shift(rn, cn, dir);
            if *self.grid.get(rn, cn) == '.' {
                debug!("Moving {} from ({}, {}) to ({}, {})", val, r, c, rn, cn);
                // Shift is possible, swap and return new position.
                *self.grid.get_mut(rn, cn) = *self.grid.get(r, c);
                *self.grid.get_mut(r, c) = '.';
                return (rn, cn);
            }
        }

        (r, c)
    }

    fn shift_wide(pos: Position<char>, dir: Direction) -> Option<Vec<Swap>> {
        let n1 = pos.get_neighbor(dir);

        match pos.value() {
            '#' => None,
            '.' => Some(Vec::new()),
            '@' => {
                let n1 = n1.unwrap();
                let swap = ((pos.row(), pos.col()), (n1.row(), n1.col()));
                if let Some(mut swaps) = Self::shift_wide(n1, dir) {
                    swaps.push(swap);
                    Some(swaps)
                } else {
                    None
                }
            }
            '[' => {
                let p2 = pos.get_neighbor(Direction::East).unwrap();
                let n1 = n1.unwrap();
                let n2 = n1.get_neighbor(Direction::East).unwrap();

                let swap = ((pos.row(), pos.col()), (n1.row(), n1.col()));
                let swap2 = ((p2.row(), p2.col()), (n2.row(), n2.col()));

                let s1 = Self::shift_wide(n1, dir);
                let s2 = Self::shift_wide(n2, dir);
                if let (Some(mut swaps), Some(other)) = (s1, s2) {
                    debug!("S1 {:?}, S2 {:?}", swaps, other);
                    swaps.extend_from_slice(&other);
                    swaps.push(swap);
                    swaps.push(swap2);
                    Some(swaps)
                } else {
                    None
                }
            }
            ']' => {
                let p2 = pos.get_neighbor(Direction::West).unwrap();
                let n1 = n1.unwrap();
                let n2 = n1.get_neighbor(Direction::West).unwrap();

                let swap = ((pos.row(), pos.col()), (n1.row(), n1.col()));
                let swap2 = ((p2.row(), p2.col()), (n2.row(), n2.col()));

                let s1 = Self::shift_wide(n1, dir);
                let s2 = Self::shift_wide(n2, dir);
                if let (Some(mut swaps), Some(other)) = (s1, s2) {
                    debug!("S1 {:?}, S2 {:?}", swaps, other);
                    swaps.extend_from_slice(&other);
                    swaps.push(swap);
                    swaps.push(swap2);
                    Some(swaps)
                } else {
                    None
                }
            }
            x => panic!("Unexpected block in shift {}", x),
        }
    }

    fn run(&mut self) {
        let (mut rbot, mut cbot) = self.get_bot_position().into();

        let box_count = self.count_boxes();

        while let Some(dir) = self.moves.pop_front() {
            info!("Moving {:?}", dir);

            if self.wide && matches!(dir, Direction::North | Direction::South) {
                let pos = self.grid.position(rbot, cbot).unwrap();
                match Self::shift_wide(pos, dir) {
                    Some(mut swaps) => {
                        info!("There are {} swaps required.", swaps.len());

                        swaps.sort();
                        if matches!(dir, Direction::South) {
                            swaps.reverse();
                        }

                        swaps.dedup();
                        info!("There are {} unique swaps.", swaps.len());

                        for ((r, c), (rn, cn)) in swaps {
                            debug!("Moving from ({}, {}) to ({}, {})", r, c, rn, cn);
                            // Shift is possible, swap and return new position.
                            *self.grid.get_mut(rn, cn) = *self.grid.get(r, c);
                            *self.grid.get_mut(r, c) = '.';
                        }

                        (rbot, cbot) = self
                            .grid
                            .position(rbot, cbot)
                            .unwrap()
                            .get_neighbor(dir)
                            .unwrap()
                            .into();
                    }
                    None => {
                        debug!("No move possible.");
                    }
                }
            } else {
                (rbot, cbot) = self.shift(rbot, cbot, dir);
            }
            self.trace_grid();

            let new_count = self.count_boxes();
            if box_count != new_count {
                panic!("Lost a box!");
            }
        }
    }

    fn score(&self) -> usize {
        self.grid
            .enumerate()
            .filter_map(|(r, c, v)| match v {
                'O' | '[' => Some(100 * r + c),
                _ => None,
            })
            .sum()
    }

    fn trace_grid(&self) {
        if tracing::event_enabled!(tracing::Level::TRACE) {
            trace!("Grid state");
            for row in self.grid.row_wise_iter() {
                let output: String = row.collect();
                trace!("{}", output);
            }
        }
    }
}

fn part1(input: &str) -> String {
    let mut warehouse = Warehouse::parse(input, false);
    info!(
        "Warehouse size is {}, {}",
        warehouse.grid.rows(),
        warehouse.grid.cols()
    );
    warehouse.run();
    warehouse.score().to_string()
}

fn part2(input: &str) -> String {
    let mut warehouse = Warehouse::parse(input, true);
    info!(
        "Warehouse size is {}, {}",
        warehouse.grid.rows(),
        warehouse.grid.cols()
    );
    warehouse.run();
    warehouse.score().to_string()
}

sample! {
    r"
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^",
    part1 = "10092",
    part2 = "9021"
}

mod short_sample {
    use super::*;

    sample! {
        r"
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<",
        part1 = "2028"
    }
}

mod wide_sample {
    use super::*;

    sample! {
        r"
#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^",
        part2 = "618"
    }
}
