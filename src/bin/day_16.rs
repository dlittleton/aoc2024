use std::collections::{BinaryHeap, HashSet};

use aoc2024::{
    collections::grid::{Direction, Grid, Position},
    sample,
};
use tracing::{debug, info};

fn main() {
    aoc2024::run(part1, Some(part2));
}

type Point = (usize, usize);

struct Path<'a> {
    score: usize,
    pos: Position<'a, char>,
    dir: Direction,
    nodes: Vec<Point>,
}

impl<'a> Path<'a> {
    fn new(score: usize, pos: Position<'a, char>, dir: Direction) -> Self {
        let nodes = vec![(pos.row(), pos.col())];
        Self {
            score,
            pos,
            dir,
            nodes,
        }
    }

    fn step(&self) -> Option<Self> {
        self.pos.get_neighbor(self.dir).map(|p| {
            let mut nodes = self.nodes.clone();
            nodes.push((p.row(), p.col()));
            Self {
                score: self.score + 1,
                pos: p,
                dir: self.dir,
                nodes,
            }
        })
    }

    fn turn(&self, dir: Direction) -> Self {
        Self {
            score: self.score + 1000,
            pos: self.pos,
            dir,
            nodes: self.nodes.clone(),
        }
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

    fn solve(&self) -> (usize, Vec<Vec<Point>>) {
        let start_pos = self.grid.position(self.start.0, self.start.1).unwrap();
        let (r_end, c_end) = self.end;

        let mut best = self
            .grid
            .map(|_| [usize::MAX, usize::MAX, usize::MAX, usize::MAX]);

        let mut low_score = None;
        let mut paths = Vec::new();

        let mut heap = BinaryHeap::new();
        heap.push(Path::new(0, start_pos, Direction::East));

        while let Some(path) = heap.pop() {
            debug!(
                "At position ({}, {}) with score {}",
                path.pos.row(),
                path.pos.col(),
                path.score
            );

            if let Some(low) = low_score {
                if path.score > low {
                    continue;
                }
            }

            let best_for_pos = &mut best.get_mut(path.pos.row(), path.pos.col())[path.dir as usize];
            if path.score > *best_for_pos {
                info!(
                    "Pruning position ({}, {}) due to lower score.",
                    path.pos.row(),
                    path.pos.col()
                );
                continue;
            }
            *best_for_pos = path.score;

            if path.pos.row() == r_end && path.pos.col() == c_end {
                low_score = Some(path.score);
                paths.push(path.nodes);
                continue;
            }

            // Step forward
            if let Some(step) = path.step() {
                if *step.pos.value() != '#' {
                    heap.push(step);
                }
            }

            // Turn
            for turn in Self::get_turn_dirs(path.dir) {
                heap.push(path.turn(turn));
            }
        }

        (low_score.unwrap(), paths)
    }
}

fn part1(input: &str) -> String {
    let maze = Maze::parse(input);
    let score = maze.solve().0;
    score.to_string()
}

fn part2(input: &str) -> String {
    let maze = Maze::parse(input);
    let paths = maze.solve().1;

    info!("There are {} paths.", paths.len());

    let unique: HashSet<Point> = paths.iter().flat_map(|p| p.iter()).copied().collect();

    unique.len().to_string()
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
    part1 = "7036",
    part2 = "45"
}
