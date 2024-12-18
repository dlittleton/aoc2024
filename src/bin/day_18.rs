use std::collections::{HashSet, VecDeque};

use aoc2024::{
    collections::grid::{Grid, CARDINAL_DIRECTIONS},
    input::get_all_numbers,
};
use tracing::{debug, info};

fn main() {
    aoc2024::run(part1, Some(part2));
}

fn search(grid: &Grid<char>) -> usize {
    let mut to_visit: VecDeque<_> = vec![(grid.position(0, 0).unwrap(), 0)].into();

    let mut seen = HashSet::new();

    while let Some((current, depth)) = to_visit.pop_front() {
        info!(
            "Visiting ({}, {}) at depth {}",
            current.row(),
            current.col(),
            depth
        );

        if current.row() == grid.rows() - 1 && current.col() == grid.cols() - 1 {
            return depth;
        }

        for neighbor in current.get_neighbors(&CARDINAL_DIRECTIONS) {
            debug!(
                "Neighbor ({}, {}) has value {}",
                neighbor.row(),
                neighbor.col(),
                neighbor.value()
            );

            if !seen.contains(&(neighbor.row(), neighbor.col())) && *neighbor.value() != '#' {
                to_visit.push_back((neighbor, depth + 1));
                seen.insert((neighbor.row(), neighbor.col()));
            }
        }
    }

    0
}

fn solve(input: &str, width: usize, height: usize, depth: usize) -> String {
    let mut g = Grid::new(height, width, '.');

    for line in input.lines().take(depth) {
        let nums = get_all_numbers::<usize>(line);
        *g.get_mut(nums[1], nums[0]) = '#';
    }

    search(&g).to_string()
}

fn solve2(input: &str, width: usize, height: usize, depth: usize) -> String {
    let mut g = Grid::new(height, width, '.');

    let mut lines = input.lines();

    // Initial conditions
    for line in lines.by_ref().take(depth) {
        let nums = get_all_numbers::<usize>(line);
        *g.get_mut(nums[1], nums[0]) = '#';
    }

    loop {
        let next = lines.next().unwrap();
        let nums = get_all_numbers::<usize>(next);
        *g.get_mut(nums[1], nums[0]) = '#';

        if search(&g) == 0 {
            return next.to_string();
        }
    }
}

fn part1(input: &str) -> String {
    solve(input, 71, 71, 1024)
}

fn part2(input: &str) -> String {
    solve2(input, 71, 71, 1024)
}

#[cfg(test)]
mod test {
    use aoc2024::sample;

    use super::*;

    fn test_part1(input: &str) -> String {
        solve(input, 7, 7, 12)
    }

    fn test_part2(input: &str) -> String {
        solve2(input, 7, 7, 12)
    }

    sample! {
        r"
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0",
        test_part1 = "22",
        test_part2 = "6,1"
    }
}
