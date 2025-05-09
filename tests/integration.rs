use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

// Helper struct for automatic cleanup
struct TestDir<'a> {
    path: &'a str,
}

impl<'a> Drop for TestDir<'a> {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(self.path);
    }
}

fn setup_test_files(dir: &str) -> TestDir {
    let _ = fs::remove_dir_all(dir);
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
    TestDir { path: dir }
}

#[test]
fn test_dupels_integration_basic_single_dir() {
    let dir = "test_files_basic";
    let _cleanup = setup_test_files(dir);

    let mut cmd = Command::cargo_bin("dupels").unwrap();
    cmd.args(&[dir]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(&format!("{}/1.txt", dir)))
        .stdout(predicate::str::contains(&format!("{}/2.txt", dir)))
        .stdout(predicate::str::contains(&format!("{}/3.txt", dir)))
        .stdout(predicate::str::contains(&format!("{}/.env.test", dir)).not());
}

#[test]
fn test_dupels_integration_single_dir_omit() {
    let dir = "test_files_omit";
    let _cleanup = setup_test_files(dir);

    let mut cmd = Command::cargo_bin("dupels").unwrap();
    cmd.args(&["-o", dir]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(&format!("{}/1.txt", dir)))
        .stdout(predicate::str::contains(&format!("{}/2.txt", dir)))
        .stdout(predicate::str::contains(&format!("{}/3.txt", dir)).not())
        .stdout(predicate::str::contains(&format!("{}/.env.test", dir)).not());
}

#[test]
fn test_dupels_integration_single_dir_a_flag_omit() {
    let dir = "test_files_a_flag_omit";
    let _cleanup = setup_test_files(dir);

    let mut cmd = Command::cargo_bin("dupels").unwrap();
    cmd.args(&["-o", "-a", dir]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(&format!("{}/1.txt", dir)))
        .stdout(predicate::str::contains(&format!("{}/2.txt", dir)))
        .stdout(predicate::str::contains(&format!("{}/3.txt", dir)).not())
        .stdout(predicate::str::contains(&format!("{}/.env.test", dir)).not());
}

#[test]
fn test_dupels_integration_nested_dir() {
    let dir = "test_files_nested";
    let _cleanup = setup_test_files(dir);

    let mut cmd = Command::cargo_bin("dupels").unwrap();
    cmd.args(&["-r", "-d", "2", dir]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(&format!("{}/1.txt", dir)))
        .stdout(predicate::str::contains(&format!("{}/2.txt", dir)))
        .stdout(predicate::str::contains(&format!("{}/3.txt", dir)))
        .stdout(predicate::str::contains(&format!(
            "{}/more_test_files/4.txt",
            dir
        )))
        .stdout(predicate::str::contains(&format!(
            "{}/more_test_files/5.txt",
            dir
        )))
        .stdout(predicate::str::contains(&format!(
            "{}/more_test_files/6.txt",
            dir
        )))
        .stdout(predicate::str::contains(&format!("{}/.env.test", dir)).not())
        .stdout(
            predicate::str::contains(&format!(
                "{}/more_test_files/more_more_test_files/7.txt",
                dir
            ))
            .not(),
        );
}

#[test]
fn test_dupels_integration_nested_dir_omit() {
    let dir = "test_files_nested_omit";
    let _cleanup = setup_test_files(dir);

    let mut cmd = Command::cargo_bin("dupels").unwrap();
    cmd.args(&["-r", "-d", "2", "-o", dir]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(&format!("{}/1.txt", dir)))
        .stdout(predicate::str::contains(&format!("{}/2.txt", dir)))
        .stdout(predicate::str::contains(&format!("{}/3.txt", dir)).not())
        .stdout(predicate::str::contains(&format!("{}/more_test_files/4.txt", dir)).not())
        .stdout(predicate::str::contains(&format!(
            "{}/more_test_files/5.txt",
            dir
        )))
        .stdout(predicate::str::contains(&format!("{}/more_test_files/6.txt", dir)).not())
        .stdout(predicate::str::contains(&format!("{}/.env.test", dir)).not())
        .stdout(
            predicate::str::contains(&format!(
                "{}/more_test_files/more_more_test_files/7.txt",
                dir
            ))
            .not(),
        );
}

#[test]
fn test_dupels_integration_two_nested_dir_a_flag_omit() {
    let dir = "test_files_two_nested_a_flag_omit";
    let _cleanup = setup_test_files(dir);

    let mut cmd = Command::cargo_bin("dupels").unwrap();
    cmd.args(&["-a", "-r", "-d", "3", "-o", dir]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(&format!("{}/1.txt", dir)))
        .stdout(predicate::str::contains(&format!("{}/2.txt", dir)))
        .stdout(predicate::str::contains(&format!("{}/3.txt", dir)).not())
        .stdout(predicate::str::contains(&format!("{}/more_test_files/4.txt", dir)).not())
        .stdout(predicate::str::contains(&format!(
            "{}/more_test_files/5.txt",
            dir
        )))
        .stdout(predicate::str::contains(&format!("{}/more_test_files/6.txt", dir)).not())
        .stdout(predicate::str::contains(&format!("{}/.env.test", dir)).not())
        .stdout(predicate::str::contains(&format!(
            "{}/more_test_files/more_more_test_files/7.txt",
            dir
        )))
        .stdout(
            predicate::str::contains(&format!(
                "{}/more_test_files/more_more_test_files/8.txt",
                dir
            ))
            .not(),
        );
}
