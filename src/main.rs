mod libs;
use clap::Parser;

fn main() {
  let args = libs::Cli::parse();
  libs::run(&args);
}
