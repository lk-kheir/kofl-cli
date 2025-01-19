use crate::cli::Command;
use crate::errors::{ErrorExecution, ErrorValidation};
use crate::context::Context;
use log::{error, info, warn};
use sha2::Digest;
use crate::validator::validator::{ValidationRegistry, ValidationResult};
use crate::validator::validator::ValidationType;
use crate::validator::validator::CommandType;


use aes::cipher::{
    KeyIvInit, StreamCipher,
    generic_array::GenericArray,
};
use ctr::Ctr32BE;
type Aes256Ctr = Ctr32BE<aes::Aes256>;


pub struct GetCmd {
    ent_name: String
}


impl GetCmd {
    pub fn new(ent_name: String) -> Self {
        GetCmd{ent_name}
    }
}


impl Command for GetCmd {

    fn execute(&self, context: &Context) -> Result<(), ErrorExecution> {
        let entry = context.db.get_entry_by_name(&self.ent_name)
            .map_err(|e| match e {
                rusqlite::Error::QueryReturnedNoRows => ErrorExecution::NoMatchingEntry,
                _ => ErrorExecution::Unknown,
            })?;

        // Get master key hash
        let master_key_hash = {
            let kgc = context.kgc.borrow();
            kgc.get_master_key_hash()
        };

        // Decode the master key
        let master_key_bytes = hex::decode(&master_key_hash)
            .map_err(|_| ErrorExecution::DecryptionError)?;

        // Create key and nonce
        let key = GenericArray::from_slice(&master_key_bytes);
        let nonce = GenericArray::from_slice(&[0u8; 16]); // Must match the nonce used in AddCmd

        // Initialize cipher
        let mut cipher = Aes256Ctr::new(key, nonce);

        // Decode the stored encrypted password
        let mut encrypted_password = hex::decode(&entry.password_hash)
            .map_err(|_| ErrorExecution::DecryptionError)?;

        // Decrypt
        cipher.apply_keystream(&mut encrypted_password);

        // Convert to string
        let decrypted_password = String::from_utf8(encrypted_password)
            .map_err(|_| ErrorExecution::DecryptionError)?;

        println!("Entry Name: {}", entry.ent_name);
        println!("Password: {}", decrypted_password);

        Ok(())
    }   

    fn validate(&self, context: &Context) -> Result<(), ErrorValidation>  {

        let val_reg = ValidationRegistry::new(CommandType::GET_CMD);

        let val_checks = vec![
            ValidationType::MasterKeyCheck,
            ValidationType::RateLimitCheck,
            ValidationType::EntryExistsCheck,
            ValidationType::DuplicateEntryCheck
        ];


        for a_check in val_checks {

            match val_reg.validators.get(&a_check).unwrap().validate(context) {
                ValidationResult::Failure(msg) => {
                    error!("{msg}");
                    return Err(ErrorValidation::Temp);
                },
                ValidationResult::Warning(msg) => warn!("{msg}"),
                _ => info!("test passed")

            }
        }
        
        return Ok(())
    }

    fn display(&self) {
        println!("Get command with entry name = {}", self.ent_name);
        ()
    }
}
