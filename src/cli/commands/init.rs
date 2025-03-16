use crate::cli::Command;
use crate::errors::{ErrorExecution, ErrorValidation};
use crate::context::Context;
use crate::session::Session;
use crate::validator::core::{ValidationResult, ValidationType};
use crate::validator::registry::ValidationRegistry;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use sha2::{Sha256, Digest};
use log::{debug, error, info, warn};


use aes::cipher::KeyIvInit;
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
        fn execute(&self, context: &Context) -> bool  {

            // let's create a function that will promot the user to set crutial settings    


            let master_pwd  = rpassword::prompt_password("type a master password ==> ").unwrap();
            let master_pwd_confirmed = rpassword::prompt_password("type the master password again ==> ").unwrap();

            if master_pwd != master_pwd_confirmed {
                error!("Password mismatch");
                return false;
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

            let user_login = context.kgc.borrow().get_user_login().clone();
            let new_session = Session::new(user_login, true);
        
            new_session.write_session_config_to_toml_file();

            info!("Master password set successfully!");
            info!("Kofl is now ready to use.");

            info!("Default settings have been applied:");
            info!("- Session duration: 30 minutes");
            info!("- Clipboard timeout: 10 seconds");
            info!("");
            info!("{}", "To customize these settings, run:");
            info!("  kofl settings list   # view all settings");
            info!("  kofl settings set    # change a setting");
            true
        }

    fn validate(&self, context: &Context) -> bool  {

        let val_reg = ValidationRegistry::<InitCmd>::new();

        let val_checks = vec![
            ValidationType::MasterKeyCheck, // we check if the master key check does not exist
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
        debug!("Init Command");
        ()
    }
}

