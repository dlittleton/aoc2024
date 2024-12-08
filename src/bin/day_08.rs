use std::collections::{HashMap, HashSet};

use aoc2024::{collections::grid::Grid, sample};
use tracing::info;

fn main() {
    aoc2024::run(part1, None);
}

#[derive(PartialEq, Eq, Hash)]
struct Position {
    r: i32,
    c: i32,
}

struct Antennas {
    grid: Grid<char>,
    positions: HashMap<char, Vec<Position>>,
}

impl Antennas {
    fn parse(input: &str) -> Self {
        let grid: Grid<char> = input.split('\n').map(|s| s.chars()).collect();
        let mut positions: HashMap<char, Vec<Position>> = HashMap::new();

        for (r, c, v) in grid.enumerate() {
            if *v == '.' {
                continue;
            }

            positions.entry(*v).or_default().push(Position {
                r: r as i32,
                c: c as i32,
            });
        }

        Antennas { grid, positions }
    }

    fn signals(&self) -> impl Iterator<Item = char> + use<'_> {
        self.positions.keys().copied()
    }

    fn get_anti_nodes(&self, signal: char) -> Vec<Position> {
        let mut result = Vec::new();

        let antenna_positions = self.positions.get(&signal).expect("Unknown signal");

        for (i, p1) in antenna_positions.iter().enumerate() {
            for p2 in antenna_positions[i + 1..].iter() {
                info!(
                    "Calculating Antinodes for signal {}, Positions ({} {}) ({} {})",
                    signal, p1.r, p1.c, p2.r, p2.c
                );

                let dr = p2.r - p1.r;
                let dc = p2.c - p1.c;

                result.push(Position {
                    r: p2.r + dr,
                    c: p2.c + dc,
                });

                result.push(Position {
                    r: p1.r - dr,
                    c: p1.c - dc,
                });
            }
        }

        result.retain(|pos| {
            pos.r >= 0
                && pos.r < self.grid.rows() as i32
                && pos.c >= 0
                && pos.c < self.grid.cols() as i32
        });

        result
    }
}

fn part1(input: &str) -> String {
    let antennas = Antennas::parse(input);

    let antinodes: HashSet<Position> = antennas
        .signals()
        .flat_map(|c| antennas.get_anti_nodes(c))
        .collect();

    antinodes.len().to_string()
}

sample! {
    r"
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............",
    part1 = "14"
}
