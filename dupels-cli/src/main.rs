use dupels_lib::{Cli, run_cli};
use clap::Parser;

fn main() {
    let args = Cli::parse();
    let output = run_cli(&args);
    if !output.is_empty() {
        println!("{}", output);
    }
}
