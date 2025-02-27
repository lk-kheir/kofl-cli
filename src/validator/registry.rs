// src/validator/registry.rs

use std::collections::HashMap;
use crate::validator::core::{ValidationType, Validator};
use crate::cli::commands::{AddCmd, DestroyCmd, GetCmd, InitCmd, LogInCmd};

use crate::validator::master_key::MasterKeyValidator;
use crate::validator::session::SessionValidator;
use crate::validator::rate_limit::RateLimitValidator;
use crate::validator::entry_exists::EntryExistsValidator;
use crate::validator::duplicate::DuplicateEntryValidator;

use super::pwd_req::PasswordRequirementValidator;

pub struct ValidationRegistry<T> {
    pub validators: HashMap<ValidationType, Box<dyn Validator<T>>>,
}

impl ValidationRegistry<InitCmd> {
    pub fn new() -> Self {
        let mut validators:  HashMap<ValidationType, Box<dyn Validator<InitCmd>>> = HashMap::new();
        validators.insert(ValidationType::MasterKeyCheck, Box::new(MasterKeyValidator {}));
        Self { validators }
    }
}

impl ValidationRegistry<LogInCmd> {
    pub fn new() -> Self {
        let mut validators: HashMap<ValidationType, Box<dyn Validator<LogInCmd>>> = HashMap::new();
        validators.insert(ValidationType::MasterKeyCheck, Box::new(MasterKeyValidator {}));
        validators.insert(ValidationType::SessionCheck, Box::new(SessionValidator {}));
        Self { validators }
    }
}

impl ValidationRegistry<DestroyCmd> {
    pub fn new() -> Self {
        let mut validators: HashMap<ValidationType, Box<dyn Validator<DestroyCmd>>> = HashMap::new();
        validators.insert(ValidationType::MasterKeyCheck, Box::new(MasterKeyValidator {}));
        validators.insert(ValidationType::SessionCheck, Box::new(SessionValidator {}));
        Self { validators }
    }
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
        validators.insert(ValidationType::SessionCheck, Box::new(SessionValidator {}));
        validators.insert(ValidationType::MasterKeyCheck, Box::new(MasterKeyValidator {}));
        validators.insert(ValidationType::RateLimitCheck, Box::new(RateLimitValidator {}));
        validators.insert(ValidationType::EntryExistsCheck, Box::new(EntryExistsValidator {}));
        validators.insert(ValidationType::DuplicateEntryCheck, Box::new(DuplicateEntryValidator {}));
        validators.insert(ValidationType::PasswordRequirementCheck, Box::new(PasswordRequirementValidator {}));
        Self { validators }
    }
}

