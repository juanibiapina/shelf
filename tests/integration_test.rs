use assert_cmd::Command;
use assert_fs::prelude::*;

#[test]
fn add_and_list_items() {
    let temp = assert_fs::TempDir::new().unwrap();
    let config_file = temp.child("foo.txt");
    config_file.touch().unwrap();

    Command::cargo_bin("shelf").unwrap()
        .arg("-c")
        .arg(config_file.path())
        .arg("add")
        .arg("entry")
        .arg("1")
        .assert()
        .success();

    Command::cargo_bin("shelf").unwrap()
        .arg("-c")
        .arg(config_file.path())
        .arg("add")
        .arg("entry 2")
        .assert()
        .success();

    Command::cargo_bin("shelf").unwrap()
        .arg("-c")
        .arg(config_file.path())
        .arg("list")
        .assert()
        .success()
        .stdout("entry 1\nentry 2\n");
}
