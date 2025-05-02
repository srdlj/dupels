use std::{collections::HashMap, env::current_dir, fs, io::Read, path::{Path, PathBuf}};
use md5;

use crate::cli::Cli;

pub struct DupeLs {
    base_path: Option<PathBuf>,
    track_dot_files: bool,
    recursive: bool,
    depth: u8,
    seperator: String,
    omit: bool,
    entries: HashMap<md5::Digest, Vec<String>>
  }
  
  impl DupeLs {
    pub fn new(cli: &Cli) -> DupeLs {
        let (recursive, depth) = match cli.depth {
            Some(depth) => (true, depth),
            None => (cli.recursive, 2),
        };

        DupeLs {
            base_path: cli.file.to_owned(),
            track_dot_files: cli.all,
            recursive,
            depth,
            seperator: cli.seperator.clone(),
            omit: cli.omit,
            entries: HashMap::new(),
        }
    }

    pub fn get_depth(&self) -> u8 {
      self.depth
    }

    pub fn get_path(&self) -> PathBuf {
      match &self.base_path {
        Some(path) => path.to_path_buf(),
        None => current_dir().expect("Could not get directory")
      }
    }

    pub fn print(&self) {
        let mut first = true;
      for (_checksum, paths) in &self.entries {
        if paths.len() <= 1 && self.omit {
            continue;
        }
        if !first {
            println!("{}", self.seperator);
        }
        first = false;
        for path in paths {
          println!("{}", path);
        }
      }
    }
  
    pub fn parse(&mut self, dir_path: &Path, depth: u8) {
      if depth == 0 || !dir_path.is_dir() {
        return;
      }
  
      let entries = match fs::read_dir(dir_path) {
        Ok(entries) => entries,
        Err(_) => {
          eprint!("Could not read directory: {}", dir_path.display());
          return;
        },
      };
      
      for entry in entries.filter_map(Result::ok) {
        let path = entry.path();
        match entry.metadata() {
          Ok(metadata) => {
            if metadata.is_dir() {
              if self.recursive {
                self.parse(&path, depth - 1);
              }
            } else {
              self.insert(&path);
            }
          }
          Err(_) => eprint!("Could not read metadata for: {}", path.display())
        }
      }
    }
  
    fn insert(&mut self, path: &Path) {
      let path_str = match path.to_str() {
        Some(s) => s,
        None => {
          eprint!("Could not convert path to string: {}", path.display());
          return;
        }
      };
  
      if let Some(filename) = path.file_name().and_then(|f| f.to_str()) {
        if self.is_dot_file(filename) && !self.track_dot_files {
          return;
        }
      }
  
      let checksum = DupeLs::get_checksum(path_str);
  
      self.entries
        .entry(checksum)
        .or_insert_with(Vec::new)
        .push(path_str.to_string());
    }
  
    fn is_dot_file(&self, filename: &str) -> bool {
        filename.starts_with('.')
    }
  
    fn get_checksum(path: &str) -> md5::Digest {
      let mut file = fs::File::open(path).expect("Could not open file.");
      let mut context = md5::Context::new();
      let mut buffer = [0u8; 8192]; // TODO: configurable 8KB buffer size 
      loop {
        let bytes_read = match file.read(&mut buffer) {
          Ok(0) => break, // EOF
          Ok(n) => n,
          Err(_) => panic!("Error reading file: {}", path)
        };
        context.consume(&buffer[..bytes_read]);
      }
      context.compute()
    }
  
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
    assert!(f.track_dot_files);
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
    assert!(f.track_dot_files);
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
    assert!(f.track_dot_files);
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
    assert!(f.track_dot_files);
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
