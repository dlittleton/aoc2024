use std::collections::VecDeque;

use aoc2024::{
    collections::grid::{Direction, Grid, Position},
    sample,
};
use tracing::{debug, info};

fn main() {
    aoc2024::run(part1, None);
}

struct Warehouse {
    grid: Grid<char>,
    moves: VecDeque<Direction>,
}

impl Warehouse {
    fn parse(input: &str) -> Self {
        let mut lines = input.lines();

        let grid: Grid<char> = lines
            .by_ref()
            .take_while(|l| !l.is_empty())
            .map(|l| l.chars())
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

        Self { grid, moves }
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

    fn run(&mut self) {
        let (mut r, mut c) = self.get_bot_position().into();

        while let Some(dir) = self.moves.pop_front() {
            info!("Moving {:?}", dir);
            (r, c) = self.shift(r, c, dir);
        }
    }

    fn score(&self) -> usize {
        self.grid
            .enumerate()
            .filter_map(|(r, c, v)| match v {
                'O' => Some(100 * r + c),
                _ => None,
            })
            .sum()
    }
}

fn part1(input: &str) -> String {
    let mut warehouse = Warehouse::parse(input);
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
    part1 = "10092"
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
