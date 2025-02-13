
use crate::validator::core::{Validator, ValidationResult};
use crate::context::Context;

pub struct MasterKeyValidator {}

impl<T> Validator<T> for MasterKeyValidator {
    fn validate(&self, context: &Context, _cmd: &T) -> ValidationResult {
        log::debug!("Running MasterKeyValidator");
        if context.kgc.borrow().is_master_key_provided() {
            ValidationResult::Success
        } else {
            ValidationResult::Failure("Master key not provided ⛔".to_string())
        }
    }
}
