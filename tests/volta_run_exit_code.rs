//! Regression test that doesn't need the `mock-network` feature: `VOLTA_BYPASS` skips platform
//! resolution, so no tool has to be fetched.
#![cfg(unix)]

use std::process::Command;

fn volta_run(args: &[&str]) -> Option<i32> {
    Command::new(env!("CARGO_BIN_EXE_volta"))
        .arg("run")
        .args(args)
        .env("VOLTA_HOME", env!("CARGO_TARGET_TMPDIR"))
        .env("VOLTA_BYPASS", "1")
        .status()
        .expect("volta should start")
        .code()
}

#[test]
fn volta_run_propagates_tool_exit_code() {
    assert_eq!(volta_run(&["sh", "-c", "exit 7"]), Some(7));
    assert_eq!(volta_run(&["sh", "-c", "exit 0"]), Some(0));
}
