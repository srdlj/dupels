use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Displays the name of files contained within a directory.
    /// If no operand is given, the contents of the current directory are displayed.
    pub file: Option<PathBuf>,

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
    pub depth: Option<usize>,

    /// Specify the seperator to use when listing the filenames.
    #[arg(short, long, default_value = ">--")]
    pub seperator: String,

    /// Omit displaying files that are unique.
    #[clap(short, long, action)]
    pub omit: bool,

    /// Specify the maximum number of threads to use.
    /// The default is the number of logical cores on the machine.
    #[clap(long, default_value = None, verbatim_doc_comment)]
    pub max_threads: Option<usize>,
}
