use aoc2024::{input::get_all_numbers, sample};
use tracing::debug;

fn main() {
    aoc2024::run(part1, None);
}

fn is_safe(levels: &[i32]) -> bool {
    let mut diffs = Vec::new();
    for i in 1..levels.len() {
        diffs.push(levels[i] - levels[i - 1]);
    }

    let sign = diffs[0].signum();

    let result = diffs
        .into_iter()
        .all(|d| d.signum() == sign && d.abs() >= 1 && d.abs() <= 3);

    debug!("{:?} is {}", levels, result);
    result
}

fn part1(input: &str) -> String {
    let count = input
        .split('\n')
        .map(|s| get_all_numbers::<i32>(s))
        .filter(|v| is_safe(&v))
        .count();

    count.to_string()
}

sample! {
    r"
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
    part1 = "2"
}
