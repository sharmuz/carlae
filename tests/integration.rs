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
