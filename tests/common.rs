use assert_cmd::Command;


pub fn run_kofl_cmd(args: &[&str]) -> Command {
    let mut cmd = Command::cargo_bin("kofl").unwrap();
    cmd.args(args);
    cmd
}

pub fn setup_test_environment() {
}

pub fn teardown_test_environment() {
}