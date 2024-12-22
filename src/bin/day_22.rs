use std::collections::{HashMap, HashSet, VecDeque};

use aoc2024::{input::get_all_numbers, sample};
use itertools::Itertools;
use tracing::{debug, info};

fn main() {
    aoc2024::run(part1, Some(part2));
}

type PriceKey = (i32, i32, i32, i32);
type PriceTracker = HashMap<PriceKey, usize>;

struct Secret {
    value: usize,
    generation: usize,
}

impl Secret {
    fn new(value: usize) -> Self {
        Secret {
            value,
            generation: 0,
        }
    }

    fn mix(&mut self, value: usize) {
        self.value ^= value;
    }

    fn prune(&mut self) {
        self.value %= 16777216;
    }

    fn evolve(&mut self) {
        let mut next = self.value * 64;
        self.mix(next);
        self.prune();

        next = self.value / 32;
        self.mix(next);
        self.prune();

        next = self.value * 2048;
        self.mix(next);
        self.prune();

        self.generation += 1;
    }

    fn run(&mut self, target_generation: usize) {
        while self.generation < target_generation {
            self.evolve();
        }
    }

    fn run_and_track_prices(&mut self, target_generation: usize, tracker: &mut PriceTracker) {
        let mut history = VecDeque::new();

        let mut seen = HashSet::new();

        while self.generation < target_generation {
            let original = self.value % 10;
            self.evolve();
            let next = self.value % 10;

            let diff = (next as i32) - (original as i32);
            history.push_back(diff);

            if history.len() > 4 {
                history.pop_front();
            }

            if history.len() == 4 {
                let key: PriceKey = history.iter().copied().collect_tuple().unwrap();

                if seen.contains(&key) {
                    debug!(
                        "Skipping sequence as it has already been seen on this secret. {:?}",
                        key
                    );
                    continue;
                }

                *tracker.entry(key).or_default() += next;
                seen.insert(key);
            }
        }
    }
}

fn part1(input: &str) -> String {
    let secrets: Vec<_> = input
        .lines()
        .map(|l| {
            let num = *get_all_numbers::<usize>(l).first().unwrap();
            let mut secret = Secret::new(num);
            secret.run(2000);
            secret.value
        })
        .collect();

    let total: usize = secrets.iter().sum();

    total.to_string()
}

fn part2(input: &str) -> String {
    let init: Vec<_> = input
        .lines()
        .map(|l| *get_all_numbers::<usize>(l).first().unwrap())
        .collect();

    let mut tracker = PriceTracker::new();
    for v in init {
        let mut secret = Secret::new(v);
        secret.run_and_track_prices(2000, &mut tracker);
    }

    let best = tracker.iter().max_by_key(|(_, v)| **v).unwrap();

    info!(
        "Best sequence produces {} bananas. Sequence is {:?}",
        best.1, best.0
    );

    best.1.to_string()
}

sample! {
    r"
1
10
100
2024",
    part1 = "37327623"
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test_log::test(rstest)]
    fn test_tracking() {
        let mut tracker = PriceTracker::new();

        let mut secret = Secret::new(123);
        secret.run_and_track_prices(10, &mut tracker);

        assert_eq!(4, *tracker.get(&(-3, 6, -1, -1)).unwrap());
        assert_eq!(4, *tracker.get(&(6, -1, -1, 0)).unwrap());
        assert_eq!(6, *tracker.get(&(-1, -1, 0, 2)).unwrap());
    }

    sample! {
        r"
1
2
3
2024",
        part2 = "23"
    }
}
