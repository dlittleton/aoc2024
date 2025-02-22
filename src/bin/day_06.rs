use std::collections::HashSet;

use aoc2024::{collections::grid::Grid, sample};

fn main() {
    aoc2024::run(part1, Some(part2));
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    row: usize,
    col: usize,
    dir: (i32, i32),
}

fn find_start(grid: &Grid<char>) -> Position {
    grid.enumerate()
        .find_map(|(r, c, v)| match *v {
            '^' => Some(Position {
                row: r,
                col: c,
                dir: (-1, 0),
            }),
            _ => None,
        })
        .expect("Failed to find start position.")
}

fn is_exit(grid: &Grid<char>, pos: &Position) -> bool {
    match (pos.row, pos.col, pos.dir) {
        (0, _, (-1, 0)) => true,
        (r, _, (1, 0)) if r == grid.rows() - 1 => true,
        (_, 0, (0, -1)) => true,
        (_, c, (0, 1)) if c == grid.cols() - 1 => true,
        _ => false,
    }
}

fn advance(grid: &Grid<char>, pos: &Position) -> Option<Position> {
    if is_exit(grid, pos) {
        return None;
    }

    let mut dir = pos.dir;

    loop {
        let r = (pos.row as i32 + dir.0) as usize;
        let c = (pos.col as i32 + dir.1) as usize;
        let val = grid.get(r, c);

        if *val != '#' {
            return Some(Position {
                row: r,
                col: c,
                dir,
            });
        }

        dir = match dir {
            (-1, 0) => (0, 1),
            (0, 1) => (1, 0),
            (1, 0) => (0, -1),
            (0, -1) => (-1, 0),
            (x, y) => panic!("Unexpected direction {} {}", x, y),
        };
    }
}

fn part1(input: &str) -> String {
    let grid: Grid<char> = input.split('\n').map(|s| s.chars()).collect();

    let mut pos = Some(find_start(&grid));
    let mut seen = HashSet::new();

    while let Some(p) = pos {
        seen.insert((p.row, p.col));
        pos = advance(&grid, &p);
    }

    seen.len().to_string()
}

fn is_loop(grid: &Grid<char>, start: &Position, row: usize, col: usize) -> bool {
    // Can't replace a non-empty tile
    if *grid.get(row, col) != '.' {
        return false;
    }

    let mut grid = grid.clone();
    *grid.get_mut(row, col) = '#';

    let mut seen = HashSet::new();

    let mut position = Some(*start);
    while let Some(pos) = position {
        if seen.contains(&pos) {
            return true;
        }
        seen.insert(pos);
        position = advance(&grid, &pos);
    }

    false
}

fn part2(input: &str) -> String {
    let grid: Grid<char> = input.split('\n').map(|s| s.chars()).collect();

    let start = find_start(&grid);

    let count = grid
        .enumerate()
        .filter(|(r, c, _)| is_loop(&grid, &start, *r, *c))
        .count();

    count.to_string()
}

sample! {
    r"
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...",
    part1 = "41",
    part2 = "6"
}
