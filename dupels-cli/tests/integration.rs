use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

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
    cmd.args(&["-r", "-d", "2", p]);

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
    cmd.args(&["-r", "-d", "2", "-o", p]);

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
    cmd.args(&["-a", "-r", "-d", "3", "-o", p]);

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
