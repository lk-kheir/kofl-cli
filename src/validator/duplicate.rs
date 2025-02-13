
use crate::validator::core::{Validator, ValidationResult};
use crate::context::Context;

pub struct DuplicateEntryValidator {}

impl<T> Validator<T> for DuplicateEntryValidator {
    fn validate(&self, _context: &Context, _cmd: &T) -> ValidationResult {
        log::debug!("Running DuplicateEntryValidator");
        ValidationResult::Success
    }
}
