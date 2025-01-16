use crate::cli::Command;
use std::fmt;
use crate::errors::{ErrorExecution, ErrorValidation};
use crate::context::Context;
use crate::db::Db::Entry;
use chrono::prelude::*;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use sha2::{Sha256, Digest};
use crate::validator::validator::ValidationRegistry;
use crate::validator::validator::ValidationType;
use crate::validator::validator::ValidationResult;


use aes::cipher::{
    KeyIvInit, StreamCipher,
    generic_array::GenericArray,
};
use ctr::Ctr32BE;
type Aes256Ctr = Ctr32BE<aes::Aes256>;


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
            

            let master_pwd  = rpassword::prompt_password("type a master password ==> ").unwrap();
            let master_pwd_confirmed = rpassword::prompt_password("type the master password again ==> ").unwrap();

            if master_pwd != master_pwd_confirmed {
                return Err(ErrorExecution::PasswordMismatch);
            }

            let salt:String = thread_rng()
                .sample_iter(&Alphanumeric)
                .take(16)
                .map(char::from)
                .collect();

            //hash the master password with the salt
            let mut hasher = Sha256::new();
            hasher.update(master_pwd.clone());
            hasher.update(salt.clone());
            let hashed_password = hasher.finalize();
            let hashed_password_hex = hex::encode(hashed_password); // Convert to hexadecimal string

            // println!("salt = {salt}");
            // println!("master pwd = {master_pwd}");
            // println!("oooooooooooooo hashed_password = {hashed_password_hex}");
            
            // Update the configuration with the salt and hashed password
            {
                let mut kgc = context.kgc.borrow_mut();
                kgc.set_salt(salt.clone());
                kgc.set_master_key_hash(hashed_password_hex); // Assuming you have a method to set the hashed password
                kgc.set_master_key_provided(true); // Assuming you have a method to set this flag
            }

            context.kgc.borrow().update();
            // Print the updated configuration
            // println!("Updated kgc = {:?}", context.kgc.borrow());

            Ok(())
        }

    fn validate(&self, context: &Context) -> Result<(), ErrorValidation>  {
        if context.kgc.borrow().is_master_key_provided() {
            return Err(ErrorValidation::AlreadyProvidedMasterKey);
            println!("Master key is provided");
        }
        else {
            println!("Master key is not provided");
        }
        return Ok(())
    }

    fn display(&self) {
        println!("Init Command");
        ()
    }
}

