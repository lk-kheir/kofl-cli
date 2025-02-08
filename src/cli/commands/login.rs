use crate::cli::Command;
use crate::errors::{ErrorExecution, ErrorValidation};
use crate::context::Context;
use crate::session::Session;
use log::info;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use sha2::{Sha256, Digest};


use aes::cipher::KeyIvInit;
use ctr::Ctr32BE;
type Aes256Ctr = Ctr32BE<aes::Aes256>;


pub struct LogInCmd {
    // for now is emty 
}

impl LogInCmd {
    pub fn new() -> Self {
        LogInCmd{}
    }   

}

impl Command for LogInCmd {


    fn execute(&self, context: &Context) -> Result<(), ErrorExecution> {
        let master_pwd_input = rpassword::prompt_password("Enter the master password ===> ").unwrap();

        let salt = context.kgc.borrow().get_salt();
        let stored_hash = context.kgc.borrow().get_hashed_pwd();
    
        let mut hasher = Sha256::new();
        hasher.update(master_pwd_input.as_bytes());
        hasher.update(salt.as_bytes());
        let computed_hash = hasher.finalize();
        let computed_hash_hex = hex::encode(computed_hash);
    
        // println!("Computed hash: {}", computed_hash_hex);
        // println!("Stored hash:   {}", stored_hash);
    
        // 4. Compare the computed hash with the stored hash.
        if computed_hash_hex != stored_hash {
            return Err(ErrorExecution::AuthenticationFailed);
        }
    
        

        let user_login = context.kgc.borrow().get_user_login().clone();
        let new_session = Session::new(user_login);
        
        new_session.write_session_config_to_toml_file();
    
    
        println!("Login successful! New session created.");
    
        Ok(())
    }
    

    fn validate(&self, context: &Context) -> Result<(), ErrorValidation>  {
        Ok(())
    }

    fn display(&self) {
        println!("Login Command");
        ()
    }
}

