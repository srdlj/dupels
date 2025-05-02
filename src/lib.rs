mod cli;
mod dupels;

pub use cli::Cli;
pub use dupels::DupeLs;


pub fn run(args: &Cli) {
  let mut dupels = DupeLs::new(&args);
  dupels.parse(&dupels.get_path(), dupels.get_depth());
  dupels.print();
}
