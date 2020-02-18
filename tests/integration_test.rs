use assert_cmd::Command;

#[test]
fn success() {
    let mut cmd = Command::cargo_bin("stuff").unwrap();

    cmd.assert().success();
}
