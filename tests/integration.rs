use std::path::Path;

use assert_cmd::{Command, pkg_name};
use predicates::prelude::*;

#[test]
fn carlae_runs_fibonacci_func() {
    let script = Path::new("tests/data/fib.carlae");
    let expected = String::from("0\n1\n1\n2\n3\n5\n8\n13\n21\n34\n");

    let mut cmd = Command::cargo_bin(pkg_name!()).expect("Crate binary to load");
    let assert = cmd.arg(script).assert();

    assert
        .success()
        .append_context("main", "Failed to execute script")
        .stdout(predicate::eq(expected));
}

#[test]
fn carlae_handles_multiple_syntax_errors() {
    let script = Path::new("tests/data/sync.carlae");
    let expected = ["Line 7", "Line 9", "Line 16", "Line 20"];

    let mut cmd = Command::cargo_bin(pkg_name!()).expect("Crate binary to load");
    let assert = cmd.arg(script).assert();

    assert
        .failure()
        .append_context("main", "Failed to handle syntax errors")
        .stderr(predicate::function(|err: &str| {
            expected.iter().all(|&text| err.contains(text))
        }))
        .stderr(predicate::str::contains("syntax errors"));
}

#[test]
fn carlae_returns_out_of_nested_if() {
    let script = Path::new("tests/data/return.carlae");
    let expected = String::from("88\n");

    let mut cmd = Command::cargo_bin(pkg_name!()).expect("Crate binary to load");
    let assert = cmd.arg(script).assert();

    assert
        .success()
        .append_context("main", "Failed to execute script")
        .stdout(predicate::eq(expected));
}

#[test]
fn carlae_and_or_skip_failing_right_operand() {
    let script = Path::new("tests/data/boolean.carlae");
    let expected = String::from("False\nTrue\n");

    let mut cmd = Command::cargo_bin(pkg_name!()).expect("Crate binary to load");
    let assert = cmd.arg(script).assert();

    assert
        .success()
        .append_context("main", "Failed to execute script")
        .stdout(predicate::eq(expected));
}
