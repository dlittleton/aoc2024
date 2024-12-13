use aoc2024::{input::get_all_numbers, sample};
use tracing::{debug, info};

fn main() {
    aoc2024::run(part1, None);
}

type Point = (isize, isize);

#[derive(Debug)]
struct ClawMachine {
    a: Point,
    b: Point,
    target: Point,
}

impl ClawMachine {
    fn parse<'a>(mut input: impl Iterator<Item = &'a str>) -> Self {
        let a = Self::read_point(input.next().unwrap());
        let b = Self::read_point(input.next().unwrap());
        let target = Self::read_point(input.next().unwrap());

        Self { a, b, target }
    }

    fn read_point(line: &str) -> Point {
        let nums = get_all_numbers::<isize>(line);
        if nums.len() != 2 {
            panic!("Unexpected number count - {}. Line: {}", nums.len(), line);
        }
        (nums[0], nums[1])
    }

    fn find_cost(&self) -> isize {
        const MAX_PRESSES: isize = 101;

        let (ax, ay) = self.a;
        let (bx, by) = self.b;
        let (tx, ty) = self.target;

        let mut solutions = Vec::new();

        for ap in 0..MAX_PRESSES {
            let (x, y) = (ax * ap, ay * ap);

            if x > tx || y > ty {
                break;
            }

            for bp in 0..MAX_PRESSES {
                let (x, y) = (x + (bx * bp), y + (by * bp));

                if x == tx && y == ty {
                    solutions.push((ap, bp));
                } else if x > tx || y > ty {
                    break;
                }
            }
        }

        info!("Found {} solutions.", solutions.len());

        solutions.iter().map(|(a, b)| a * 3 + b).min().unwrap_or(0)
    }
}

fn part1(input: &str) -> String {
    let mut lines = input.split('\n');
    let mut machines = Vec::new();

    loop {
        machines.push(ClawMachine::parse(lines.by_ref()));

        if lines.next().is_none() {
            break;
        }
    }

    debug!("Machines {:?}", machines);

    let total: isize = machines.iter().map(|m| m.find_cost()).sum();

    total.to_string()
}

sample! {
    r"
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279",
    part1 = "480"
}
