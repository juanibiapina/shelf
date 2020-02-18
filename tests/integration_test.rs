use assert_cmd::Command;
use assert_fs::prelude::*;

#[test]
fn add_and_get_items() {
    let temp = assert_fs::TempDir::new().unwrap();
    let config_file = temp.child("foo.yml");

    Command::cargo_bin("shelf").unwrap()
        .arg("-c")
        .arg(config_file.path())
        .arg("add")
        .arg("key")
        .arg("value")
        .assert()
        .success();

    Command::cargo_bin("shelf").unwrap()
        .arg("-c")
        .arg(config_file.path())
        .arg("add")
        .arg("entry 2")
        .arg("word")
        .assert()
        .success();

    Command::cargo_bin("shelf").unwrap()
        .arg("-c")
        .arg(config_file.path())
        .arg("get")
        .arg("key")
        .assert()
        .success()
        .stdout("value\n");

    Command::cargo_bin("shelf").unwrap()
        .arg("-c")
        .arg(config_file.path())
        .arg("get")
        .arg("entry 2")
        .assert()
        .success()
        .stdout("word\n");
}
