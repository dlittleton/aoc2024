use std::collections::VecDeque;

use aoc2024::collections::grid::{Grid, Position, CARDINAL_DIRECTIONS};
use tracing::{debug, info};

fn main() {
    aoc2024::run(part1, Some(part2));
}

type Point = (usize, usize);

fn distance(a: &Point, b: &Point) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

struct PathNode {
    point: Point,
    d_start: usize,
    d_end: usize,
}

struct Maze {
    grid: Grid<char>,
    start: Point,
    end: Point,
}

impl Maze {
    fn parse(input: &str) -> Self {
        let grid: Grid<char> = input.lines().map(|l| l.chars()).collect();

        let start = grid.find(&'S').unwrap().into();
        let end = grid.find(&'E').unwrap().into();

        Self { grid, start, end }
    }

    fn end_pos(&self) -> Position<char> {
        self.grid.position(self.end.0, self.end.1).unwrap()
    }

    fn distance_from_end(&self) -> (usize, Vec<PathNode>) {
        let mut lengths = self.grid.map(|_| 0);

        let mut to_visit = VecDeque::new();
        to_visit.push_back((self.end_pos(), 0));

        while let Some((pos, depth)) = to_visit.pop_front() {
            *lengths.get_mut(pos.row(), pos.col()) = depth;

            for n in pos.get_neighbors(&CARDINAL_DIRECTIONS) {
                if !matches!(n.value(), '#' | 'E') && *lengths.get(n.row(), n.col()) == 0 {
                    to_visit.push_back((n, depth + 1));
                }
            }
        }

        let origin = *lengths.get(self.start.0, self.start.1);

        let nodes = self
            .grid
            .enumerate()
            .filter_map(|(r, c, v)| match v {
                '#' => None,
                _ => Some(PathNode {
                    point: (r, c),
                    d_start: origin - lengths.get(r, c),
                    d_end: *lengths.get(r, c),
                }),
            })
            .collect();

        (origin, nodes)
    }

    fn find_cheats(&self, max_cheat: usize) -> Vec<usize> {
        let (origin, nodes) = self.distance_from_end();
        let mut result = Vec::new();

        for a in nodes.iter() {
            for b in nodes.iter() {
                let d = distance(&a.point, &b.point);
                // Only cheat between nodes if the target node is closer to the end.
                if d > 0 && d <= max_cheat && b.d_end < a.d_end {
                    let total_distance = a.d_start + d + b.d_end;
                    debug!(
                        "Path from ({}, {}) to ({}, {}) is length {}",
                        a.point.0, a.point.1, b.point.0, b.point.1, total_distance
                    );
                    result.push(origin - total_distance);
                }
            }
        }

        result
    }
}

fn solve(input: &str, max_cheat: usize, limit: usize) -> String {
    let maze = Maze::parse(input);

    let paths = maze.find_cheats(max_cheat);
    info!("There are {} cheat paths.", paths.len());
    let count = paths.iter().filter(|v| **v >= limit).count();

    count.to_string()
}

fn part1(input: &str) -> String {
    solve(input, 2, 100)
}

fn part2(input: &str) -> String {
    solve(input, 20, 100)
}

#[cfg(test)]
mod test {

    use aoc2024::sample;

    use super::*;

    fn test_part1(input: &str) -> String {
        // Use shorter cutoff for the sample maze
        solve(input, 2, 20)
    }

    fn test_part2(input: &str) -> String {
        solve(input, 20, 70)
    }

    sample! {
        r"
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############",
        test_part1 = "5",
        test_part2 = "41"
    }
}
