use crate::cli::Command;
use crate::errors::{ErrorExecution, ErrorValidation};
use crate::context::Context;
use crate::validator::core::{ValidationResult, ValidationType};
use crate::validator::registry::ValidationRegistry;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use rusqlite::config;
use sha2::{Sha256, Digest};
use log::{debug, error, info, warn};
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
    fn execute(&self, context: &Context) -> bool  {
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
            error!("Invalid password");
            return false;
        }

        // retrieve all config path and session path

        let binding = context.kgc.borrow();
        let data_path = binding.get_data_storage_path();
        let config_path = binding.get_config_path();
        let session_path = context.ss.get_session_path();
        let cheksum_path = config_path.with_extension("checksum");
        
        // debug!("{}", data_path.display());
        // debug!("{}", config_path.display());
        // debug!("{}", session_path.display());
        // debug!("{}", cheksum_path.display());

        match fs::remove_file(&data_path) {
            Ok(_) => (),
            Err(err) => {
                error!("Failed to remove data storage file: {}", err);
                return false;
            }
        }
        match fs::remove_file(&config_path) {
            Ok(_) => (),
            Err(err) => {
                error!("Failed to remove kofl configuration file: {}", err);
                return false;
            }
        }

        match fs::remove_file(&session_path) {
            Ok(_) => (),
            Err(err) => {
                error!("Failed to remove kofl session file: {}", err);
                return false;
            }
        }

        match fs::remove_file(cheksum_path) {
            Ok(_) => (),
            Err(err) => {
                error!("Failed to remove checksum file: {}", err);
                return false;
            }
        }


        // later to be decided if we want to remove backup or not

        info!("\nAll data related to Kofl configuration have been deleted.\nRun init to start again");

        true
    }

    fn validate(&self, context: &Context) -> bool  {
        
        let val_reg = ValidationRegistry::<DestroyCmd>::new();

        let val_checks = vec![
            ValidationType::MasterKeyCheck,
            ValidationType::SessionCheck,
        ];


        for a_check in val_checks {

            match val_reg.validators.get(&a_check).unwrap().validate(context, &self) {
                ValidationResult::Failure(msg) => {
                    error!("{msg}");
                    return false;
                },
                ValidationResult::Warning(msg) => warn!("{msg}"),
                ValidationResult::Success => debug!("test passed ✅")

            }
        }
        
        true
    }

    fn display(&self) {
        debug!("Destroy Command");
        ()
    }
}

