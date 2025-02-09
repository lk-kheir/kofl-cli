use crate::cli::Command;
use crate::errors::{ErrorExecution, ErrorValidation};
use crate::context::Context;
use crate::validator::core::{ValidationResult, ValidationType};
use crate::validator::registry::ValidationRegistry;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use rusqlite::config;
use sha2::{Sha256, Digest};
use log::{error, info, warn};
use std::fs;


use aes::cipher::KeyIvInit;
use ctr::Ctr32BE;
type Aes256Ctr = Ctr32BE<aes::Aes256>;


pub struct DestroyCmd {
    // for now is emty 
}

impl DestroyCmd {
    pub fn new() -> Self {
        DestroyCmd{}
    }

}

impl Command for DestroyCmd {
    fn execute(&self, context: &Context) -> Result<(), ErrorExecution>  {
        warn!(
        "Note this is will delete all your data!!, Backup if needed
        ");
        
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

        // retrieve all config path and session path

        let binding = context.kgc.borrow();
        let data_path = binding.get_data_storage_path();
        let config_path = binding.get_config_path();
        let session_path = context.ss.get_session_path();
        
        // println!("{}", data_path.display());
        // println!("{}", config_path.display());
        // println!("{}", session_path.display());

        // remove the db;
        fs::remove_file(data_path)?;
        fs::remove_file(config_path)?;
        fs::remove_file(session_path)?;

        info!("\nAll data related to Kofl configuration have been deleted.\nRun init to start again");

        Ok(())
    }

    fn validate(&self, context: &Context) -> Result<(), ErrorValidation>  {
        
        let val_reg = ValidationRegistry::<DestroyCmd>::new();

        let val_checks = vec![
            // ValidationType::MasterKeyCheck,
            ValidationType::SessionCheck,
        ];


        for a_check in val_checks {

            match val_reg.validators.get(&a_check).unwrap().validate(context, &self) {
                ValidationResult::Failure(msg) => {
                    error!("{msg}");
                    return Err(ErrorValidation::Temp);
                },
                ValidationResult::Warning(msg) => warn!("{msg}"),
                _ => info!("test passed ✅")

            }
        }
        
        return Ok(())
    }

    fn display(&self) {
        info!("Destroy Command");
        ()
    }
}

