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
use log::{info, warn, error};
use env_logger::Env;
use colored::*;
use std::io::Write;


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
                _ => {}
            }
        }
    }
}

fn main() {
    // Initialize the logger
    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .format(|buf, record| {
            let level = match record.level() {
                log::Level::Error => "ERROR".red(),
                log::Level::Warn => "WARN".yellow(),
                log::Level::Info => "INFO".green(),
                log::Level::Debug => "DEBUG".blue(),
                log::Level::Trace => "TRACE".purple(),
            };
            writeln!(
                buf,
                "[{}] - {}:{}:{} - {}",
                level,
                record.module_path().unwrap_or("unknown_module"),
                record.file().unwrap_or("unknown_file"),
                record.line().unwrap_or(0),
                record.args()
            )
        })
        .init();


    let context = Context::new().unwrap();
    info!("{:?}", context.kgc);
    info!("{:?}", context.ss);

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