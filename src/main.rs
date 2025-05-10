use clap::Parser;
use dupels as lib;

fn main() {
    let args = lib::Cli::parse();
    let output = lib::run(&args);
    println!("{}", output);
}
