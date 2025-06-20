mod cli;
mod gui;
mod dupels;

pub use cli::Cli;
pub use gui::Gui;
pub use dupels::{DupeLs, DupeLsConfig};

pub const MAX_THREAD_LIMIT: usize = 32;
pub const DEFAULT_DEPTH: usize = 2;
const CHECKSUM_READ_BUFFER_SIZE: usize = 8192;

impl From<&Cli> for DupeLsConfig {
    fn from(cli: &Cli) -> Self {
        let (recursive, depth) = match cli.depth {
            Some(depth) => (true, depth),
            None => (cli.recursive, DEFAULT_DEPTH),
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

impl From<&Gui> for DupeLsConfig {
    fn from(gui: &Gui) -> Self {

        // Set recursive based on user input for depth.
        let (recursive, depth) = match gui.depth {
            0 => (false, DEFAULT_DEPTH),
            d => (true, d),
        };
        DupeLsConfig {
            base_path: Some(gui.directory.clone().into()),
            track_dot_files: gui.all,
            recursive,
            depth,
            seperator: "===".to_string(),
            max_threads: None, // Let DupleLs resolve thread count.
            omit: gui.omit,
        }
    }
}

pub fn run_cli(args: &Cli) -> String{
    let config = DupeLsConfig::from(args);
    let mut dupels = DupeLs::new(config);
    dupels.parse();
    dupels.get_output_string()
}

pub fn run_gui(gui: &Gui) -> Vec<String> {
    let config = DupeLsConfig::from(gui);
    let mut dupels = DupeLs::new(config);
    dupels.parse();
    dupels.get_output_vec()
}

#[cfg(test)]
mod tests {
    use crate::cli::Cli;
    use crate::gui::Gui;
    use crate::dupels::DupeLsConfig;
    use crate::run_cli;
    use std::fs::File;
    use std::ops::Not;
    use std::path::PathBuf;
    use std::io::Write;
    use tempfile::tempdir;

    fn setup_test_file() -> (tempfile::TempDir, std::path::PathBuf) {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test_file.txt");
        let mut file = File::create(&file_path).unwrap();
        file.write_all("Hello World".as_bytes()).unwrap();
        (dir, file_path)
    }

    #[test]
    fn test_from_cli_for_dupe_ls_config_recursive_depth() {
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

    #[test]
    fn test_from_cli_for_dupe_ls_config_not_recursive() {
        let cli = Cli {
            all: true,
            recursive: false,
            depth: None,
            omit: false,
            seperator: "===".to_string(),
            max_threads: Some(1),
            file: Some(PathBuf::from("/tmp")),
        };
        let config = DupeLsConfig::from(&cli);
        assert!(config.recursive.not());
        assert_eq!(config.depth, 2);
    }

    #[test]
    fn test_from_cli_for_dupe_implicit_recursive() {
        let cli = Cli {
            all: true,
            recursive: false,
            depth: Some(10),
            omit: false,
            seperator: "===".to_string(),
            max_threads: Some(1),
            file: Some(PathBuf::from("/tmp")),
        };
        let config = DupeLsConfig::from(&cli);
        assert!(config.recursive);
        assert_eq!(config.depth, 10);
    }

    #[test]
    fn test_run() {
        let (dir, file) = setup_test_file();
        let cli = Cli {
            all: true,
            recursive: false,
            depth: Some(2),
            omit: false,
            seperator: "===".to_string(),
            max_threads: Some(1),
            file: Some(PathBuf::from(dir.path().to_path_buf())),
        };
        let output = run_cli(&cli);
        assert_eq!(output, file.to_string_lossy().to_string());
    }

}
