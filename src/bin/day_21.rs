use std::collections::HashMap;

use aoc2024::{
    collections::grid::{Direction, Grid, Position},
    input::get_all_numbers,
    sample,
};
use cached::proc_macro::cached;
use itertools::Itertools;
use lazy_static::lazy_static;
use tracing::{debug, info};

fn main() {
    aoc2024::run(part1, Some(part2));
}

const NUMERIC_KEYPAD: &str = r"
789
456
123
-0A";

const DIRECTIONAL_KEYPAD: &str = r"
-^A
<v>";

lazy_static! {
    static ref NUMERIC_GRID: Grid<char> =
        NUMERIC_KEYPAD.trim().lines().map(|l| l.chars()).collect();
    static ref DIRECTIONAL_GRID: Grid<char> = DIRECTIONAL_KEYPAD
        .trim()
        .lines()
        .map(|l| l.chars())
        .collect();
    static ref NUMERIC_POSITIONS: HashMap<char, (usize, usize)> = NUMERIC_GRID
        .enumerate()
        .map(|(r, c, v)| (*v, (r, c)))
        .collect();
    static ref DIRECTIONAL_POSITIONS: HashMap<char, (usize, usize)> = DIRECTIONAL_GRID
        .enumerate()
        .map(|(r, c, v)| (*v, (r, c)))
        .collect();
}

fn get_paths(mut r: usize, mut c: usize, rt: usize, ct: usize) -> Vec<Direction> {
    let mut moves = Vec::new();

    while r < rt {
        moves.push(Direction::South);
        r += 1;
    }

    while r > rt {
        moves.push(Direction::North);
        r -= 1;
    }

    while c < ct {
        moves.push(Direction::East);
        c += 1;
    }

    while c > ct {
        moves.push(Direction::West);
        c -= 1;
    }

    moves
}

fn is_valid_path(start: Position<char>, path: &Vec<Direction>) -> bool {
    let mut pos = start;
    for d in path {
        pos = pos.get_neighbor(*d).unwrap();
        if *pos.value() == '-' {
            return false;
        }
    }
    true
}

fn to_move_string(path: &Vec<Direction>) -> String {
    let mut result = String::new();

    for d in path {
        result.push(match d {
            Direction::North => '^',
            Direction::East => '>',
            Direction::South => 'v',
            Direction::West => '<',
        });
    }

    result.push('A');

    result
}

#[cached]
fn expand(moves: String, depth: usize) -> usize {
    if depth == 0 {
        return moves.len();
    }

    let mut length = 0;

    let mut last_char = 'A';
    for c in moves.chars() {
        length += get_shortest_dir_path(last_char, c, depth);
        last_char = c;
    }

    length
}

#[cached]
fn get_shortest_dir_path(start: char, end: char, depth: usize) -> usize {
    debug!("Looking from {} to {}", start, end);
    let (r, c) = DIRECTIONAL_POSITIONS.get(&start).unwrap();
    let (r_end, c_end) = DIRECTIONAL_POSITIONS.get(&end).unwrap();

    let start_pos = DIRECTIONAL_GRID.position(*r, *c).unwrap();

    let mut result = Vec::new();
    let paths = get_paths(*r, *c, *r_end, *c_end);
    for m in paths.iter().copied().permutations(paths.len()).unique() {
        debug!("Possible path {:?}", m);
        if !is_valid_path(start_pos, &m) {
            continue;
        }

        let move_string = to_move_string(&m);
        result.push(expand(move_string, depth - 1));
    }

    *result.iter().min().unwrap()
}

fn get_shortest_path(start: char, end: char, initial_depth: usize) -> usize {
    info!("Looking from {} to {}", start, end);
    let (r, c) = NUMERIC_POSITIONS.get(&start).unwrap();
    let (r_end, c_end) = NUMERIC_POSITIONS.get(&end).unwrap();

    let start_pos = NUMERIC_GRID.position(*r, *c).unwrap();

    let mut result = Vec::new();
    let paths = get_paths(*r, *c, *r_end, *c_end);
    for m in paths.iter().copied().permutations(paths.len()).unique() {
        debug!("Possible path {:?}", m);
        if !is_valid_path(start_pos, &m) {
            continue;
        }

        let move_string = to_move_string(&m);
        result.push(expand(move_string, initial_depth));
    }

    *result.iter().min().unwrap()
}

fn solve(line: &str, initial_depth: usize) -> (usize, usize) {
    let numeric_value = *get_all_numbers::<usize>(line).first().unwrap();

    let mut total = 0;
    let mut last_char = 'A';
    for c in line.chars() {
        total += get_shortest_path(last_char, c, initial_depth);
        last_char = c;
    }

    (numeric_value, total)
}

fn part1(input: &str) -> String {
    let result: Vec<_> = input
        .lines()
        .map(|l| solve(l, 2))
        .map(|(n, l)| n * l)
        .collect();

    let total: usize = result.iter().sum();
    total.to_string()
}

fn part2(input: &str) -> String {
    let result: Vec<_> = input
        .lines()
        .map(|l| solve(l, 25))
        .map(|(n, l)| n * l)
        .collect();

    let total: usize = result.iter().sum();
    total.to_string()
}

sample! {
    r"
029A
980A
179A
456A
379A",
    part1 = "126384"
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test_log::test(rstest)]
    #[case("029A", 29, 68)]
    #[case("980A", 980, 60)]
    #[case("179A", 179, 68)]
    #[case("456A", 456, 64)]
    #[case("379A", 379, 64)]
    fn test_expansion(#[case] input: &str, #[case] numeric: usize, #[case] length: usize) {
        let (n, l) = solve(input, 2);
        assert_eq!(numeric, n);
        assert_eq!(length, l);
    }
}
