mod cli;
mod dupels;

pub use cli::Cli;
pub use dupels::{DupeLs, DupeLsConfig};

impl From<&Cli> for DupeLsConfig {
  fn from(cli: &Cli) -> Self {
      let (recursive, depth) = match cli.depth {
          Some(depth) => (true, depth),
          None => (cli.recursive, 2),
      };
      DupeLsConfig {
          base_path: cli.file.clone(),
          track_dot_files: cli.all,
          recursive,
          depth,
          seperator: cli.seperator.clone(),
          max_threads: cli.max_threads,
          omit: cli.omit,
      }
  }
}

pub fn run(args: &Cli) {
  let config = DupeLsConfig::from(args);
  let mut dupels = DupeLs::new(config);
  dupels.parse();
  dupels.print();
}

#[cfg(test)]
mod tests {
    use crate::cli::Cli;
    use crate::dupels::DupeLsConfig;
    use std::path::PathBuf;

    #[test]
    fn test_from_cli_for_dupe_ls_config() {
        let cli = Cli {
            all: true,
            recursive: true,
            depth: Some(5),
            omit: false,
            seperator: "===".to_string(),
            max_threads: Some(1),
            file: Some(PathBuf::from("/tmp")),
        };
        let config = DupeLsConfig::from(&cli);
        assert_eq!(config.base_path, Some(PathBuf::from("/tmp")));
        assert!(config.track_dot_files);
        assert!(config.recursive);
        assert_eq!(config.depth, 5);
        assert!(!config.omit);
        assert_eq!(config.max_threads, Some(1));
        assert_eq!(config.seperator, "===");
    }
}
