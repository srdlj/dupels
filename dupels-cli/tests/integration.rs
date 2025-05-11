use assert_cmd::Command;
use predicates::prelude::*;
use std::fs::{self, File};
use std::io::Write;
use tempfile::tempdir;
use dupels_lib::{MAX_THREAD_LIMIT, DEFAULT_DEPTH};

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

// Se up no dir with no read permissions (Unix only)
#[cfg(unix)]
fn create_no_read_permission_dir(dir: &std::path::Path, name: &str) -> std::path::PathBuf {
    let dir_path = dir.join(name);
    fs::create_dir(&dir_path).unwrap();
    fs::set_permissions(&dir_path, fs::Permissions::from_mode(0o000)).unwrap();
    dir_path
}

fn setup_test_files() -> tempfile::TempDir {
    let tmp_dir = tempdir().unwrap();
    let dir = tmp_dir.path().to_str().unwrap();
    fs::create_dir_all(format!("{}/more_test_files/more_more_test_files", dir)).unwrap();
    fs::write(format!("{}/1.txt", dir), "Hello").unwrap();
    fs::write(format!("{}/2.txt", dir), "Hello").unwrap();
    fs::write(format!("{}/3.txt", dir), "Hello World").unwrap();
    fs::write(format!("{}/.env.test", dir), ".env test").unwrap();
    fs::write(
        format!("{}/more_test_files/4.txt", dir),
        "This is a unique file",
    )
    .unwrap();
    fs::write(format!("{}/more_test_files/5.txt", dir), "Hello").unwrap();
    fs::write(
        format!("{}/more_test_files/6.txt", dir),
        "This is another unique file",
    )
    .unwrap();
    fs::write(
        format!("{}/more_test_files/more_more_test_files/7.txt", dir),
        "Hello",
    )
    .unwrap();
    fs::write(
        format!("{}/more_test_files/more_more_test_files/8.txt", dir),
        "Last one",
    )
    .unwrap();
    tmp_dir
}

#[test]
fn test_help() {
    let mut cmd = Command::cargo_bin("dupels").unwrap();
    cmd.arg("--help");
    cmd.assert().success();
}

#[test]
fn test_help_thread_and_depth_defaults() {
    let mut cmd = Command::cargo_bin("dupels").unwrap();
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(MAX_THREAD_LIMIT.to_string()))
        .stdout(predicate::str::contains(   DEFAULT_DEPTH.to_string()));
}

#[test]
fn test_dupels_integration_basic_single_dir() {
    let dir = setup_test_files();
    let p = dir.path().to_str().unwrap();
    let mut cmd = Command::cargo_bin("dupels").unwrap();
    cmd.args(&[p]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(&format!("{}/1.txt", p)))
        .stdout(predicate::str::contains(&format!("{}/2.txt", p)))
        .stdout(predicate::str::contains(&format!("{}/3.txt", p)))
        .stdout(predicate::str::contains(&format!("{}/.env.test", p)).not());
}

#[test]
fn test_dupels_integration_single_dir_omit() {
    let dir = setup_test_files();
    let p = dir.path().to_str().unwrap();

    let mut cmd = Command::cargo_bin("dupels").unwrap();
    cmd.args(&["-o", p]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(&format!("{}/1.txt", p)))
        .stdout(predicate::str::contains(&format!("{}/2.txt", p)))
        .stdout(predicate::str::contains(&format!("{}/3.txt", p)).not())
        .stdout(predicate::str::contains(&format!("{}/.env.test", p)).not());
}

#[test]
fn test_dupels_integration_single_dir_a_flag_omit() {
    let dir = setup_test_files();
    let p = dir.path().to_str().unwrap();

    let mut cmd = Command::cargo_bin("dupels").unwrap();
    cmd.args(&["-o", "-a", p]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(&format!("{}/1.txt", p)))
        .stdout(predicate::str::contains(&format!("{}/2.txt", p)))
        .stdout(predicate::str::contains(&format!("{}/3.txt", p)).not())
        .stdout(predicate::str::contains(&format!("{}/.env.test", p)).not());
}

#[test]
fn test_dupels_integration_nested_dir() {
    let dir = setup_test_files();
    let p = dir.path().to_str().unwrap();

    let mut cmd = Command::cargo_bin("dupels").unwrap();
    cmd.args(&["-r", "-d", "1", p]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(&format!("{}/1.txt", p)))
        .stdout(predicate::str::contains(&format!("{}/2.txt", p)))
        .stdout(predicate::str::contains(&format!("{}/3.txt", p)))
        .stdout(predicate::str::contains(&format!(
            "{}/more_test_files/4.txt",
            p
        )))
        .stdout(predicate::str::contains(&format!(
            "{}/more_test_files/5.txt",
            p
        )))
        .stdout(predicate::str::contains(&format!(
            "{}/more_test_files/6.txt",
            p
        )))
        .stdout(predicate::str::contains(&format!("{}/.env.test", p)).not())
        .stdout(
            predicate::str::contains(&format!(
                "{}/more_test_files/more_more_test_files/7.txt",
                p
            ))
            .not(),
        );
}

#[test]
fn test_dupels_integration_nested_dir_omit() {
    let dir = setup_test_files();
    let p = dir.path().to_str().unwrap();

    let mut cmd = Command::cargo_bin("dupels").unwrap();
    cmd.args(&["-r", "-d", "1", "-o", p]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(&format!("{}/1.txt", p)))
        .stdout(predicate::str::contains(&format!("{}/2.txt", p)))
        .stdout(predicate::str::contains(&format!("{}/3.txt", p)).not())
        .stdout(predicate::str::contains(&format!("{}/more_test_files/4.txt", p)).not())
        .stdout(predicate::str::contains(&format!(
            "{}/more_test_files/5.txt",
            p
        )))
        .stdout(predicate::str::contains(&format!("{}/more_test_files/6.txt", p)).not())
        .stdout(predicate::str::contains(&format!("{}/.env.test", p)).not())
        .stdout(
            predicate::str::contains(&format!(
                "{}/more_test_files/more_more_test_files/7.txt",
                p
            ))
            .not(),
        );
}

#[test]
fn test_dupels_integration_two_nested_dir_a_flag_omit() {
    let dir = setup_test_files();
    let p = dir.path().to_str().unwrap();

    let mut cmd = Command::cargo_bin("dupels").unwrap();
    cmd.args(&["-a", "-r", "-d", "2", "-o", p]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(&format!("{}/1.txt", p)))
        .stdout(predicate::str::contains(&format!("{}/2.txt", p)))
        .stdout(predicate::str::contains(&format!("{}/3.txt", p)).not())
        .stdout(predicate::str::contains(&format!("{}/more_test_files/4.txt", p)).not())
        .stdout(predicate::str::contains(&format!(
            "{}/more_test_files/5.txt",
            p
        )))
        .stdout(predicate::str::contains(&format!("{}/more_test_files/6.txt", p)).not())
        .stdout(predicate::str::contains(&format!("{}/.env.test", p)).not())
        .stdout(predicate::str::contains(&format!(
            "{}/more_test_files/more_more_test_files/7.txt",
            p
        )))
        .stdout(
            predicate::str::contains(&format!(
                "{}/more_test_files/more_more_test_files/8.txt",
                p
            ))
            .not(),
        );
}

#[test]
#[cfg(unix)]
fn test_dupels_integration_no_read_permission_file() {
    let dir = tempdir().unwrap();
    let p = dir.path().to_str().unwrap();
    let _file_path = create_no_read_permission_file(dir.path(), "no_read.txt");

    let mut cmd = Command::cargo_bin("dupels").unwrap();
    cmd.args(&[p]);

    cmd.assert()
        .success()
        .stderr(predicate::str::contains(&format!("Could not open file '{}/no_read.txt", p)))
        .stderr(predicate::str::contains("Permission denied"));
}

#[test]
#[cfg(unix)]
fn test_dupels_integration_no_read_permission_dir() {
    let dir = tempdir().unwrap();
    let p = dir.path().to_str().unwrap();
    let _dir_path = create_no_read_permission_dir(dir.path(), "no_read_dir");

    let mut cmd = Command::cargo_bin("dupels").unwrap();
    cmd.args(&[p]);

    cmd.assert()
        .success();  // Directory is still readable, just check it passes.
}