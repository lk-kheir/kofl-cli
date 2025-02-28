#![allow(warnings)]

mod cli;
mod config;
mod backup;
mod context;
mod db;
mod errors;
mod session;
mod utils;
mod validator;
mod constants;


// Updated imports for the commands
use clap::{Parser, Subcommand};
// Import commands from the new location
use cli::commands::{AddCmd, DestroyCmd, GetCmd, InitCmd, LogInCmd, UpdateCmd}; // Updated path
use cli::Command; // Import the Command trait from cli module
use colored::*;
use context::Context;
use env_logger::{Env, Target};
use log::{debug, error, info, warn};
use std::f32::consts::E;
use std::io::Write;
use std::process;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init {},
    Login {},
    Destroy {},
    Add { 
        name: String,
        #[arg(short, long)]
        suggest: bool
    },
    Get { ent_name: String },
    Update {
        ent_name: String,
        #[arg(short, long)]
        suggest: bool
    }
}


fn init_logger() {
    #[cfg(feature = "prod")]
    {
        env_logger::Builder::from_env(Env::default().default_filter_or("info"))
            .format(|buf, record| {
                writeln!(
                    buf,
                    "[{}] - {}",
                    record.level(),
                    record.args()
                )
            })
            .target(Target::Stdout)
            .init();
    }

    #[cfg(feature = "dev")]
    {
        env_logger::Builder::from_env(Env::default().default_filter_or("debug"))
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
            .target(Target::Stdout)
            .init();
    }
}

fn execute_command<T: Command>(cmd: &T, context: &Context) {
    if cmd.validate(context) {
        if cmd.execute(context) {
            cmd.display();
        } else {
            error!("Error during execution");
        }
    } else {
        error!("Error during validation");
    }
}

fn main() {
    // Initialize the logger
    init_logger();

    let context = Context::new().unwrap_or_else(|err| {
        error!("Program terminated due to setup issues: {}", err);
        process::exit(1);
    });

    debug!("{:?}", context.kgc);
    debug!("{:?}", context.ss);

    let cli = Cli::parse();

    match &cli.command {
        Commands::Init {} => {
            let init_command = InitCmd::new();
            execute_command(&init_command, &context);
        }
        Commands::Add { name, suggest   } => {
            info!("add commend with name {} and suggest flag is set to {}", name , suggest);
            if (*suggest) {
                let add_command = AddCmd::new(name.to_string(), String::from(""), *suggest);
                execute_command(&add_command, &context);
            }else {
                let pwd = rpassword::prompt_password("Enter the password for the entry ===> ").unwrap();
                let add_command = AddCmd::new(name.to_string(), pwd, *suggest);
                execute_command(&add_command, &context);
            }
        }
        Commands::Update { ent_name, suggest} => {
            let pwd = rpassword::prompt_password("Update the password for the entry ===> ").unwrap();
            let update_command = UpdateCmd::new(ent_name.to_string(), pwd);
            execute_command(&update_command, &context);
        }
        Commands::Get { ent_name } => {
            let get_command = GetCmd::new(ent_name.to_string());
            execute_command(&get_command, &context);
        }
        Commands::Login {} => {
            let login_command = LogInCmd::new();
            execute_command(&login_command, &context);
        }
        Commands::Destroy {  } => {
            let destroy_command = DestroyCmd::new();
            execute_command(&destroy_command, &context);
        }
    }
}
