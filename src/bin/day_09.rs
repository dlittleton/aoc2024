use std::iter;

use aoc2024::sample;

fn main() {
    aoc2024::run(part1, None);
}

struct Disk {
    files: Vec<Option<i32>>,
}

impl Disk {
    fn parse(input: &str) -> Self {
        let mut file_id = 0;
        let mut files = Vec::new();

        for (i, c) in input.chars().enumerate() {
            let block: usize = c.to_string().parse().expect("Failed to parse digit");
            if i % 2 == 0 {
                files.extend(iter::repeat(Some(file_id)).take(block));
                file_id += 1;
            } else {
                files.extend(iter::repeat(None).take(block));
            }
        }

        Disk { files }
    }

    fn compact(&mut self) {
        let mut head = 0;
        let mut tail = self.files.len() - 1;

        loop {
            // Move head to first empty position
            while self.files[head].is_some() {
                head += 1;
            }

            // Move tail to first non empty position
            while self.files[tail].is_none() {
                tail -= 1;
            }

            if head > tail {
                break;
            }

            self.files.swap(head, tail);
        }
    }

    fn checksum(&self) -> u64 {
        self.files
            .iter()
            .enumerate()
            .filter_map(|(i, value)| value.map(|v| i as u64 * v as u64))
            .sum()
    }
}

fn part1(input: &str) -> String {
    let mut disk = Disk::parse(input);
    disk.compact();

    disk.checksum().to_string()
}

sample! {
    r"2333133121414131402",
    part1 = "1928"
}
