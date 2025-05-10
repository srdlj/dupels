use crossbeam_channel::{unbounded, Sender};
use md5;
use num_cpus;
use std::{
    collections::HashMap,
    env, fs,
    io::Read,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    thread,
};

const MAX_THREAD_LIMIT: usize = 32;

/// Configuration for the DupeLs duplicate file finder.
///
/// # Fields
/// - `base_path`: The root directory to start searching for duplicates. If `None`, the current directory is used.
/// - `track_dot_files`: If `true`, include files and directories whose names begin with a dot (`.`).
/// - `recursive`: If `true`, search subdirectories recursively up to `depth`.
/// - `depth`: The maximum recursion depth for directory traversal.
/// - `seperator`: String used to separate groups of duplicate files in the output.
/// - `omit`: If `true`, omit groups that contain only a single file from the output.
/// - `max_threads`: The maximum number of threads to use for processing files.
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
///     max_threads: Some(4),
/// };
/// ```
pub struct DupeLsConfig {
    pub base_path: Option<PathBuf>,
    pub track_dot_files: bool,
    pub recursive: bool,
    pub depth: usize,
    pub seperator: String,
    pub omit: bool,
    pub max_threads: Option<usize>,
}

impl DupeLsConfig {
    pub fn resolved_base_path(&self) -> PathBuf {
        match &self.base_path {
            Some(path) => path.clone(),
            None => env::current_dir().expect("Could not get current directory"),
        }
    }
}

/// A struct for finding duplicate files.
/// # Fields
/// - `base_path`: The base path to start searching for duplicates.
/// - `track_dot_files`: Whether to track dot files (hidden files).
/// - `recursive`: Whether to search recursively in subdirectories.
/// - `depth`: The depth of recursion.
/// - `seperator`: The string used to separate duplicate file groups in output.
/// - `omit`: Whether to omit single files from the output.
/// - `max_threads`: The maximum number of threads to use for processing files.
/// - `entries`: A map of checksums to file paths.
pub struct DupeLs {
    base_path: PathBuf,
    track_dot_files: bool,
    recursive: bool,
    depth: usize,
    seperator: String,
    omit: bool,
    max_threads: usize,
    entries: Arc<Mutex<HashMap<md5::Digest, Vec<String>>>>,
}

impl DupeLs {
    pub fn new(config: DupeLsConfig) -> DupeLs {
        let max_threads = config
            .max_threads
            .unwrap_or_else(|| num_cpus::get())
            .min(MAX_THREAD_LIMIT);
        DupeLs {
            base_path: config.resolved_base_path(),
            track_dot_files: config.track_dot_files,
            recursive: config.recursive,
            depth: config.depth,
            seperator: config.seperator,
            omit: config.omit,
            max_threads,
            entries: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn get_sorted_checksums(&self) -> Vec<md5::Digest> {
        let map = self.entries.lock().unwrap();
        let mut checksums: Vec<_> = map.keys().cloned().collect();
        checksums.sort_by(|a, b| a.as_ref().cmp(b.as_ref()));
        checksums
    }

    pub fn get_output_vec(&self) -> Vec<String> {
        let checksums = self.get_sorted_checksums();
        let map = self.entries.lock().unwrap();
        let mut lines = Vec::new();
        let mut first = true;
        for checksum in checksums {
            let paths = &map[&checksum];
            if paths.len() <= 1 && self.omit {
                continue;
            }
            if !first {
                lines.push(self.seperator.clone());
            }
            first = false;
            for path in paths {
                lines.push(path.clone());
            }
        }
        lines
    }

    pub fn get_output_string(&self) -> String {
        let lines = self.get_output_vec();
        lines.join("\n")
    }

    fn walk_and_send(&self, dir_path: &Path, depth: usize, s: &Sender<String>) {
        if depth == 0 || !dir_path.is_dir() {
            return;
        }
        let entries = match fs::read_dir(dir_path) {
            Ok(entries) => entries,
            Err(_) => return,
        };
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_dir() {
                    if self.recursive {
                        self.walk_and_send(&path, depth - 1, s);
                    }
                } else {
                    if let Some(filename) = path.file_name().and_then(|f| f.to_str()) {
                        if self.is_dot_file(filename) && !self.track_dot_files {
                            continue;
                        }
                    }
                    let _ = s.send(path.to_string_lossy().to_string());
                }
            }
        }
    }

    pub fn parse(&mut self) {
        let (s, r) = unbounded::<String>();
        let entries = Arc::clone(&self.entries);

        let mut handles: Vec<thread::JoinHandle<()>> = Vec::new();
        for _ in 0..self.max_threads {
            let recv: crossbeam_channel::Receiver<String> = r.clone();
            let entries = Arc::clone(&entries);
            handles.push(thread::spawn(move || {
                for path in recv.iter() {
                    match DupeLs::get_checksum(&path) {
                        Ok(checksum) => {
                            let mut map = entries.lock().unwrap();
                            map.entry(checksum).or_insert_with(Vec::new).push(path);        
                        }
                        Err(err_msg) => {
                            eprintln!("{}", err_msg);
                        }
                    }
                }
            }));
        }

        self.walk_and_send(&self.base_path, self.depth, &s);

        drop(s);

        for handle in handles {
            let _ = handle.join();
        }
    }

    fn is_dot_file(&self, filename: &str) -> bool {
        filename.starts_with('.')
    }

    fn get_checksum(path: &str) -> Result<md5::Digest, String> {
        let mut file = fs::File::open(path)
            .map_err(|e| format!("Could not open file '{}': {}", path, e))?;
        let mut context = md5::Context::new();
        let mut buffer = [0u8; 8192];
        loop {
            let bytes_read = file.read(&mut buffer)
                .map_err(|e| format!("Error reading file '{}': {}", path, e))?;
            if bytes_read == 0 {
                break;
            }
            context.consume(&buffer[..bytes_read]);
        }
        Ok(context.compute())
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use tempfile::tempdir;

    #[cfg(unix)]
    use std::os::unix::fs::PermissionsExt;

    // Set up file with no read permissions (Unix only)
    #[cfg(unix)]
    fn create_no_read_permission_file(dir: &std::path::Path, name: &str) -> std::path::PathBuf {
        use std::fs::Permissions;
        let file_path = dir.join(name);
        let mut file = File::create(&file_path).unwrap();
        file.write_all(b"secret").unwrap();
        fs::set_permissions(&file_path, Permissions::from_mode(0o000)).unwrap();
        file_path
    }

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
        files.push(create_test_file(
            &subdir,
            "6.txt",
            "This is another unique file",
        ));
        let subsubdir = subdir.join("more_more_files");
        fs::create_dir(&subsubdir).unwrap();
        files.push(create_test_file(&subsubdir, "7.txt", "Hello"));
        files.push(create_test_file(&subsubdir, "8.txt", "Last one"));
        (dir, files)
    }

    #[test]
    fn init_test_with_no_file_no_r_flag() {
        let config = DupeLsConfig {
            base_path: None,
            track_dot_files: true,
            recursive: false,
            depth: 2,
            omit: false,
            max_threads: Some(1),
            seperator: "---".to_string(),
        };
        let d = DupeLs::new(config);
        assert!(d.track_dot_files);
        assert!(!d.recursive);
        assert_eq!(d.depth, 2);
        assert!(!d.omit);
        assert!(d.base_path.is_dir());
        assert_eq!(d.base_path, env::current_dir().unwrap());
        assert_eq!("---".to_string(), d.seperator);
    }

    #[test]
    fn init_test_with_r_flag() {
        let (dir, _files) = setup_test_files();
        let config = DupeLsConfig {
            base_path: Some(PathBuf::from(dir.path().to_path_buf())),
            track_dot_files: true,
            recursive: true,
            depth: 2,
            omit: false,
            max_threads: Some(1),
            seperator: "---".to_string(),
        };
        let d = DupeLs::new(config);
        assert!(d.track_dot_files);
        assert!(d.recursive);
        assert_eq!(d.depth, 2);
        assert!(!d.omit);
        assert!(d.base_path.is_dir());
        assert!(d.base_path.exists());
        assert_eq!(d.base_path, dir.path().to_path_buf());
        assert_eq!("---".to_string(), d.seperator);
    }

    #[test]
    fn init_test_with_rd_flags() {
        let (dir, _files) = setup_test_files();
        let config = DupeLsConfig {
            base_path: Some(PathBuf::from(dir.path().to_path_buf())),
            track_dot_files: true,
            recursive: true,
            depth: 2,
            omit: false,
            max_threads: Some(1),
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
    fn test_thread_default() {
        let (dir, _files) = setup_test_files();
        let config = DupeLsConfig {
            base_path: Some(PathBuf::from(dir.path().to_path_buf())),
            track_dot_files: true,
            recursive: true,
            depth: 2,
            omit: false,
            max_threads: None,
            seperator: "hi".to_string(),
        };
        let d = DupeLs::new(config);
        assert!(d.max_threads <= MAX_THREAD_LIMIT);
    }

    #[test]
    fn test_legal_specified_max_thread() {
        let (dir, _files) = setup_test_files();
        let config = DupeLsConfig {
            base_path: Some(PathBuf::from(dir.path().to_path_buf())),
            track_dot_files: true,
            recursive: true,
            depth: 2,
            omit: false,
            max_threads: Some(MAX_THREAD_LIMIT - 1),
            seperator: "hi".to_string(),
        };
        let d = DupeLs::new(config);
        assert!(d.max_threads <= MAX_THREAD_LIMIT - 1);
    }

    #[test]
    fn test_thread_safe_guard() {
        let (dir, _files) = setup_test_files();
        let config = DupeLsConfig {
            base_path: Some(PathBuf::from(dir.path().to_path_buf())),
            track_dot_files: true,
            recursive: true,
            depth: 2,
            omit: false,
            max_threads: Some(num_cpus::get() + 10),
            seperator: "hi".to_string(),
        };
        let d = DupeLs::new(config);
        assert!(d.max_threads <= MAX_THREAD_LIMIT);
    }

    #[test]
    fn test_md5_checksum() {
        let (_dir, files) = setup_test_files();
        let file_path = files.iter().find(|p| p.ends_with("1.txt")).unwrap();
        let expected_dupe_md5: &str = "8b1a9953c4611296a827abf8c47804d7";
        assert_eq!(
            format!("{:x}", DupeLs::get_checksum(file_path.to_str().unwrap()).unwrap()),
            expected_dupe_md5
        );
    }

    #[test]
    fn test_get_checksum_bad_path_fail() {
        let invalid_path = "/invalid/path/to/nonexistent/file.txt";
        let result = DupeLs::get_checksum(invalid_path);
        assert!(result.is_err());
        let error_message = result.unwrap_err();
        assert!(error_message.contains(&format!("Could not open file '{}'", invalid_path)));
    }

    #[test]
    fn test_get_checksum_on_directory() {
        let dir = tempdir().unwrap();
        let result = super::DupeLs::get_checksum(dir.path().to_str().unwrap());
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_no_r_1_thread() {
        let (dir, _files) = setup_test_files();
        let config = DupeLsConfig {
            base_path: Some(dir.path().to_path_buf()),
            track_dot_files: true,
            recursive: false,
            depth: 2,
            omit: false,
            max_threads: Some(1),
            seperator: "---".to_string(),
        };
        let mut d = DupeLs::new(config);
        d.parse();
        let map = d.entries.lock().unwrap();
        assert_eq!(map.len(), 3);
    }

    #[test]
    fn test_parse_no_r_default_threads() {
        let (dir, _files) = setup_test_files();
        let config = DupeLsConfig {
            base_path: Some(dir.path().to_path_buf()),
            track_dot_files: true,
            recursive: false,
            depth: 2,
            omit: false,
            max_threads: None,
            seperator: "---".to_string(),
        };
        let mut d = DupeLs::new(config);
        d.parse();
        let map = d.entries.lock().unwrap();
        assert_eq!(map.len(), 3);
    }

    #[test]
    fn test_parse_r_d_2_1_thread() {
        let (dir, _files) = setup_test_files();
        let config = DupeLsConfig {
            base_path: Some(dir.path().to_path_buf()),
            track_dot_files: true,
            recursive: true,
            depth: 2,
            omit: false,
            max_threads: Some(1),
            seperator: "---".to_string(),
        };
        let mut d = DupeLs::new(config);
        d.parse();
        let map = d.entries.lock().unwrap();
        assert_eq!(map.len(), 5);
    }

    #[test]
    fn test_parse_r_d_2_default_threads() {
        let (dir, _files) = setup_test_files();
        let config = DupeLsConfig {
            base_path: Some(dir.path().to_path_buf()),
            track_dot_files: true,
            recursive: true,
            depth: 2,
            omit: false,
            max_threads: None,
            seperator: "---".to_string(),
        };
        let mut d = DupeLs::new(config);
        d.parse();
        let map = d.entries.lock().unwrap();
        assert_eq!(map.len(), 5);
    }
    #[test]
    fn test_get_sorted_checksums() {
        let (dir, _files) = setup_test_files();
        let config = DupeLsConfig {
            base_path: Some(dir.path().to_path_buf()),
            track_dot_files: true,
            recursive: true,
            depth: 2,
            omit: false,
            max_threads: None,
            seperator: "---".to_string(),
        };

        let mut d = DupeLs::new(config);
        d.parse();
        let sorted_checksums = d.get_sorted_checksums();
        let map = d.entries.lock().unwrap();

        // Check that the checksums are sorted
        assert!(sorted_checksums
            .windows(2)
            .all(|w| w[0].as_ref() <= w[1].as_ref()));

        // Check that all checksums in the map are present in the sorted checksums
        for checksum in map.keys() {
            assert!(sorted_checksums.contains(checksum));
        }

        assert_eq!(sorted_checksums.len(), map.len());
    }

    #[test]
    fn test_get_output_vec() {
        let (dir, _files) = setup_test_files();
        let config = DupeLsConfig {
            base_path: Some(dir.path().to_path_buf()),
            track_dot_files: true,
            recursive: false,
            depth: 1,
            omit: false,
            max_threads: None,
            seperator: "---".to_string(),
        };

        let mut d = DupeLs::new(config);
        d.parse();
        let mut output_vec = d.get_output_vec();
        output_vec.retain(|line| line != &d.seperator);

        // Check that the output vector contains the expected number of lines
        assert_eq!(output_vec.len(), 4);
    }

    #[test]
    fn test_get_output_str() {
        let (dir, _files) = setup_test_files();
        let config = DupeLsConfig {
            base_path: Some(dir.path().to_path_buf()),
            track_dot_files: true,
            recursive: false,
            depth: 1,
            omit: false,
            max_threads: None,
            seperator: "---".to_string(),
        };

        let mut d = DupeLs::new(config);
        d.parse();
        let output_str = d.get_output_string();
        let mut lines: Vec<&str> = output_str.split('\n').collect();
        lines.retain(|line| line != &d.seperator);

        // Check that the output string contains the expected number of lines
        assert_eq!(lines.len(), 4);
    }

    #[test]
    #[cfg(unix)]
    fn test_parse_dir_with_permission_denied_file() {
        let dir = tempdir().unwrap();
        let _file_path = create_no_read_permission_file(dir.path(), "no_read.txt");
        let config = DupeLsConfig {
            base_path: Some(dir.path().to_path_buf()),
            track_dot_files: true,
            recursive: false,
            depth: 1,
            omit: false,
            max_threads: None,
            seperator: "---".to_string(),
        };
        let mut d = DupeLs::new(config);
        d.parse();
        let output_str = d.get_output_string();
        assert_eq!(output_str.len(), 0);
    }
}
