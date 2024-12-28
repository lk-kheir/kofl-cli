mod cli;
mod errors;
mod config;
mod utils;

use std::fs;
use std::env;
use std::path::PathBuf;
use std::process::exit;
use clap::{Parser, Subcommand};
use cli::cli::{AddCmd, InitCmd, Command};
use config::Config::KoflGlobalConfig;

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
fn main() {

    let mut kgc: KoflGlobalConfig = KoflGlobalConfig::new();
    kgc.load();
    println!("{:?}", kgc);

    return;
    // try to load the config file from home directory

    if let Some(home_dir) = env::home_dir() {
        // Construct the path to the configuration file
        let mut config_file_path = PathBuf::from(home_dir);
        config_file_path.push(".kofl");

        // Check if the configuration file exists
        if config_file_path.exists() {
            println!("Configuration file exists at: {:?}", config_file_path);
        } else {
            println!("Configuration file does not exist.");
            println!("run: <kofl init> to init .config file");
            // exit(1);
        }
    } else {
        println!("Could not determine home directory.");
    }

    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { name, password } => {
            let add_command = AddCmd::new(name.to_string(), password.to_string());

            match add_command.validate() {
                Ok(_) => {println!("validation done");}
                Err(_) => {println!("OOps somtheing went wroong during validation");}
            }
            match add_command.execute() {
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
            let _ = InitCommand.execute();
        }
    }
}