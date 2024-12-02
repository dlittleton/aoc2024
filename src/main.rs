use clap::Parser;
use tracing::{debug, info};

#[derive(Debug, Parser)]
struct Args {
    #[command(flatten)]
    verbosity: clap_verbosity_flag::Verbosity,
}

fn main() {
    let args = Args::parse();

    tracing_subscriber::fmt()
        .with_max_level(args.verbosity)
        .init();

    println!("Hello, world!");
    debug!("Hello from debug!");
    info!("Hello from info!");
}
