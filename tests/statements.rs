use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn example_statements() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("harulisp").unwrap();

    cmd.arg("--script").arg("( hoge hoge )").assert().success();

    Ok(())
}
