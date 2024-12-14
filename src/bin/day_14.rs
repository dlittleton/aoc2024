use aoc2024::input::get_all_numbers;
use tracing::info;

fn main() {
    aoc2024::run(part1, None);
}

struct Robot {
    px: i32,
    py: i32,
    vx: i32,
    vy: i32,
}

impl Robot {
    fn parse(input: &str) -> Self {
        let nums = get_all_numbers(input);
        if nums.len() != 4 {
            panic!("Failed to parse robot: {}", input);
        }

        let px = nums[0];
        let py = nums[1];
        let vx = nums[2];
        let vy = nums[3];

        Self { px, py, vx, vy }
    }

    fn get_position(&self, t: i32, width: i32, height: i32) -> (i32, i32) {
        let px = (self.px + (self.vx * t)).rem_euclid(width);
        let py = (self.py + (self.vy * t)).rem_euclid(height);

        (px, py)
    }
}

fn solve(input: &str, width: i32, height: i32) -> String {
    let bots: Vec<_> = input.lines().map(Robot::parse).collect();

    let mid_x = width / 2;
    let mid_y = height / 2;

    let mut quadrants = [0, 0, 0, 0];
    for bot in bots {
        let (x, y) = bot.get_position(100, width, height);
        info!("Bot located at {}, {}", x, y);

        if x < mid_x && y < mid_y {
            quadrants[0] += 1;
        } else if x > mid_x && y < mid_y {
            quadrants[1] += 1;
        } else if x < mid_x && y > mid_y {
            quadrants[2] += 1;
        } else if x > mid_x && y > mid_y {
            quadrants[3] += 1;
        }
    }

    let total = quadrants.iter().copied().reduce(|acc, v| acc * v).unwrap();

    total.to_string()
}

fn part1(input: &str) -> String {
    solve(input, 101, 103)
}

#[cfg(test)]
mod wrapped_sample_test {
    use aoc2024::sample;

    use super::*;

    fn part1_test_size(input: &str) -> String {
        solve(input, 11, 7)
    }

    sample! {
        r"
    p=0,4 v=3,-3
    p=6,3 v=-1,-3
    p=10,3 v=-1,2
    p=2,0 v=2,-1
    p=0,0 v=1,3
    p=3,0 v=-2,-2
    p=7,6 v=-1,-3
    p=3,0 v=-1,-2
    p=9,3 v=2,3
    p=7,3 v=-1,2
    p=2,4 v=2,-3
    p=9,5 v=-3,-3",
        part1_test_size = "12"
    }
}
