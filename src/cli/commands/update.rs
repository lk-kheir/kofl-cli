use crate::backup::Backup;
use crate::cli::Command;
use crate::validator::core::{ValidationType, ValidationResult};
use crate::validator::registry::ValidationRegistry;
use std::fmt;
use crate::errors::{ErrorExecution, ErrorValidation};
use crate::context::Context;
use crate::db::Db::Entry;
use chrono::prelude::*;
use log::{debug, info, warn, error};
use sha2::Digest;


use aes::cipher::{
    KeyIvInit, StreamCipher,
    generic_array::GenericArray,
};
use ctr::Ctr32BE;
type Aes256Ctr = Ctr32BE<aes::Aes256>;
pub struct UpdateCmd {
    pub name: String,
    pub password: String,
}


impl  UpdateCmd {
    pub fn new(name: String, password: String) -> UpdateCmd
    {
        UpdateCmd{name, password}
    }
}

impl PartialEq for UpdateCmd {
    fn eq(&self, other: &Self) -> bool {
        if (self.name == other.name) && (self.password == other.password) {return true}
        false
    }
}

impl fmt::Debug for UpdateCmd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Add Command")
         .field("name", &self.name)
         .field("password", &self.password)
         .finish()
    }
}


impl Command for UpdateCmd {
    fn execute(&self, context: &Context) -> bool  {
        let master_key_hash = {
            let kgc = context.kgc.borrow();
            kgc.get_hashed_pwd()
        };

        let master_key_bytes = match hex::decode(&master_key_hash) {
            Ok(bytes) => bytes,
            Err(e) => {
                error!("Error decoding master key hash: {}", e);
                return false;
            }
        };

        // Create key and nonce
        let key = GenericArray::from_slice(&master_key_bytes);
        let nonce = GenericArray::from_slice(&[0u8; 16]); // In production, use secure random nonce

        // Initialize ciph
        let mut cipher = Aes256Ctr::new(key, nonce);

        // Encrypt the password
        let mut encrypted_password = self.password.clone().into_bytes();
        cipher.apply_keystream(&mut encrypted_password);

        // Convert to hex for storage
        let encrypted_password_hex = hex::encode(encrypted_password);

        let entry_id = match context.db.get_entry_by_name(&self.name) {
            Ok(entry) => entry.id,
            Err(e) => {
                error!("Error retrieving entry by name: {}", e); // should shoul never happen as this check happened in validate
                return false;
            }
        };

        // Create new entry with updated information
        let updated_entry = Entry {
            id: entry_id,
            ent_name: self.name.clone(),
            password_hash: encrypted_password_hex,
            timestamp: Utc::now().to_rfc3339(),
        };

        // Update the entry in the database
        match context.db.update_entry(entry_id, updated_entry) {
            Ok(_) => {
                info!("Entry updated successfully");
            },
            Err(e) => {
                error!("Error updating entry: {}", e);
                return false;
            }
        }

        let bc = Backup::new().unwrap();

        bc.create_new_backup(&context.kgc.borrow().get_config_path(), 
        &context.kgc.borrow().get_data_storage_path(), 
        &context.kgc.borrow().get_config_path().with_extension("checksum")).unwrap();

        true
    }

    fn validate(&self, context: &Context) -> bool  {
        
        let val_reg = ValidationRegistry::<UpdateCmd>::new();

        let val_checks = vec![
            ValidationType::MasterKeyCheck,
            ValidationType::SessionCheck,
            ValidationType::EntryExistsCheck,
            ValidationType::PasswordRequirementCheck,
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
        debug!("Update command with name = {}", self.name);
        ()
    }
}