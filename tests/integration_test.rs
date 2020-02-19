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
        .arg("group")
        .arg("key")
        .arg("value")
        .assert()
        .success();

    Command::cargo_bin("shelf").unwrap()
        .arg("-c")
        .arg(config_file.path())
        .arg("add")
        .arg("group")
        .arg("entry 2")
        .arg("word")
        .assert()
        .success();

    Command::cargo_bin("shelf").unwrap()
        .arg("-c")
        .arg(config_file.path())
        .arg("get")
        .arg("group")
        .arg("key")
        .assert()
        .success()
        .stdout("value\n");

    Command::cargo_bin("shelf").unwrap()
        .arg("-c")
        .arg(config_file.path())
        .arg("get")
        .arg("group")
        .arg("entry 2")
        .assert()
        .success()
        .stdout("word\n");

    Command::cargo_bin("shelf").unwrap()
        .arg("-c")
        .arg(config_file.path())
        .arg("get")
        .arg("group")
        .assert()
        .success()
        .stdout("key\tvalue\nentry 2\tword\n");
}
