use std::collections::{HashMap, HashSet};

use aoc2024::{input::get_all_numbers, sample};
use tracing::{debug, info};

fn main() {
    aoc2024::run(part1, None);
}

type RuleMap = HashMap<i32, HashSet<i32>>;

fn parse_rules<'a>(input: impl Iterator<Item = &'a str>) -> RuleMap {
    let mut rules = RuleMap::new();

    input.take_while(|s| !s.is_empty()).for_each(|line| {
        let numbers = get_all_numbers::<i32>(line);
        let a = *numbers.first().unwrap();
        let b = *numbers.last().unwrap();

        rules.entry(b).or_default().insert(a);
    });

    rules
}

fn is_valid(rules: &RuleMap, pages: &[i32]) -> bool {
    let mut forbidden: HashSet<i32> = HashSet::new();

    for p in pages.iter() {
        if forbidden.contains(p) {
            return false;
        }

        forbidden.extend(rules.get(p).unwrap_or(&HashSet::new()));
    }

    true
}

fn part1(input: &str) -> String {
    let mut lines = input.split('\n');

    let rules = parse_rules(lines.by_ref());
    debug!("{:?}", rules);

    let total: i32 = lines
        .by_ref()
        .map(get_all_numbers::<i32>)
        .filter(|pages| is_valid(&rules, pages))
        .map(|pages| {
            info!("{:?} is valid", pages);
            *pages.get(pages.len() / 2).unwrap()
        })
        .sum();

    total.to_string()
}

sample! {
    r"
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47",
    part1 = "143"
}
