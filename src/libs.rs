use std::{collections::HashMap, env::current_dir, fs, path::PathBuf};
use clap::Parser;
use md5;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
  /// Include directory entries whose names begin with a dot (.)
  #[clap(short, action)]
  all: bool,

  /// Generate the file names in a direcotry tree by walking the tree top-down.
  /// If the -d option is specified, walk to the depth specified, otherwise the default is depth of 2.
  #[clap(short, action, verbatim_doc_comment)]
  recursive: bool,

  /// Specifies the depth to generate file names during walk.
  /// The -d option implies the -r option.
  #[arg(short, long, verbatim_doc_comment)]
  depth: Option<u8>,

  /// Specify the seperator to use when listing the filenames.
  /// The default seperator is ">--"
  #[arg(short, long, default_value = "---", verbatim_doc_comment)]
  seperator: String,

  /// Omit displaying files that are unique.
  #[clap(short, long, action)]
  omit: bool,

  /// Displays the name of files contained within a directory.
  /// If no operands are given, the contents of the current directory are displayed.
  file: Option<PathBuf>
}

pub struct DupeLs {
  path: Option<PathBuf>,
  all: bool,
  recursive: bool,
  depth: u8,
  seperator: String,
  omit: bool,
  entries: HashMap<md5::Digest, Vec<String>>
}

impl DupeLs {
  pub fn new(cli: &Cli) -> DupeLs {
    match cli.depth {
      Some(depth) => DupeLs {
          path: cli.file.to_owned(),
          all: cli.all,
          recursive: true,
          depth: depth,
          seperator: cli.seperator.clone(),
          omit: cli.omit,
          entries: HashMap::new()
        },
        None => DupeLs {
          path: cli.file.to_owned(),
          all: cli.all,
          recursive: cli.recursive,
          depth: 2,
          seperator: cli.seperator.clone(),
          omit: cli.omit,
          entries: HashMap::new()
        }
    }
  }

  pub fn get_path(&self) -> PathBuf {
    match &self.path {
      Some(path) => path.to_path_buf(),
      None => current_dir().expect("Could not get directory")
    }
  }

  pub fn print(&self) {
    for (_, v) in &self.entries {
      if v.len() == 1 && self.omit { continue; }
      for p in v {
        println!("{}", p);
      }
      println!("{}", self.seperator);
    }
  }

  pub fn parse(&mut self, dir_path: &PathBuf, depth: u8) {
    if depth == 0 { return }
    if let Ok(file_paths) = fs::read_dir(dir_path) {
      for entry in file_paths {
        if let Ok(entry) = entry {
          if let Ok(metadata) = entry.metadata() {
            if !metadata.is_dir() {
              self.insert(&entry.path());
            } else if self.recursive {
              self.parse(&entry.path(), depth - 1)
            }
          }
        }
      }
    }
  }

  fn insert(&mut self, path: &PathBuf) {
    if let Some(path) = path.to_str() {
      if path.starts_with(".") && !self.all { return;}
      let checksum = DupeLs::get_checksum(path);
      match self.entries.contains_key(&checksum) {
        true => self.entries.get_mut(&checksum).unwrap().push(path.to_string()),
        false => {
          self.entries.insert(checksum, vec![path.to_string()]);
        }
      }
    }
  }

  fn is_dot_file(self, filename: &str) -> bool {
    todo!();
  }

  fn get_checksum(path: &str) -> md5::Digest {
    let content = fs::read(path).expect("Could not read file.");
    let checksum = md5::compute(content);
    checksum
  }

}

pub fn run(args: &Cli) {
  let mut dupels = DupeLs::new(&args);
  dupels.parse(&dupels.get_path(), dupels.depth);
  dupels.print();
}


#[cfg(test)]
mod test {

use super::*;

  #[test]
  fn init_test_no_r_flag() {
    let cli = Cli {
      all: true,
      recursive: false,
      depth: None,
      omit: false,
      seperator: "---".to_string(),
      file: Some(PathBuf::from("files"))
    };
    let f = DupeLs::new(&cli);
    assert!(f.all);
    assert!(!f.recursive);
    assert_eq!(f.depth, 2);
    assert!(!f.omit);
    assert_eq!("---".to_string(), f.seperator);
  }

  #[test]
  fn init_test_with_no_file() {
    let cli = Cli {
      all: true,
      recursive: false,
      depth: None,
      omit: false,
      seperator: "---".to_string(),
      file: None
    };
    let f = DupeLs::new(&cli);
    assert!(f.all);
    assert!(!f.recursive);
    assert_eq!(f.depth, 2);
    assert!(!f.omit);
    assert_eq!("---".to_string(), f.seperator);
  }

  #[test]
  fn init_test_with_r_flag() {
    let cli = Cli {
      all: true,
      recursive: true,
      depth: None,
      omit: false,
      seperator: "---".to_string(),
      file: Some(PathBuf::from("files"))
    };
    let f = DupeLs::new(&cli);
    assert!(f.all);
    assert!(f.recursive);
    assert_eq!(f.depth, 2);
    assert!(!f.omit);
    assert_eq!("---".to_string(), f.seperator);
  }

  #[test]
  fn init_test_with_rd_flags() {
    let cli = Cli {
      all: true,
      recursive: true,
      depth: Some(2),
      omit: false,
      seperator: "hi".to_string(),
      file: Some(PathBuf::from("files"))
    };
    let f = DupeLs::new(&cli);
    assert!(f.all);
    assert!(f.recursive);
    assert_eq!(f.depth, 2);
    assert!(!f.omit);
    assert_eq!("hi".to_string(), f.seperator);
  }

  #[test]
  fn test_checksum() {
    let expected_dupe_md5: &str = "09f7e02f1290be211da707a266f153b3";
    assert_eq!(format!("{:x}", DupeLs::get_checksum("files/1.txt")), expected_dupe_md5);
  }

  #[test]
  fn test_parse_no_r() {
    let cli = Cli {
      all: true,
      recursive: false,
      depth: None,
      omit: false,
      seperator: "---".to_string(),
      file: Some(PathBuf::from("files"))
    };
    let mut d = DupeLs::new(&cli);
    d.parse(&d.get_path(), d.depth);
    assert_eq!(d.entries.len(), 3);
  }

  #[test]
  fn test_parse_r_d_2() {
    let cli = Cli {
      all: true,
      recursive: true,
      depth: Some(2),
      omit: false,
      seperator: "---".to_string(),
      file: Some(PathBuf::from("files"))
    };
    let mut d = DupeLs::new(&cli);
    d.parse(&d.get_path(), d.depth);
    assert_eq!(d.entries.len(), 5);
  }
}
