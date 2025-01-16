mod cli;
mod errors;
mod config;
mod utils;
mod db;
mod context;
mod validator;

use clap::{Parser, Subcommand};
use cli::cli::{AddCmd, InitCmd, GetCmd, Command};
use context::Context;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        name: String,
        password: String,
    },
    Init { },
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

#[warn(unused_variables)]
#[warn(unused_imports)]
fn main() -> () {
    let context = Context::new().unwrap();
    println!("kgc = {:?}", context.kgc);

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
    }
}