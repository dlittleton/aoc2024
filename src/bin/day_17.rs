use aoc2024::{input::get_all_numbers, sample};
use tracing::{debug, info};

fn main() {
    aoc2024::run(part1, Some(part2));
}

#[derive(Clone)]
struct Device {
    reg_a: u64,
    reg_b: u64,
    reg_c: u64,
    program: Vec<u64>,
}

impl Device {
    fn parse(input: &str) -> Self {
        let mut lines = input.lines();

        let reg_a = get_all_numbers(lines.next().unwrap())[0];
        let reg_b = get_all_numbers(lines.next().unwrap())[0];
        let reg_c = get_all_numbers(lines.next().unwrap())[0];
        lines.next().expect("Failed to skip blank line");
        let program = get_all_numbers(lines.next().unwrap());

        Self {
            reg_a,
            reg_b,
            reg_c,
            program,
        }
    }

    fn to_combo(&self, arg: u64) -> u64 {
        match arg {
            0..=3 => arg,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            x => panic!("Unexpected combo argument: {}", x),
        }
    }

    fn run(&mut self, disable_jumps: bool) -> Vec<u64> {
        let mut i = 0;
        let mut out = Vec::new();

        while i < self.program.len() {
            let op = self.program[i];
            let arg = self.program.get(i + 1);

            match (op, arg) {
                // adv
                (0, Some(a)) => {
                    let v = self.to_combo(*a);
                    self.reg_a >>= v;
                }
                //bxl
                (1, Some(a)) => {
                    self.reg_b ^= *a;
                }
                //bst
                (2, Some(a)) => {
                    self.reg_b = self.to_combo(*a) % 8;
                }
                //jnz
                (3, Some(a)) => {
                    if disable_jumps {
                        break;
                    } else if self.reg_a != 0 {
                        i = *a as usize;
                        continue;
                    }
                }
                //bxc
                (4, Some(_)) => {
                    self.reg_b ^= self.reg_c;
                }
                //out
                (5, Some(a)) => {
                    out.push(self.to_combo(*a) % 8);
                }
                //bdv
                (6, Some(a)) => {
                    let v = self.to_combo(*a);
                    self.reg_b = self.reg_a >> v;
                }
                //cdv
                (7, Some(a)) => {
                    let v = self.to_combo(*a);
                    self.reg_c = self.reg_a >> v;
                }
                (op, arg) => panic!("Unexpected operation {} {:?}", op, arg),
            }

            i += 2;
        }

        out
    }

    // Search output conditions 3 bits at a time.
    fn search(&mut self, target_a: u64, target_output_idx: i32) -> Option<u64> {
        // Hit all targets
        if target_output_idx < 0 {
            return Some(target_a);
        }

        let start = (target_a << 3).max(1);

        let target_output = *self.program.get(target_output_idx as usize).unwrap();
        info!(
            "Searching for target output: {}. Start position {}",
            target_output, start
        );

        for i in start..start + 8 {
            // Assumption: B and C are reset and don't need to preserve value across jumps
            self.reg_a = i;
            self.reg_b = 0;
            self.reg_c = 0;

            let result = self.run(true);
            let r = *result.first().unwrap();

            if r == target_output && self.reg_a == target_a {
                debug!("Found possible answer at {}", i);
                if let Some(child) = self.search(i, target_output_idx - 1) {
                    info!("Returning answer: {}", child);
                    return Some(child);
                }
            }
        }

        None
    }

    fn find_initial_conditions(&mut self) -> u64 {
        self.search(0, self.program.len() as i32 - 1).unwrap()
    }
}

fn part1(input: &str) -> String {
    let mut device = Device::parse(input);
    let out: Vec<String> = device.run(false).iter().map(|v| v.to_string()).collect();
    out.join(",")
}

fn part2(input: &str) -> String {
    let mut device = Device::parse(input);
    let out = device.find_initial_conditions();

    device.reg_a = out;
    device.reg_b = 0;
    device.reg_c = 0;

    let check = device.run(false);
    assert_eq!(check, device.program);

    out.to_string()
}

sample! {
    r"
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0",
    part1 = "4,6,3,5,6,3,5,2,1,0"
}

mod sample_2 {
    use super::*;
    sample! {
    r"
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0",
        part2 = "117440"
    }
}
