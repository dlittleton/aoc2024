use std::iter;

use aoc2024::sample;
use tracing::info;

fn main() {
    aoc2024::run(part1, Some(part2));
}

#[derive(Clone, Copy)]
struct FreeSpace {
    start: usize,
    len: usize,
}

struct FileSpace {
    start: usize,
    len: usize,
    id: i32,
}

struct Disk {
    files: Vec<Option<i32>>,
    empty: Vec<FreeSpace>,
    used: Vec<FileSpace>,
}

impl Disk {
    fn parse(input: &str) -> Self {
        let mut file_id = 0;
        let mut files = Vec::new();
        let mut empty = Vec::new();
        let mut used = Vec::new();

        for (i, c) in input.chars().enumerate() {
            let block: usize = c.to_string().parse().expect("Failed to parse digit");
            let start_idx = files.len();
            if i % 2 == 0 {
                used.push(FileSpace {
                    start: start_idx,
                    len: block,
                    id: file_id,
                });
                files.extend(iter::repeat(Some(file_id)).take(block));
                file_id += 1;
            } else {
                if block > 0 {
                    empty.push(FreeSpace {
                        start: start_idx,
                        len: block,
                    });
                }
                files.extend(iter::repeat(None).take(block));
            }
        }

        Disk { files, empty, used }
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

    fn contiguous_compact(&mut self) {
        let mut moves = 0;
        let mut skips = 0;

        while let Some(block) = self.used.pop() {
            if let Some((i, free)) = self
                .empty
                .iter()
                .enumerate()
                .find(|(_, f)| f.len >= block.len && f.start < block.start)
                .map(|(i, f)| (i, *f))
            {
                info!("Found earlier position for block {}", block.id);
                moves += 1;

                for pos in 0..block.len {
                    self.files.swap(free.start + pos, block.start + pos);
                }

                if block.len == free.len {
                    self.empty.remove(i);
                } else {
                    self.empty[i].len -= block.len;
                    self.empty[i].start += block.len;
                }
            } else {
                info!("Could not move block {}", block.id);
                skips += 1;
            }
        }

        info!("Moved {} blocks.", moves);
        info!("Skipped {} blocks.", skips);
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

fn part2(input: &str) -> String {
    let mut disk = Disk::parse(input);
    disk.contiguous_compact();

    disk.checksum().to_string()
}

sample! {
    r"2333133121414131402",
    part1 = "1928",
    part2 = "2858"
}
