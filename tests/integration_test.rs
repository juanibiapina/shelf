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
        .arg("level 1")
        .arg("level 2")
        .arg("value")
        .assert()
        .success();

    Command::cargo_bin("shelf").unwrap()
        .arg("-c")
        .arg(config_file.path())
        .arg("add")
        .arg("entry 2")
        .arg("false")
        .assert()
        .success();

    Command::cargo_bin("shelf").unwrap()
        .arg("-c")
        .arg(config_file.path())
        .arg("list")
        .assert()
        .success()
        .stdout("level 1,level 2,value\nentry 2,false\n");
}
