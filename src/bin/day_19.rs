use aoc2024::sample;
use cached::proc_macro::cached;
use tracing::debug;

fn main() {
    aoc2024::run(part1, None);
}

struct Towels {
    available: Vec<String>,
    targets: Vec<String>,
}

#[cached(key = "String", convert = r#"{ String::from(target) }"#)]
fn try_match_pattern(available: &Vec<String>, target: &str) -> bool {
    if target.is_empty() {
        return true;
    }

    for a in available {
        if target.starts_with(a) && try_match_pattern(available, &target[a.len()..]) {
            return true;
        }
    }

    false
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

    fn count_possible(&mut self) -> usize {
        self.targets
            .iter()
            .filter(|t| try_match_pattern(&self.available, t))
            .count()
    }
}

fn part1(input: &str) -> String {
    let mut towels = Towels::parse(input);
    let count = towels.count_possible();
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
    part1 = "6"
}
