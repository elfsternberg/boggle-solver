use std::process::Command;
use assert_cmd::prelude::*;
use predicates::str::contains;
use tempfile::NamedTempFile;
use std::io::Write;

// Note to self: The env!() there is obviously the environment set by
// `cargo test`, and not anything weird coming out of stdout.

#[test]
fn cli_version() {
    Command::cargo_bin("boggle-solve")
        .unwrap()
        .args(&["-V"])
        .assert()
        .stdout(contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn cli_no_args() {
    Command::cargo_bin("boggle-solve").unwrap().assert().failure();
}

#[test]
fn cli_small_board() {
    let mut board = NamedTempFile::new().unwrap();
    write!(board, "an\ntd\n").unwrap();
    Command::cargo_bin("boggle-solve")
        .unwrap()
        .args(&[board.path()])
        .assert()
        .stdout(contains("tan"));
}
