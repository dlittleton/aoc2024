use aoc2024::sample;
use cached::proc_macro::cached;
use tracing::debug;

fn main() {
    aoc2024::run(part1, Some(part2));
}

struct Towels {
    available: Vec<String>,
    targets: Vec<String>,
}

#[cached(key = "String", convert = r#"{ String::from(target) }"#)]
fn try_match_pattern(available: &Vec<String>, target: &str) -> usize {
    if target.is_empty() {
        return 1;
    }

    let mut total = 0;
    for a in available {
        if target.starts_with(a) {
            total += try_match_pattern(available, &target[a.len()..]);
        }
    }

    total
}

impl Towels {
    fn parse(input: &str) -> Self {
        let mut lines = input.lines();
        let available = lines
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        // Skip blank line
        lines.next();

        let targets = lines.map(|l| l.to_string()).collect();

        debug!("Available towels {:?}", available);
        debug!("Target patterns {:?}", targets);

        Self { available, targets }
    }

    fn count_possible(&self) -> usize {
        self.targets
            .iter()
            .filter(|t| try_match_pattern(&self.available, t) > 0)
            .count()
    }

    fn count_permutations(&self) -> usize {
        self.targets
            .iter()
            .map(|t| try_match_pattern(&self.available, t))
            .sum()
    }
}

fn part1(input: &str) -> String {
    let towels = Towels::parse(input);
    let count = towels.count_possible();
    count.to_string()
}

fn part2(input: &str) -> String {
    let towels = Towels::parse(input);
    let count = towels.count_permutations();
    count.to_string()
}

sample! {
    r"
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb",
    part1 = "6",
    part2 = "16"
}
