use assert_cmd::cargo;
use assert_cmd::prelude::*;
use predicates::str::contains;
use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

// Note to self: The env!() there is obviously the environment set by
// `cargo test`, and not anything weird coming out of stdout.

fn runexec() -> Command {
    Command::new(cargo::cargo_bin!("boggle-solve"))
}

#[test]
fn cli_version() {
    runexec()
        .args(&["-V"])
        .assert()
        .stdout(contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn cli_no_args() {
    runexec().assert().failure();
}

#[test]
fn cli_small_board_custom_dict() {
    let mut board = NamedTempFile::new().unwrap();
    let mut dict = NamedTempFile::new().unwrap();
    write!(board, "an\nut\n").unwrap();
    write!(dict, "art\nant\nnut\naunt\ntan\n").unwrap();
    let args = [
        board.path().to_str().unwrap(),
        "--dict",
        dict.path().to_str().unwrap(),
    ];
    runexec().args(&args).assert().stdout(contains("aunt"));
}
