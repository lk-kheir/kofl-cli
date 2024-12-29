mod cli;
mod errors;
mod config;
mod utils;
mod context;

use std::fs;
use std::env;
use std::path::PathBuf;
use std::process::exit;
use clap::{Parser, Subcommand};
use cli::cli::{AddCmd, InitCmd, Command};
use config::Config::KoflGlobalConfig;
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
        // #[arg(short, long)]
        name: String,
        
        // #[arg(short, long)]
        password: String,
    },
    Init {

    }
}
#[warn(unused_variables)]
#[warn(unused_imports)]
fn main() -> () {

    let context = Context::new();

    println!("kgc = {:?}", context.kgc);

    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { name, password } => {
            let add_command = AddCmd::new(name.to_string(), password.to_string());

            match add_command.validate(&context) {
                Ok(_) => {println!("validation done");}
                Err(_) => {println!("OOps somtheing went wroong during validation");}
            }
            match add_command.execute(&context) {
                Ok(_) => {
                    add_command.display();
                }
                Err(e) => {
                    eprintln!("Error executing command");
                }
            }
        },
        Commands::Init {  } => {
            let InitCommand = InitCmd::new();
            let _ = InitCommand.execute(&context);
        }
    }
}