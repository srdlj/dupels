use clap::Parser;
use dupels as lib;

fn main() {
    let args = lib::Cli::parse();
    lib::run(&args);
}
