#![allow(warnings)]


mod cli;
mod errors;
mod config;
mod utils;
mod db;
mod context;
mod validator;
mod session;

// Updated imports for the commands
use clap::{Parser, Subcommand};
// Import commands from the new location
use cli::commands::{AddCmd, GetCmd, InitCmd, LogInCmd};  // Updated path
use cli::Command;  // Import the Command trait from cli module
use context::Context;

use crate::session::session::Session;



#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init { },
    Login {},
    Add {
        name: String,
        password: String,
    },
    Get {
        ent_name: String
    }
}

fn execute_command<T: Command>(cmd: &T, context: &Context) {
    match cmd.validate(context) {
        Ok(_) => {
            match cmd.execute(context) {
                Ok(_) => {
                    cmd.display();
                }
                Err(exec_err) => {
                    eprintln!("Error during execution: {}", exec_err);
                }
            }
        }
        Err(val_err) => {
            match val_err {
                errors::ErrorValidation::UnprovidedMasterKey => {
                    eprintln!("Please provide a master key");
                }
                _ => eprintln!("Validation error: {}", val_err)
            }
        }
    }
}


fn main() -> () {
    let context = Context::new().unwrap();
    println!("{:?}", context.kgc);
    println!("{:?}", context.ss);
    let cli = Cli::parse();

    match &cli.command { 
        Commands::Init {  } => {
            let init_command = InitCmd::new();
            execute_command(&init_command, &context);
        },
        Commands::Add { name, password } => {
            let add_command = AddCmd::new(name.to_string(), password.to_string());
            execute_command(&add_command, &context);
        },
        Commands::Get { ent_name  } => {
            let get_command = GetCmd::new(ent_name.to_string());
            execute_command(&get_command, &context);
        }
        Commands::Login {} => {
            let login_command = LogInCmd::new();
            execute_command(&login_command, &context);

        } 
    }
}