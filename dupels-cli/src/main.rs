use dupels_lib::{Cli, run};
use clap::Parser;

fn main() {
    let args = Cli::parse();
    let output = run(&args);
    print!("{}", output);
}
