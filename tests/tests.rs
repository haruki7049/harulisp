use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn test() -> anyhow::Result<()> {
    let _ = Command::cargo_bin("harulisp")?
        .arg("--script")
        .arg("( hoge hoge )")
        .assert()
        .success();

    let _ = Command::cargo_bin("harulisp")?
        .arg("--script")
        .arg("(def main (lambda '() (progn ( write_line \"FooBar...!!\" ) ( write_line \"This is a example text\"))))")
        .assert()
        .success();

    Ok(())
}
