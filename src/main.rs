mod cli;
mod errors;


use clap::{Parser, Subcommand};
use cli::cli::{AddCmd, Command};


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
}

fn main() {
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
        }
    }
}