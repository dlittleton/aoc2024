use std::collections::{HashSet, VecDeque};

use aoc2024::collections::grid::{Grid, Position, CARDINAL_DIRECTIONS};
use tracing::info;

fn main() {
    aoc2024::run(part1, None);
}

type Point = (usize, usize);

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

    fn start_pos(&self) -> Position<char> {
        self.grid.position(self.start.0, self.start.1).unwrap()
    }

    fn search(&self, skip: Option<Point>) -> usize {
        let mut seen: HashSet<Point> = HashSet::new();
        let mut to_visit = VecDeque::new();

        to_visit.push_back((self.start_pos(), 0));

        while let Some((pos, depth)) = to_visit.pop_front() {
            if self.end == pos.into() {
                return depth;
            }

            if seen.contains(&pos.into()) {
                continue;
            }

            seen.insert(pos.into());
            for n in pos.get_neighbors(&CARDINAL_DIRECTIONS) {
                if *n.value() != '#' || skip.is_some_and(|v| v == n.into()) {
                    to_visit.push_back((n, depth + 1));
                }
            }
        }

        panic!("Unable to reach exit.");
    }

    fn try_cheats(&self) -> Vec<usize> {
        let mut result = Vec::new();

        for (r, c, v) in self.grid.enumerate() {
            if matches!(v, '#') {
                result.push(self.search(Some((r, c))));
            }
        }

        result
    }
}

fn solve(input: &str, limit: usize) -> String {
    let maze = Maze::parse(input);

    let original = maze.search(None);
    info!("Original path length: {}", original);

    let paths = maze.try_cheats();
    info!("There are {} cheat paths.", paths.len());

    let good = paths.iter().filter(|v| (original - **v) >= limit).count();
    info!("There are {} paths that save at least {}", good, limit);

    good.to_string()
}

fn part1(input: &str) -> String {
    solve(input, 100)
}

#[cfg(test)]
mod test {

    use aoc2024::sample;

    use super::*;

    fn test_part_1(input: &str) -> String {
        // Use shorter cutoff for the sample maze
        solve(input, 20)
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
        test_part_1 = "5"
    }
}
