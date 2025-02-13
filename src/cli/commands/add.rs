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
pub struct AddCmd {
    pub name: String,
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

        // Initialize cipher
        let mut cipher = Aes256Ctr::new(key, nonce);

        // Encrypt the password
        let mut encrypted_password = self.password.clone().into_bytes();
        cipher.apply_keystream(&mut encrypted_password);

        // Convert to hex for storage
        let encrypted_password_hex = hex::encode(encrypted_password);

        // Create new entry
        let new_entry = Entry {
            id: 0, // will be ignored by sqlite
            ent_name: self.name.clone(),
            password_hash: encrypted_password_hex,
            timestamp: Utc::now().to_rfc3339()
        };

        // Add the entry to the database if error return false

        match context.db.add_entry(new_entry) {
            Ok(_) => {
                info!("Entry added successfully");
            },
            Err(e) => {
                error!("Error adding entry: {}", e);
                return false;
            }
        }

        true
    }

    fn validate(&self, context: &Context) -> bool  {
        
        let val_reg = ValidationRegistry::<AddCmd>::new();

        let val_checks = vec![
            ValidationType::MasterKeyCheck,
            ValidationType::SessionCheck,
            ValidationType::EntryExistsCheck,
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
        debug!("Add command with name = {}", self.name);
        ()
    }
}