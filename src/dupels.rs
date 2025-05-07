use std::{collections::HashMap, env::current_dir, fs, io::Read, path::{Path, PathBuf}};
use md5;

/// Configuration for the DupeLs duplicate file finder.
///
/// `DupeLsConfig` encapsulates all options needed to control the behavior of the duplicate search,
/// independent of any CLI or UI layer. This struct is used to initialize a [`DupeLs`] instance.
///
/// # Fields
/// - `base_path`: The root directory to start searching for duplicates. If `None`, the current directory is used.
/// - `track_dot_files`: If `true`, include files and directories whose names begin with a dot (`.`).
/// - `recursive`: If `true`, search subdirectories recursively up to `depth`.
/// - `depth`: The maximum recursion depth for directory traversal.
/// - `seperator`: String used to separate groups of duplicate files in the output.
/// - `omit`: If `true`, omit groups that contain only a single file from the output.
///
/// # Example
/// ```
/// use dupels::DupeLsConfig;
/// use std::path::PathBuf;
///
/// let config = DupeLsConfig {
///     base_path: Some(PathBuf::from("/tmp")),
///     track_dot_files: true,
///     recursive: true,
///     depth: 3,
///     seperator: "---".to_string(),
///     omit: false,
/// };
/// ```
pub struct DupeLsConfig {
  pub base_path: Option<PathBuf>,
  pub track_dot_files: bool,
  pub recursive: bool,
  pub depth: u8,
  pub seperator: String,
  pub omit: bool,
}

/// The `DupeLs` struct is responsible for finding duplicate files in a directory.
/// It uses the MD5 checksum of files to identify duplicates.
///
/// # Fields
/// - `base_path`: The base path to start searching for duplicates.
/// - `track_dot_files`: Whether to track dot files (hidden files).
/// - `recursive`: Whether to search recursively in subdirectories.
/// - `depth`: The depth of recursion.
/// - `seperator`: The string used to separate duplicate file groups in output.
/// - `omit`: Whether to omit single files from the output.
/// - `entries`: A map of checksums to file paths.
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
    pub fn new(config: DupeLsConfig) -> DupeLs {
      DupeLs {
          base_path: config.base_path,
          track_dot_files: config.track_dot_files,
          recursive: config.recursive,
          depth: config.depth,
          seperator: config.seperator,
          omit: config.omit,
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
  use tempfile::tempdir;
  use std::fs::{self, File};
  use std::io::Write;

  fn create_test_file(dir: &std::path::Path, name: &str, contents: &str) -> std::path::PathBuf {
      let file_path = dir.join(name);
      let mut file = File::create(&file_path).unwrap();
      file.write_all(contents.as_bytes()).unwrap();
      file_path
  }

  fn setup_test_files() -> (tempfile::TempDir, Vec<std::path::PathBuf>) {
      let dir = tempdir().unwrap();
      let mut files = Vec::new();
      files.push(create_test_file(dir.path(), "1.txt", "Hello"));
      files.push(create_test_file(dir.path(), "2.txt", "Hello"));
      files.push(create_test_file(dir.path(), "3.txt", "Hello World"));
      files.push(create_test_file(dir.path(), ".env.test", ".env test"));
      let subdir = dir.path().join("more_files");
      fs::create_dir(&subdir).unwrap();
      files.push(create_test_file(&subdir, "4.txt", "This is a unique file"));
      files.push(create_test_file(&subdir, "5.txt", "Hello"));
      files.push(create_test_file(&subdir, "6.txt", "This is another unique file"));
      let subsubdir = subdir.join("more_more_files");
      fs::create_dir(&subsubdir).unwrap();
      files.push(create_test_file(&subsubdir, "7.txt", "Hello"));
      files.push(create_test_file(&subsubdir, "8.txt", "Last one"));
      (dir, files)
  }

  #[test]
  fn init_test_no_r_flag() {
      let config = DupeLsConfig {
          base_path: None,
          track_dot_files: true,
          recursive: false,
          depth: 2,
          omit: false,
          seperator: "---".to_string(),
      };
      let f = DupeLs::new(config);
      assert!(f.track_dot_files);
      assert!(!f.recursive);
      assert_eq!(f.depth, 2);
      assert!(!f.omit);
      assert_eq!("---".to_string(), f.seperator);
  }

  #[test]
  fn init_test_with_no_file() {
      let config = DupeLsConfig {
          base_path: None,
          track_dot_files: true,
          recursive: false,
          depth: 2,
          omit: false,
          seperator: "---".to_string(),
      };
      let f = DupeLs::new(config);
      assert!(f.track_dot_files);
      assert!(!f.recursive);
      assert_eq!(f.depth, 2);
      assert!(!f.omit);
      assert_eq!("---".to_string(), f.seperator);
  }

  #[test]
  fn init_test_with_r_flag() {
      let config = DupeLsConfig {
          base_path: Some(PathBuf::from("files")),
          track_dot_files: true,
          recursive: true,
          depth: 2,
          omit: false,
          seperator: "---".to_string(),
      };
      let f = DupeLs::new(config);
      assert!(f.track_dot_files);
      assert!(f.recursive);
      assert_eq!(f.depth, 2);
      assert!(!f.omit);
      assert_eq!("---".to_string(), f.seperator);
  }

  #[test]
  fn init_test_with_rd_flags() {
      let config = DupeLsConfig {
          base_path: Some(PathBuf::from("files")),
          track_dot_files: true,
          recursive: true,
          depth: 2,
          omit: false,
          seperator: "hi".to_string(),
      };
      let f = DupeLs::new(config);
      assert!(f.track_dot_files);
      assert!(f.recursive);
      assert_eq!(f.depth, 2);
      assert!(!f.omit);
      assert_eq!("hi".to_string(), f.seperator);
  }

  #[test]
  fn test_md5_checksum() {
      let (_dir, files) = setup_test_files();
      let file_path = files.iter().find(|p| p.ends_with("1.txt")).unwrap();
      let expected_dupe_md5: &str = "8b1a9953c4611296a827abf8c47804d7";
      assert_eq!(
          format!("{:x}", DupeLs::get_checksum(file_path.to_str().unwrap())),
          expected_dupe_md5
      );
  }

  #[test]
  fn test_parse_no_r() {
      let (dir, _files) = setup_test_files();
      let config = DupeLsConfig {
          base_path: Some(dir.path().to_path_buf()),
          track_dot_files: true,
          recursive: false,
          depth: 2,
          omit: false,
          seperator: "---".to_string(),
      };
      let mut d = DupeLs::new(config);
      d.parse(&d.get_path(), d.depth);
      assert_eq!(d.entries.len(), 3);
  }

  #[test]
  fn test_parse_r_d_2() {
      let (dir, _files) = setup_test_files();
      let config = DupeLsConfig {
          base_path: Some(dir.path().to_path_buf()),
          track_dot_files: true,
          recursive: true,
          depth: 2,
          omit: false,
          seperator: "---".to_string(),
      };
      let mut d = DupeLs::new(config);
      d.parse(&d.get_path(), d.depth);
      assert_eq!(d.entries.len(), 5);
  }
}
