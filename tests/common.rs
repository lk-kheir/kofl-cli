use assert_cmd::Command;
use log::{debug, error, info};
use std::{env, fs};
use super::utils::get_home_dir;
pub fn run_kofl_cmd(args: &[&str]) -> Command {
    let mut cmd = Command::cargo_bin("kofl").unwrap();
    cmd.args(args);
    cmd
}

pub fn setup_test_environment() {
    info!("Preparing the set up for test envirement");
    
    let home_dir = get_home_dir();
    let data_path = home_dir.join("kofl.sqlite");
    let config_path = home_dir.join(".kofl");
    let session_path = home_dir.join(".kofl_session");



    // fs::remove_file(data_path)?;
    // fs::remove_file(config_path)?;
    // fs::remove_file(session_path)?;

    // fs::remove_file()
}

pub fn teardown_test_environment() {
    info!("Cleaning the  set up for test envirement");
}