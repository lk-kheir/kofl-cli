// src/validator/registry.rs

use std::collections::HashMap;
use crate::validator::core::{ValidationType, Validator};
use crate::cli::commands::{GetCmd, AddCmd, InitCmd};

use crate::validator::master_key::MasterKeyValidator;
use crate::validator::session::SessionValidator;
use crate::validator::rate_limit::RateLimitValidator;
use crate::validator::entry_exists::EntryExistsValidator;
use crate::validator::duplicate::DuplicateEntryValidator;

pub struct ValidationRegistry<T> {
    pub validators: HashMap<ValidationType, Box<dyn Validator<T>>>,
}

impl ValidationRegistry<GetCmd> {
    pub fn new() -> Self {
        let mut validators: HashMap<ValidationType, Box<dyn Validator<GetCmd>>> = HashMap::new();
        validators.insert(ValidationType::MasterKeyCheck, Box::new(MasterKeyValidator {}));
        validators.insert(ValidationType::SessionCheck, Box::new(SessionValidator {}));
        validators.insert(ValidationType::RateLimitCheck, Box::new(RateLimitValidator {}));
        validators.insert(ValidationType::EntryExistsCheck, Box::new(EntryExistsValidator {}));
        validators.insert(ValidationType::DuplicateEntryCheck, Box::new(DuplicateEntryValidator {}));
        Self { validators }
    }
}

impl ValidationRegistry<AddCmd> {
    pub fn new() -> Self {
        let mut validators: HashMap<ValidationType, Box<dyn Validator<AddCmd>>> = HashMap::new();
        validators.insert(ValidationType::MasterKeyCheck, Box::new(MasterKeyValidator {}));
        validators.insert(ValidationType::RateLimitCheck, Box::new(RateLimitValidator {}));
        validators.insert(ValidationType::EntryExistsCheck, Box::new(EntryExistsValidator {}));
        validators.insert(ValidationType::DuplicateEntryCheck, Box::new(DuplicateEntryValidator {}));
        Self { validators }
    }
}

impl ValidationRegistry<InitCmd> {
    pub fn new() -> Self {
        let validators = HashMap::new();
        Self { validators }
    }
}
