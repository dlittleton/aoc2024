use std::collections::HashMap;

use aoc2024::{
    collections::grid::{Grid, Position, CARDINAL_DIRECTIONS},
    sample,
};

fn main() {
    aoc2024::run(part1, Some(part2));
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

    fn score_trailheads(&self) -> Vec<HashMap<(usize, usize), usize>> {
        self.grid
            .enumerate()
            .filter_map(|(r, c, v)| match v {
                0 => {
                    let mut peaks = HashMap::new();
                    let pos = self.grid.position(r, c).unwrap();

                    Self::find_peaks(&pos, &mut peaks);
                    Some(peaks)
                }
                _ => None,
            })
            .collect()
    }

    fn find_peaks(pos: &Position<i32>, peaks: &mut HashMap<(usize, usize), usize>) {
        if *pos.value() == 9 {
            *peaks.entry((pos.row(), pos.col())).or_default() += 1;
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
    let total: usize = map
        .score_trailheads()
        .iter()
        .map(|scores| scores.len())
        .sum();

    total.to_string()
}

fn part2(input: &str) -> String {
    let map = TrailMap::parse(input);
    let total: usize = map
        .score_trailheads()
        .iter()
        .map(|scores| scores.values().sum::<usize>())
        .sum();

    total.to_string()
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
    part1 = "36",
    part2 = "81"
}
