use std::collections::BinaryHeap;

use aoc2024::{
    collections::grid::{Direction, Grid, Position},
    sample,
};
use tracing::{debug, info};

fn main() {
    aoc2024::run(part1, None);
}

type Point = (usize, usize);

struct Path<'a> {
    score: usize,
    pos: Position<'a, char>,
    dir: Direction,
}

impl<'a> Path<'a> {
    fn new(score: usize, pos: Position<'a, char>, dir: Direction) -> Self {
        Self { score, pos, dir }
    }
}

impl Ord for Path<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.score.cmp(&self.score)
    }
}

impl PartialOrd for Path<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Path<'_> {
    fn eq(&self, other: &Self) -> bool {
        other.score.eq(&self.score)
    }
}

impl Eq for Path<'_> {}

struct Maze {
    grid: Grid<char>,
    start: Point,
    end: Point,
}

impl Maze {
    fn parse(input: &str) -> Self {
        let grid: Grid<char> = input.lines().map(|l| l.chars()).collect();

        let mut start = (0, 0);
        let mut end = (0, 0);

        for (r, c, v) in grid.enumerate() {
            match v {
                'E' => end = (r, c),
                'S' => start = (r, c),
                _ => (),
            }
        }

        Self { grid, start, end }
    }

    fn get_turn_dirs(dir: Direction) -> Vec<Direction> {
        match dir {
            Direction::North | Direction::South => vec![Direction::East, Direction::West],
            Direction::East | Direction::West => vec![Direction::North, Direction::South],
        }
    }

    fn solve(&self) -> Option<usize> {
        let start_pos = self.grid.position(self.start.0, self.start.1).unwrap();
        let (r_end, c_end) = self.end;

        let mut best = self
            .grid
            .map(|_| [usize::MAX, usize::MAX, usize::MAX, usize::MAX]);

        let mut heap = BinaryHeap::new();
        heap.push(Path::new(0, start_pos, Direction::East));

        while let Some(path) = heap.pop() {
            debug!(
                "At position ({}, {}) with score {}",
                path.pos.row(),
                path.pos.col(),
                path.score
            );

            let best_for_pos = &mut best.get_mut(path.pos.row(), path.pos.col())[path.dir as usize];
            if path.score >= *best_for_pos {
                info!(
                    "Pruning position ({}, {}) due to lower score.",
                    path.pos.row(),
                    path.pos.col()
                );
                continue;
            }
            *best_for_pos = path.score;

            if path.pos.row() == r_end && path.pos.col() == c_end {
                return Some(path.score);
            }

            // Step forward
            if let Some(step) = path.pos.get_neighbor(path.dir) {
                if *step.value() != '#' {
                    heap.push(Path::new(path.score + 1, step, path.dir))
                }
            }

            // Turn
            for turn in Self::get_turn_dirs(path.dir) {
                heap.push(Path::new(path.score + 1000, path.pos, turn))
            }
        }

        None
    }
}

fn part1(input: &str) -> String {
    let maze = Maze::parse(input);
    let score = maze.solve().unwrap();
    score.to_string()
}

sample! {
    r"
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############",
    part1 = "7036"
}
