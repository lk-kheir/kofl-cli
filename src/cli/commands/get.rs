use crate::cli::Command;
use crate::errors::{ErrorExecution, ErrorValidation};
use crate::context::Context;
use log::{debug, error, info, warn};
use sha2::Digest;
use crate::validator::core::{CommandType, ValidationResult, ValidationType};
use crate::validator::registry::ValidationRegistry;
use arboard::{Clipboard};
use arboard::Error as ClipboardError;



use aes::cipher::{
    KeyIvInit, StreamCipher,
    generic_array::GenericArray,
};
use ctr::Ctr32BE;
type Aes256Ctr = Ctr32BE<aes::Aes256>;


pub struct GetCmd {
    pub ent_name: String
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
            kgc.get_hashed_pwd()
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

        // println!("Entry Name: {}", entry.ent_name);
        // println!("Password: {}", decrypted_password);

        let mut clipboard = Clipboard::new().unwrap();
       

        // let the_string = "Hello, world!";
        match clipboard.set_text(decrypted_password) {
            Ok(_) => info!("Password is copied to clipboard"),
            Err(ClipboardError::ClipboardNotSupported) => error!("Your system does not support clibboard."),
            Err(ClipboardError::ContentNotAvailable) => error!("Content not available"),
            Err(ClipboardError::ClipboardOccupied) => error!("Clipboard is occupied"),
            Err(ClipboardError::ConversionFailure) => error!("Conversion failure should not happen as we retrieve utf-8 strings not images"),
            Err(ClipboardError::Unknown{description}) => error!("{}", description),
            Err(_) => error!("undefined behaviour"),
        }
        
        // println!("Clipboard text was: {}", clipboard.get_text().unwrap());
        Ok(())
    }   

    fn validate(&self, context: &Context) -> Result<(), ErrorValidation>  {

        let val_reg = ValidationRegistry::<GetCmd>::new();

        let val_checks = vec![
            ValidationType::MasterKeyCheck,
            ValidationType::SessionCheck,
            ValidationType::EntryExistsCheck,
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
        info!("Get command with entry name = {}", self.ent_name);
        ()
    }
}
