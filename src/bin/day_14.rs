use std::iter;

use aoc2024::{collections::grid::Grid, input::get_all_numbers};
use tracing::{debug, info};

fn main() {
    aoc2024::run(part1, Some(part2));
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

    fn advance(&mut self, width: i32, height: i32) {
        self.px = (self.px + self.vx).rem_euclid(width);
        self.py = (self.py + self.vy).rem_euclid(height);
    }
}

struct RoboGrid {
    robots: Vec<Robot>,
    grid: Grid<i32>,
}

impl RoboGrid {
    fn new(robots: Vec<Robot>, width: i32, height: i32) -> Self {
        let mut grid: Grid<i32> = iter::repeat(iter::repeat(0).take(width as usize))
            .take(height as usize)
            .collect();

        info!(
            "Created grid with {} rows and {} cols",
            grid.rows(),
            grid.cols()
        );

        for bot in robots.iter() {
            *grid.get_mut(bot.py as usize, bot.px as usize) += 1;
        }

        Self { robots, grid }
    }

    fn advance(&mut self) {
        for bot in self.robots.iter_mut() {
            *self.grid.get_mut(bot.py as usize, bot.px as usize) -= 1;
            bot.advance(self.grid.cols() as i32, self.grid.rows() as i32);
            *self.grid.get_mut(bot.py as usize, bot.px as usize) += 1;
        }
    }

    fn dump_grid(&self) {
        for row in self.grid.row_wise_iter() {
            let line: String = row.map(|v| if *v > 0 { "+" } else { "." }).collect();
            println!("{}", line)
        }
    }

    fn detect_tree(&self) -> usize {
        // Note: The assumed shape turned out to be wildly incorrect, but in the
        // process of trying to find heuristics the below check for contiguous
        // rows seems to be good enough.

        // Assumed tree shape
        // ....+....
        // ...+.+...
        // ..+...+..
        // .+++++++.
        // ....+....

        // Heuristic: Search for the bottom line.
        let mut longest_row = 0;

        for row in self.grid.row_wise_iter() {
            let mut contiguous = 0;
            for col in row {
                if *col > 0 {
                    contiguous += 1;
                } else {
                    longest_row = longest_row.max(contiguous);
                    contiguous = 0;
                }
            }
            longest_row = longest_row.max(contiguous);
        }

        longest_row as usize
    }

    fn search_tree(&mut self) -> usize {
        let mut t = 0;

        self.dump_grid();
        loop {
            t += 1;
            self.advance();
            //self.dump_grid();
            let bots_in_tree = self.detect_tree();
            debug!("Time {} - Bots in tree {}", t, bots_in_tree);

            if bots_in_tree > 10 {
                info!("There are {} bots in the tree. Stopping.", bots_in_tree);
                self.dump_grid();
                break;
            }
        }

        t
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

fn solve2(input: &str, width: i32, height: i32) -> String {
    let bots: Vec<_> = input.lines().map(Robot::parse).collect();
    let mut robogrid = RoboGrid::new(bots, width, height);

    let t = robogrid.search_tree();

    t.to_string()
}

fn part1(input: &str) -> String {
    solve(input, 101, 103)
}

fn part2(input: &str) -> String {
    solve2(input, 101, 103)
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
