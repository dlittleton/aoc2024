use aoc2024::{input::get_all_numbers, sample};
use tracing::debug;

fn main() {
    aoc2024::run(part1, Some(part2));
}

fn check_diffs(diffs: &[i32]) -> bool {
    let sign = diffs[0].signum();

    diffs
        .iter()
        .all(|d| d.signum() == sign && d.abs() >= 1 && d.abs() <= 3)
}

fn is_safe(levels: &[i32]) -> bool {
    let mut diffs = Vec::new();
    for i in 1..levels.len() {
        diffs.push(levels[i] - levels[i - 1]);
    }

    let result = check_diffs(&diffs);
    debug!("{:?} is {}", levels, result);
    result
}

fn part1(input: &str) -> String {
    let count = input
        .split('\n')
        .map(get_all_numbers::<i32>)
        .filter(|v| is_safe(v))
        .count();

    count.to_string()
}

fn is_safe2(levels: &[i32]) -> bool {
    let mut diffs = Vec::new();

    for skip in 0..levels.len() {
        let mut diff = Vec::new();
        let mut last = None;

        for (i, v) in levels.iter().enumerate() {
            if i == skip {
                continue;
            }

            if let Some(x) = last {
                diff.push(v - x)
            }

            last = Some(v);
        }
        diffs.push(diff);
    }

    diffs.into_iter().any(|d| check_diffs(&d))
}

fn part2(input: &str) -> String {
    let count = input
        .split('\n')
        .map(get_all_numbers::<i32>)
        .filter(|v| is_safe2(v))
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
    part1 = "2",
    part2 = "4"
}
