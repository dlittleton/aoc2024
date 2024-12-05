pub mod collections;
pub mod input;
pub mod sample;

use std::{fs, path::PathBuf};

use clap::Parser;
use tracing::info;

pub type Solution = fn(&str) -> String;

#[derive(Debug, Parser)]
struct Args {
    #[command(flatten)]
    verbosity: clap_verbosity_flag::Verbosity,

    /// Override day from input path
    #[arg(short, long)]
    part_two: bool,

    /// Input path
    input_file: PathBuf,
}

pub fn run(part1: Solution, part2: Option<Solution>) {
    let bin = std::env::args()
        .next()
        .expect("Failed to read program name");

    let args = Args::parse();

    tracing_subscriber::fmt()
        .with_max_level(args.verbosity)
        .init();

    info!("Running {}", bin);
    info!("Input file: {}", args.input_file.to_str().unwrap());
    info!("Solving part {}", if args.part_two { "two" } else { "one" });

    let contents = fs::read_to_string(&args.input_file).expect("Failed to read input file.");

    let result = if args.part_two {
        part2.expect("Part two not implemented.")(&contents)
    } else {
        part1(&contents)
    };

    println!("{}", result);
}
