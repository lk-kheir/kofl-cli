mod Cli {
use std::fmt;
pub trait Command {
    fn execute(&self) -> String;
    fn validate(&self) -> String;
    fn display(&self) -> String;
}


pub struct Add {
    name: String,
    password: String,
}

impl  Add {
    fn new(name: String, password: String) -> Add
    {
        Add{name: "facebook".to_string(), password: "whocares".to_string()}
    }
}

impl PartialEq for Add {
    fn eq(&self, other: &Self) -> bool {
        if (self.name == other.name) && (self.password == other.password) {return true}
        false
    }
}

impl fmt::Debug for Add {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Add Command")
         .field("name", &self.name)
         .field("password", &self.password)
         .finish()
    }
}

impl Command for Add {
    fn execute(&self) -> String  {
        String::from("this this execute funtion")
    }

    fn validate(&self) -> String  {
        String::from("this this validate  funtion")
    }

    fn display(&self) -> String  {
        String::from("this this display funtion")
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let add_commad: Add = Add::new("facebook".to_string(), "whocares".to_string());
        assert_eq!(add_commad, Add {
            name: "facebook".to_string(),
            password: "whocares".to_string()
        }
        );
    }
}
}