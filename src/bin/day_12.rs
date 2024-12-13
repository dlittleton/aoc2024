use std::collections::BTreeSet;
use std::{char, collections::HashMap};

use aoc2024::collections::grid::{get_direction_delta, Direction};
use aoc2024::{
    collections::grid::{Grid, Position, CARDINAL_DIRECTIONS},
    sample,
};
use tracing::info;

fn main() {
    aoc2024::run(part1, Some(part2));
}

struct Plot {
    plant: char,
    id: usize,
}

type Point = (isize, isize);

#[derive(Default)]
struct Perimeters {
    nodes: BTreeSet<Point>,
    perimeters: [BTreeSet<Point>; 4],
}

impl Perimeters {
    fn add(&mut self, pos: &Position<Plot>, dir: Direction) {
        let p = (pos.row() as isize, pos.col() as isize);
        self.nodes.insert(p);
        self.perimeters[dir as usize].insert(p);
    }

    fn get_merge_directions(dir: Direction) -> Vec<Direction> {
        match dir {
            Direction::North | Direction::South => vec![Direction::East, Direction::West],
            Direction::East | Direction::West => vec![Direction::North, Direction::South],
        }
    }

    fn combine_sides(&mut self) -> usize {
        let mut sides = 0;

        for dir in CARDINAL_DIRECTIONS.iter() {
            let perims = &mut self.perimeters[*dir as usize];

            while let Some(point) = perims.pop_first() {
                // New side.
                sides += 1;

                // Remove touching nodes in the same perimeter set
                for search in Self::get_merge_directions(*dir) {
                    let mut new = point.clone();
                    let add = get_direction_delta(search);

                    loop {
                        new = (new.0 + add.0, new.1 + add.1);

                        if !perims.remove(&new) {
                            break;
                        }
                    }
                }
            }
        }

        sides
    }
}

struct Garden {
    plots: Grid<Plot>,
}

impl Garden {
    fn parse(input: &str) -> Self {
        let plants: Grid<char> = input.split('\n').map(|line| line.chars()).collect();
        let mut plots = plants.map(|c| Plot { plant: *c, id: 0 });

        let mut next_id = 1;

        for pos in plants.positions() {
            Self::fill_plot(&pos, next_id, &mut plots);
            next_id += 1;
        }

        Garden { plots }
    }

    fn fill_plot(pos: &Position<char>, id: usize, plots: &mut Grid<Plot>) {
        if plots.get(pos.row(), pos.col()).id != 0 {
            return;
        }

        plots.get_mut(pos.row(), pos.col()).id = id;
        pos.get_neighbors(&CARDINAL_DIRECTIONS)
            .filter(|n| n.value() == pos.value())
            .for_each(|n| Self::fill_plot(&n, id, plots));
    }

    fn score(&self) -> usize {
        let mut perimeters: HashMap<usize, Vec<usize>> = HashMap::new();

        for pos in self.plots.positions() {
            let mut perimeter = 0;
            for dir in CARDINAL_DIRECTIONS.iter() {
                if let Some(neighbor) = pos.get_neighbor(*dir) {
                    if pos.value().plant != neighbor.value().plant {
                        perimeter += 1;
                    }
                } else {
                    perimeter += 1;
                }
            }

            perimeters
                .entry(pos.value().id)
                .or_default()
                .push(perimeter);
        }

        let mut total = 0;
        for (k, v) in perimeters {
            let size = v.len();
            let perimeter: usize = v.iter().sum();
            let cost = size * perimeter;

            info! {"Region {} => {} * {} = {}", k, size, perimeter, cost}
            total += cost;
        }

        total
    }

    fn score_sides(&self) -> usize {
        let mut size: HashMap<usize, usize> = HashMap::new();
        let mut perimeters: HashMap<usize, Perimeters> = HashMap::new();

        for pos in self.plots.positions() {
            let perim = perimeters.entry(pos.value().id).or_default();
            for dir in CARDINAL_DIRECTIONS.iter() {
                if let Some(neighbor) = pos.get_neighbor(*dir) {
                    if pos.value().plant != neighbor.value().plant {
                        perim.add(&pos, *dir);
                    }
                } else {
                    perim.add(&pos, *dir);
                }
            }

            *size.entry(pos.value().id).or_default() += 1;
        }

        let mut total = 0;
        for (k, v) in size {
            let perim = perimeters.get_mut(&k).unwrap();
            let sides = perim.combine_sides();

            let cost = v * sides;

            info! {"Region {} => Size {} * Sides {} = {}", k, v, sides, cost}
            total += cost;
        }

        total
    }
}

fn part1(input: &str) -> String {
    let g = Garden::parse(input);
    let price = g.score();
    price.to_string()
}

fn part2(input: &str) -> String {
    let g = Garden::parse(input);
    let price = g.score_sides();
    price.to_string()
}

sample! {
    r"
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE",
    part1 = "1930",
    part2 = "1206"
}

mod sample2 {
    use super::*;

    sample! {
        r"
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA",
        part2 = "368"
    }
}

mod sample3 {
    use super::*;

    sample! {
        r"
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE",
        part2 = "236"
    }
}
