use std::collections::HashMap;

pub mod validator {
    use super::*;
    use crate::context::Context;

    #[derive(Hash, Eq, PartialEq, Clone)]
    pub enum ValidationType {
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

    pub trait Validator {
        fn validate(&self, context: &Context) -> ValidationResult;
    }
    
    struct MasterKeyValidator {}
    struct RateLimitValidator {}
    struct EntryExistsValidator {}
    struct DuplicateEntryValidator {}

    impl Validator for RateLimitValidator {
        fn validate(&self, context: &Context) -> ValidationResult {
            println!("Validator for RateLimitValidator ✅");
            ValidationResult::Success
        }
    }
    impl Validator for MasterKeyValidator {
        fn validate(&self, context: &Context) -> ValidationResult {
            println!("Validator for MasterKeyValidator ✅");
            ValidationResult::Success
        }
    }


    impl Validator for EntryExistsValidator {
        fn validate(&self, context: &Context) -> ValidationResult {
            println!("Validator for EntryExistsValidator ✅");
            ValidationResult::Success
        }
    }

    impl Validator for DuplicateEntryValidator {
        fn validate(&self, context: &Context) -> ValidationResult {
            println!("Validator for DuplicateEntryValidator ✅");
            ValidationResult::Success
        }
    }

    pub struct ValidationRegistry {
        pub validators: HashMap<ValidationType, Box<dyn Validator>>,
    }

    impl ValidationRegistry {
        pub fn new(cmdType: CommandType) -> Self {
            let mut validators = HashMap::new();
            
            match cmdType {
                CommandType::GET_CMD => {
                    validators.insert(ValidationType::MasterKeyCheck, Box::new(MasterKeyValidator {}) as Box<dyn Validator>);
                    validators.insert(ValidationType::RateLimitCheck, Box::new(RateLimitValidator {}) as Box<dyn Validator>);
                    validators.insert(ValidationType::EntryExistsCheck, Box::new(EntryExistsValidator {}) as Box<dyn Validator>);
                    validators.insert(ValidationType::DuplicateEntryCheck, Box::new(DuplicateEntryValidator {}) as Box<dyn Validator>);
                },
                CommandType::ADD_CMD => {
                    validators.insert(ValidationType::MasterKeyCheck, Box::new(MasterKeyValidator {}) as Box<dyn Validator>);
                    validators.insert(ValidationType::RateLimitCheck, Box::new(RateLimitValidator {}) as Box<dyn Validator>);
                    validators.insert(ValidationType::EntryExistsCheck, Box::new(EntryExistsValidator {}) as Box<dyn Validator>);
                    validators.insert(ValidationType::DuplicateEntryCheck, Box::new(DuplicateEntryValidator {}) as Box<dyn Validator>);
                },
                CommandType::INIT_CMD => {
                    validators.insert(ValidationType::MasterKeyCheck, Box::new(MasterKeyValidator {}) as Box<dyn Validator>);
                    validators.insert(ValidationType::RateLimitCheck, Box::new(RateLimitValidator {}) as Box<dyn Validator>);
                    validators.insert(ValidationType::EntryExistsCheck, Box::new(EntryExistsValidator {}) as Box<dyn Validator>);
                    validators.insert(ValidationType::DuplicateEntryCheck, Box::new(DuplicateEntryValidator {}) as Box<dyn Validator>);
                }
            }
            
            Self { validators }
        }

        // pub fn validate_all(&self, required_validations: Vec<ValidationType>, context: &Context) -> Vec<ValidationResult> {
        //     required_validations
        //         .iter()
        //         .filter_map(|val_type| {
        //             self.validators
        //                 .get(val_type)
        //                 .map(|validator| validator.validate(context))
        //         })
        //         .collect()
        // }

        // // Optional helper method to register new validators
        // pub fn register(&mut self, val_type: ValidationType, validator: Box<dyn Validator>) {
        //     self.validators.insert(val_type, validator);
        // }
    }
}