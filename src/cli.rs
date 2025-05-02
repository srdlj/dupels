use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
  /// Include directory entries whose names begin with a dot (.)
  #[clap(short, action)]
  pub all: bool,

  /// Generate the file names in a direcotry tree by walking the tree top-down.
  /// If the -d option is specified, walk to the depth specified, otherwise the default is depth of 2.
  #[clap(short, action, verbatim_doc_comment)]
  pub recursive: bool,

  /// Specifies the depth to generate file names during walk.
  /// The -d option implies the -r option.
  #[arg(short, long, verbatim_doc_comment)]
  pub depth: Option<u8>,

  /// Specify the seperator to use when listing the filenames.
  /// The default seperator is ">--"
  #[arg(short, long, default_value = ">--", verbatim_doc_comment)]
  pub seperator: String,

  /// Omit displaying files that are unique.
  #[clap(short, long, action)]
  pub omit: bool,

  /// Displays the name of files contained within a directory.
  /// If no operands are given, the contents of the current directory are displayed.
  pub file: Option<PathBuf>
}
