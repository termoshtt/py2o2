use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Cli {
    path: PathBuf,
}

fn main() {
    let cli = Cli::parse();
    dbg!(cli);
}
