// tests/init_cmd_test.rs
use assert_cmd::Command;
use predicates::str::contains;
use rstest::rstest;

// Import shared functions from common.rs
mod common;
use common::{run_kofl_cmd, setup_test_environment, teardown_test_environment};

#[rstest]
fn test_get_command() {
    setup_test_environment();
    // run destory as if it's the first thing a user will run
    let mut cmd = run_kofl_cmd(&["destroy"]);
    cmd.assert()
        .success()
        .stdout(contains("Destroy Command"));
    teardown_test_environment();
}
