mod cli;
mod errors;

use cli::Cli::{Add, Command};
fn main() {
    let add_command = Add::new("google".to_string(), "gggg".to_string());

    add_command.validate();
    add_command.execute();
    add_command.display();
}
