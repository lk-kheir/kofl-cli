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

    impl Validator for MasterKeyValidator {
        fn validate(&self, context: &Context) -> ValidationResult {
            // Your implementation here
            ValidationResult::Success
        }
    }

    impl Validator for RateLimitValidator {
        fn validate(&self, context: &Context) -> ValidationResult {
            // Your implementation here
            ValidationResult::Success
        }
    }

    impl Validator for EntryExistsValidator {
        fn validate(&self, context: &Context) -> ValidationResult {
            // Your implementation here
            ValidationResult::Success
        }
    }

    impl Validator for DuplicateEntryValidator {
        fn validate(&self, context: &Context) -> ValidationResult {
            // Your implementation here
            ValidationResult::Success
        }
    }

    pub struct ValidationRegistry {
        validators: HashMap<ValidationType, Box<dyn Validator>>,
    }

    impl ValidationRegistry {
        pub fn new() -> Self {
            let mut validators = HashMap::new();
            validators.insert(ValidationType::MasterKeyCheck, Box::new(MasterKeyValidator {}));
            validators.insert(ValidationType::RateLimitCheck, Box::new(RateLimitValidator {}));
            validators.insert(ValidationType::EntryExistsCheck, Box::new(EntryExistsValidator {}));
            validators.insert(ValidationType::DuplicateEntryCheck, Box::new(DuplicateEntryValidator {}));
            
            Self { validators }
        }

        pub fn validate_all(&self, required_validations: Vec<ValidationType>, context: &Context) -> Vec<ValidationResult> {
            required_validations
                .iter()
                .filter_map(|val_type| {
                    self.validators
                        .get(val_type)
                        .map(|validator| validator.validate(context))
                })
                .collect()
        }

        // Optional helper method to register new validators
        pub fn register(&mut self, val_type: ValidationType, validator: Box<dyn Validator>) {
            self.validators.insert(val_type, validator);
        }
    }
}