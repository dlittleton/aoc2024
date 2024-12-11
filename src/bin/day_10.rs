use std::collections::HashSet;

use aoc2024::{
    collections::grid::{Grid, Position, CARDINAL_DIRECTIONS},
    sample,
};

fn main() {
    aoc2024::run(part1, None);
}

struct TrailMap {
    grid: Grid<i32>,
}

impl TrailMap {
    fn parse(input: &str) -> Self {
        let grid = input
            .split('\n')
            .map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as i32))
            .collect();

        Self { grid }
    }

    fn score_trailheads(&self) -> usize {
        self.grid
            .enumerate()
            .filter_map(|(r, c, v)| match v {
                0 => {
                    let mut peaks = HashSet::new();
                    let pos = self.grid.position(r, c).unwrap();

                    Self::find_peaks(&pos, &mut peaks);
                    Some(peaks.len())
                }
                _ => None,
            })
            .sum()
    }

    fn find_peaks(pos: &Position<i32>, peaks: &mut HashSet<(usize, usize)>) {
        if *pos.value() == 9 {
            peaks.insert((pos.row(), pos.col()));
            return;
        }

        for dir in CARDINAL_DIRECTIONS.iter() {
            if let Some(next) = pos.get_neighbor(*dir) {
                if *next.value() == *pos.value() + 1 {
                    Self::find_peaks(&next, peaks);
                }
            }
        }
    }
}

fn part1(input: &str) -> String {
    let map = TrailMap::parse(input);
    map.score_trailheads().to_string()
}

sample! {
    r"
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732",
    part1 = "36"
}
