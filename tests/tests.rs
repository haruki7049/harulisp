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

    let _ = Command::cargo_bin("harulisp")?
        .arg("--script")
        .arg("(def main (lambda '(hoge) (progn ( write_line hoge ) ( write_line \"This is a example text\"))))")
        .assert()
        .success();

    let _ = Command::cargo_bin("harulisp")?
        .arg("--script")
        .arg("(def main (lambda '(hoge) (progn ( write_line hoge ) ( write_line \"This is a example text\"))))")
        .assert()
        .success();

    let _ = Command::cargo_bin("harulisp")?
        .arg("--script")
        .arg(
            "(def main (lambda '() (echo_foobar)))
             (def echo_foobar (lambda '() (progn (write_line \"foobar\"))))",
        )
        .assert()
        .success();

    let _ = Command::cargo_bin("harulisp")?
        .arg("--script")
        .arg(
            "(def main (lambda '() (echo_foobar)))
             (def echo_foobar (lambda '() (progn (write_line \"foobar\"))))
            ",
        )
        .assert()
        .success();

    Ok(())
}
