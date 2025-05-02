use dupels as lib;
use clap::Parser;

fn main() {
  let args = lib::Cli::parse();
  lib::run(&args);
}
