pub mod cli {
use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::{Error as IOError, Write};
use std::path::PathBuf;
use std::env;
use crate::errors::{ErrorExecution, ErrorValidation};
use crate::config::Config::KoflGlobalConfig;
use crate::context::Context;

#[warn(unused_variables)]
#[warn(unused_imports)]
pub trait Command {
    fn validate(&self, context: &Context) -> Result<(), ErrorValidation>;
    fn execute(&self, context: &Context) -> Result<(), ErrorExecution>;
    fn display(&self);
}

// maybe ent_name and ent_pass is much better

pub struct AddCmd {
    name: String,
    password: String,
}


impl  AddCmd {
    pub fn new(name: String, password: String) -> AddCmd
    {
        AddCmd{name, password}
    }
}

impl PartialEq for AddCmd {
    fn eq(&self, other: &Self) -> bool {
        if (self.name == other.name) && (self.password == other.password) {return true}
        false
    }
}

impl fmt::Debug for AddCmd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Add Command")
         .field("name", &self.name)
         .field("password", &self.password)
         .finish()
    }
}

impl Command for AddCmd {
    fn execute(&self, context: &Context) -> Result<(), ErrorExecution>  {
        let db_file_path = context.kgc.get_data_storage_path();
        // Open the file in write mode (creates a new file or truncates an existing file)
        let mut file = match OpenOptions::new()
            .append(true)
            .create(true)
            .write(true)
            .open(db_file_path) 
        {
            Ok(file) => file,
            Err(e) => return Err(ErrorExecution::IoError(e)),
        };

        // Write the name and password to the file
        match writeln!(file, "name: {} password: {}", self.name, self.password) {
            Ok(_) => Ok(()),
            Err(e) => Err(ErrorExecution::IoError(e)),
        }    
    }

    fn validate(&self, context: &Context) -> Result<(), ErrorValidation>  {
        return Ok(())
    }

    fn display(&self) {
        println!("Add command with name = {}, password = {}", self.name, self.password);
        ()
    }
}


pub struct InitCmd {
    // for now is emty 
}

impl InitCmd {
    pub fn new() -> Self {
        InitCmd{}
    }

}


impl Command for InitCmd {
    fn execute(&self, context: &Context) -> Result<(), ErrorExecution>  {
        
        Ok(())
    }

    fn validate(&self, context: &Context) -> Result<(), ErrorValidation>  {

        // if init command is used when already someconfig exists throw an erro
        return Ok(())
    }

    fn display(&self) {
        println!("Init Command");
        ()
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let add_commad: AddCmd = AddCmd::new("facebook".to_string(), "whocares".to_string());
        assert_eq!(add_commad, AddCmd {
            name: "facebook".to_string(),
            password: "whocares".to_string()
        }
        );
    }
}
}