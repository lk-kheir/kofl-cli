use crate::cli::commands::{AddCmd, DestroyCmd, GetCmd, InitCmd, LogInCmd};
use crate::validator::core::{Validator, ValidationResult};
use crate::context::Context;

pub struct MasterKeyValidator {}

impl Validator<InitCmd> for MasterKeyValidator {
    fn validate(&self, context: &Context, _cmd: &InitCmd) -> ValidationResult {
        log::debug!("Running MasterKeyValidator");
        if context.kgc.borrow().is_master_key_provided() {
            ValidationResult::Failure("Master key is already provided ⛔".to_string())
        } else {
            ValidationResult::Success
        }
    }
}

impl Validator<GetCmd> for MasterKeyValidator {
    fn validate(&self, context: &Context, _cmd: &GetCmd) -> ValidationResult {
        log::debug!("Running MasterKeyValidator");
        if context.kgc.borrow().is_master_key_provided() {
            ValidationResult::Success
        } else {
            ValidationResult::Failure("Master key not provided ⛔".to_string())
        }
    }
}

impl Validator<LogInCmd> for MasterKeyValidator {
    fn validate(&self, context: &Context, _cmd: &LogInCmd) -> ValidationResult {
        log::debug!("Running MasterKeyValidator");
        if context.kgc.borrow().is_master_key_provided() {
            ValidationResult::Success
        } else {
            ValidationResult::Failure("Master key not provided ⛔".to_string())
        }
    }
}

impl Validator<AddCmd> for MasterKeyValidator {
    fn validate(&self, context: &Context, _cmd: &AddCmd) -> ValidationResult {
        log::debug!("Running MasterKeyValidator");
        if context.kgc.borrow().is_master_key_provided() {
            ValidationResult::Success
        } else {
            ValidationResult::Failure("Master key not provided ⛔".to_string())
        }
    }
}

impl Validator<DestroyCmd> for MasterKeyValidator {
    fn validate(&self, context: &Context, _cmd: &DestroyCmd) -> ValidationResult {
        log::debug!("Running MasterKeyValidator");
        if context.kgc.borrow().is_master_key_provided() {
            ValidationResult::Success
        } else {
            ValidationResult::Failure("Master key not provided ⛔".to_string())
        }
    }
}