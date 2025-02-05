use std::collections::HashMap;

pub mod validator {
    use log::{info, error};

    use super::*;
    use crate::{context::Context, cli::commands::{GetCmd, AddCmd, InitCmd}};

    #[derive(Hash, Eq, PartialEq, Clone)]
    pub enum ValidationType {
        SessionCheck,
        MasterKeyCheck,
        RateLimitCheck,
        EntryExistsCheck,
        DuplicateEntryCheck,
        // more to come later
    }

    pub enum CommandType {
        GET_CMD,
        ADD_CMD,
        INIT_CMD
    }

    #[derive(Debug)]
    pub enum ValidationResult {
        Success,
        Failure(String),
        Warning(String),
    }
    pub trait Validator<T> {
        fn validate(&self, _context: &Context, cmd: &T) -> ValidationResult;
    }
    
    struct MasterKeyValidator {}
    struct SessionValidator {}
    struct RateLimitValidator {}
    struct EntryExistsValidator {}
    struct DuplicateEntryValidator {}
    

    impl<> Validator<GetCmd> for SessionValidator  {
        fn validate(&self, context: &Context, cmd: &GetCmd) -> ValidationResult {
            info!("Validator for SessionValidator ❓");

            if !context.ss.check_if_expired() {
                ValidationResult::Success

            }else {
                ValidationResult::Failure("Session expired ⛔".to_string())
            }
        }
    }


    impl<T> Validator<T> for RateLimitValidator {
        fn validate(&self, context: &Context, cmd: &T) -> ValidationResult {
            info!("Validator for RateLimit ❓");
            ValidationResult::Success
        }
    }
    
    impl<T> Validator<T> for MasterKeyValidator {
        fn validate(&self, context: &Context, cmd: &T) -> ValidationResult {
            info!("Validator for MasterKey ❓");
            if context.kgc.borrow().is_master_key_provided() {
                // info!("Master key provided");
                ValidationResult::Success
            } else {
                // error!("Master key Not provided");
                ValidationResult::Failure("Master key not provided ⛔".to_string())
            }
        }
    }
    
    impl<> Validator<GetCmd> for EntryExistsValidator {
        fn validate(&self, context: &Context, cmd: &GetCmd) -> ValidationResult {
            info!("Validator for EntryExistsValidator ❓");
            match context.db.entry_exist(cmd.ent_name.clone())
            {
                Ok(res) => if res {
                    ValidationResult::Success
                } else {
                    ValidationResult::Failure("No entry found with similar name ⛔".to_string())
                }
                Err(err) => {
                    ValidationResult::Failure("Error during in db ⛔".to_string())

                }
            }
        }
    }

    impl<> Validator<AddCmd> for EntryExistsValidator {
        fn validate(&self, context: &Context, cmd: &AddCmd) -> ValidationResult {
            info!("Validator for EntryExistsValidator ✅");

            match context.db.entry_exist(cmd.name.clone())
            {
                Ok(res) => if res {
                    ValidationResult::Failure("Master key not provided".to_string())
                } else {
                     ValidationResult::Success
                }
                Err(err) => {
                    ValidationResult::Failure("Master key not provided".to_string())

                }
            }
        }
    }
    
    impl<T> Validator<T> for DuplicateEntryValidator {
        fn validate(&self, context: &Context, cmd: &T) -> ValidationResult {
            info!("Validator for DuplicateEntryValidator");
            ValidationResult::Success
        }
    }
    
    pub struct ValidationRegistry<T> {
        pub validators: HashMap<ValidationType, Box<dyn Validator<T>>>,
    }
    
    impl ValidationRegistry<GetCmd> {
        pub fn new() -> Self {
            let mut validators = HashMap::new();
            validators.insert(ValidationType::MasterKeyCheck, Box::new(MasterKeyValidator {}) as Box<dyn Validator<GetCmd>>);
            validators.insert(ValidationType::SessionCheck, Box::new(SessionValidator {}) as Box<dyn Validator<GetCmd>>);
            validators.insert(ValidationType::RateLimitCheck, Box::new(RateLimitValidator {}) as Box<dyn Validator<GetCmd>>);
            validators.insert(ValidationType::EntryExistsCheck, Box::new(EntryExistsValidator {}) as Box<dyn Validator<GetCmd>>);
            validators.insert(ValidationType::DuplicateEntryCheck, Box::new(DuplicateEntryValidator {}) as Box<dyn Validator<GetCmd>>);
            Self { validators }
        }
    }
    
    impl ValidationRegistry<AddCmd> {
        pub fn new() -> Self {
            let mut validators = HashMap::new();
            validators.insert(ValidationType::MasterKeyCheck, Box::new(MasterKeyValidator {}) as Box<dyn Validator<AddCmd>>);
            validators.insert(ValidationType::RateLimitCheck, Box::new(RateLimitValidator {}) as Box<dyn Validator<AddCmd>>);
            validators.insert(ValidationType::EntryExistsCheck, Box::new(EntryExistsValidator {}) as Box<dyn Validator<AddCmd>>);
            validators.insert(ValidationType::DuplicateEntryCheck, Box::new(DuplicateEntryValidator {}) as Box<dyn Validator<AddCmd>>);
            Self { validators }
        }
    }
    
    impl ValidationRegistry<InitCmd> {
        pub fn new() -> Self {
            let mut validators = HashMap::new();
            Self { validators }
        }
    }
    
}