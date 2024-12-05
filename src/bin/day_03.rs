use aoc2024::sample;
use regex::Regex;
use tracing::{debug, info};

fn main() {
    aoc2024::run(part1, Some(part2));
}

fn part1(input: &str) -> String {
    let re_mul = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let total: i32 = re_mul
        .captures_iter(input)
        .map(|m| {
            let a = m.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let b: i32 = m.get(2).unwrap().as_str().parse::<i32>().unwrap();

            a * b
        })
        .sum();

    total.to_string()
}

fn part2(input: &str) -> String {
    let mut enabled = true;
    let re_mul = Regex::new(r"(?:do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\))").unwrap();
    let total: i32 = re_mul
        .captures_iter(input)
        .map(|m| {
            let full = m.get(0).unwrap().as_str();
            debug!("Found {}", full);
            match (full, enabled) {
                ("do()", _) => {
                    info!("Enabled multiplication.");
                    enabled = true;
                    0
                }
                ("don't()", _) => {
                    info!("Disabled multiplication.");
                    enabled = false;
                    0
                }
                (_, true) => {
                    let a = m.get(1).unwrap().as_str().parse::<i32>().unwrap();
                    let b: i32 = m.get(2).unwrap().as_str().parse::<i32>().unwrap();

                    a * b
                }
                (_, false) => 0,
            }
        })
        .sum();

    total.to_string()
}

sample! {
    r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
    part1 = "161"
}

mod s2 {
    use super::*;

    sample! {
        r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
        part2 = "48"
    }
}
