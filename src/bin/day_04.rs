use aoc2024::{collections::grid::Grid, sample};
use lazy_static::lazy_static;
use tracing::debug;

fn main() {
    aoc2024::run(part1, None);
}

lazy_static! {
    static ref DIRECTIONS: Vec<(i32, i32)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1)
    ];
}

fn children(grid: &Grid<char>, x: usize, y: usize, depth: i32) -> Vec<String> {
    let mut result = Vec::new();

    for (dx, dy) in DIRECTIONS.iter() {
        result.push(
            (0..depth)
                .map(|i| (x as i32 + dx * i, y as i32 + dy * i))
                .take_while(|(x, y)| {
                    *x >= 0 && *x < grid.rows() as i32 && *y >= 0 && *y < grid.cols() as i32
                })
                .map(|(x, y)| grid.get(x as usize, y as usize))
                .collect(),
        );
    }

    result
}

fn part1(input: &str) -> String {
    let chars: Grid<char> = input.split('\n').map(|s| s.chars()).collect();

    let mut total = 0;
    for (x, y, _) in chars.enumerate() {
        let words = children(&chars, x, y, 4);
        debug!("{:?}", words);
        total += words.iter().filter(|w| *w == "XMAS").count();
    }

    total.to_string()
}

sample! {
    r"
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX",
    part1 = "18"
}
