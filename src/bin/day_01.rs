use std::collections::HashMap;

use aoc2024::input::get_all_numbers;

fn main() {
    aoc2024::run(part1, Some(part2));
}

fn parse(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut first = Vec::new();
    let mut second = Vec::new();

    input.split('\n').for_each(|l| {
        let nums = get_all_numbers::<i32>(l);
        first.push(*nums.first().unwrap());
        second.push(*nums.last().unwrap());
    });

    (first, second)
}

fn part1(input: &str) -> String {
    let (mut first, mut second) = parse(input);

    first.sort();
    second.sort();

    let total: i32 = first
        .into_iter()
        .zip(second)
        .map(|(a, b)| (a - b).abs())
        .sum();

    format!("{}", total)
}

fn part2(input: &str) -> String {
    let (first, second) = parse(input);

    let mut counts: HashMap<i32, i32> = HashMap::new();

    second.into_iter().for_each(|k| {
        *counts.entry(k).or_insert(0) += 1;
    });

    let total: i32 = first
        .into_iter()
        .map(|v| v * counts.get(&v).unwrap_or(&0))
        .sum();

    format!("{}", total)
}

#[cfg(test)]
mod tests {

    use super::*;

    const SAMPLE: &str = r"
3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_part1() {
        let contents = SAMPLE.trim_start();
        let result = part1(contents);
        assert_eq!("11", result);
    }

    #[test]
    fn test_part2() {
        let contents = SAMPLE.trim_start();
        let result = part2(contents);
        assert_eq!("31", result);
    }
}
