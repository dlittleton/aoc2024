use std::collections::BTreeSet;

use aoc2024::{input::get_all_numbers, sample};

fn main() {
    aoc2024::run(part1, Some(part2));
}

struct Equation {
    target: i64,
    values: Vec<i64>,
}

fn parse(input: &str) -> Vec<Equation> {
    input
        .split('\n')
        .map(|line| {
            let all_nums = get_all_numbers::<i64>(line);
            match &all_nums[..] {
                [t, v @ ..] => Equation {
                    target: *t,
                    values: v.to_vec(),
                },
                _ => panic!("Failed to parse equation"),
            }
        })
        .collect()
}

fn can_match(eq: &Equation, allow_concat: bool) -> bool {
    let mut possibilities = BTreeSet::new();
    possibilities.insert(eq.values[0]);

    for v in eq.values[1..].iter() {
        possibilities = possibilities
            .iter()
            .flat_map(|p| {
                let mut r = vec![p + v, p * v];
                if allow_concat {
                    r.push(format!("{}{}", p, v).parse().expect("Concatenation failed"));
                }
                r
            })
            .collect();
    }

    possibilities.contains(&eq.target)
}

fn part1(input: &str) -> String {
    let eqs = parse(input);

    let total: i64 = eqs
        .iter()
        .filter_map(|eq| {
            if can_match(eq, false) {
                Some(eq.target)
            } else {
                None
            }
        })
        .sum();

    total.to_string()
}

fn part2(input: &str) -> String {
    let eqs = parse(input);

    let total: i64 = eqs
        .iter()
        .filter_map(|eq| {
            if can_match(eq, true) {
                Some(eq.target)
            } else {
                None
            }
        })
        .sum();

    total.to_string()
}

sample! {
    r"
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20",
    part1 = "3749",
    part2 = "11387"
}
