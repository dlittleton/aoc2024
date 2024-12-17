use aoc2024::{input::get_all_numbers, sample};

fn main() {
    aoc2024::run(part1, None);
}

struct Device {
    reg_a: u32,
    reg_b: u32,
    reg_c: u32,
    program: Vec<u32>,
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

    fn to_combo(&self, arg: u32) -> u32 {
        match arg {
            0..=3 => arg,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            x => panic!("Unexpected combo argument: {}", x),
        }
    }

    fn run(&mut self) -> Vec<u32> {
        let mut i = 0;
        let mut out = Vec::new();

        while i < self.program.len() {
            let op = self.program[i];
            let arg = self.program.get(i + 1);

            match (op, arg) {
                // adv
                (0, Some(a)) => {
                    let v = self.to_combo(*a);
                    self.reg_a /= 1 << v;
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
                    if self.reg_a != 0 {
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
                    self.reg_b = self.reg_a / (1 << v);
                }
                //cdv
                (7, Some(a)) => {
                    let v = self.to_combo(*a);
                    self.reg_c = self.reg_a / (1 << v);
                }
                (op, arg) => panic!("Unexpected operation {} {:?}", op, arg),
            }

            i += 2;
        }

        out
    }
}

fn part1(input: &str) -> String {
    let mut device = Device::parse(input);
    let out: Vec<String> = device.run().iter().map(|v| v.to_string()).collect();
    out.join(",")
}

sample! {
    r"
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0",
    part1 = "4,6,3,5,6,3,5,2,1,0"
}
