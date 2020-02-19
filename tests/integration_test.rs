use assert_cmd::Command;
use assert_fs::prelude::*;

struct Context {
    config_file_path: String,
}

fn assert(c: &Context, args: &[&str]) -> assert_cmd::assert::Assert {
    Command::cargo_bin("shelf").unwrap()
        .arg("-c")
        .arg(&c.config_file_path)
        .args(args)
        .assert()
}

#[test]
fn add_and_get_items() {
    let temp = assert_fs::TempDir::new().unwrap();
    let config_file = temp.child("foo.yml");

    let c = Context { config_file_path: config_file.path().to_str().unwrap().to_owned() };

    assert(&c, &["add", "group", "key", "value"])
        .success();

    assert(&c, &["add", "group", "entry 2", "word"])
        .success();

    assert(&c, &["get", "group", "key"])
        .success()
        .stdout("value\n");

    assert(&c, &["get", "group", "entry 2"])
        .success()
        .stdout("word\n");

    assert(&c, &["get", "group"])
        .success()
        .stdout("entry 2\tword\nkey\tvalue\n");
}

#[test]
fn get_a_map() {
    let temp = assert_fs::TempDir::new().unwrap();
    let config_file = temp.child("foo.yml");

    let c = Context { config_file_path: config_file.path().to_str().unwrap().to_owned() };

    assert(&c, &["add", "a", "b", "c"])
        .success();

    assert(&c, &["add", "a", "d", "e"])
        .success();

    assert(&c, &["get"])
        .success()
        .stdout("a\t...\n");

    assert(&c, &["get", "a"])
        .success()
        .stdout("b\tc\nd\te\n");
}
