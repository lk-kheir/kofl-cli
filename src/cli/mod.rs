pub mod cli {
use std::fmt;
use crate::errors::{ErrorExecution, ErrorValidation};
pub trait Command {
    fn execute(&self) -> Result<(), ErrorExecution>;
    fn validate(&self) -> Result<(), ErrorValidation>;
    fn display(&self);
}


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
    fn execute(&self) -> Result<(), ErrorExecution>  {
        return Ok(())    
    }

    fn validate(&self) -> Result<(), ErrorValidation>  {
        return Ok(())
    }

    fn display(&self) {
        println!("Add command with name = {}, password = {}", self.name, self.password);
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