use aoc2024::{collections::grid::Grid, sample};
use tracing::info;

fn main() {
    aoc2024::run(part1, None);
}

fn parse_grids(input: &str) -> Vec<Grid<char>> {
    let mut lines = input.lines().peekable();

    let mut result = Vec::new();
    while lines.peek().is_some() {
        result.push(
            lines
                .by_ref()
                .take_while(|l| !l.is_empty())
                .map(|l| l.chars())
                .collect(),
        );
    }

    result
}

fn heights(grid: &Grid<char>) -> Vec<usize> {
    grid.col_wise_iter()
        .map(|col| col.filter(|c| matches!(c, '#')).count())
        .collect()
}

fn is_lock(grid: &Grid<char>) -> bool {
    grid.row_wise_iter()
        .next()
        .unwrap()
        .all(|c| matches!(c, '#'))
}

fn part1(input: &str) -> String {
    let schematics = parse_grids(input);

    let (locks, keys): (Vec<_>, Vec<_>) = schematics.into_iter().partition(is_lock);

    let cutoff = locks.first().unwrap().rows();
    info!("Cutoff is {}", cutoff);

    let lock_heights: Vec<_> = locks.iter().map(heights).collect();
    let key_heights: Vec<_> = keys.iter().map(heights).collect();

    let mut total = 0;
    for l in lock_heights.iter() {
        for k in key_heights.iter() {
            if l.iter()
                .zip(k.iter())
                .map(|(a, b)| a + b)
                .all(|v| v <= cutoff)
            {
                total += 1;
            }
        }
    }

    info!("There are {} keys.", keys.len());
    info!("There are {} locks.", locks.len());

    total.to_string()
}

sample! {
    r"
#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####",
    part1 = "3"
}
