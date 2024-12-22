use aoc2024::{input::get_all_numbers, sample};

fn main() {
    aoc2024::run(part1, None);
}

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

sample! {
    r"
1
10
100
2024",
    part1 = "37327623"
}
