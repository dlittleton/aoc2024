use std::{char, collections::HashMap};

use aoc2024::{
    collections::grid::{Grid, Position, CARDINAL_DIRECTIONS},
    sample,
};
use tracing::info;

fn main() {
    aoc2024::run(part1, None);
}

struct Plot {
    plant: char,
    id: usize,
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
}

fn part1(input: &str) -> String {
    let g = Garden::parse(input);
    let price = g.score();
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
    part1 = "1930"
}
