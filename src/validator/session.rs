// src/validator/session.rs

use crate::validator::core::{Validator, ValidationResult};
use crate::context::Context;
use crate::cli::commands::GetCmd;

pub struct SessionValidator {}

impl Validator<GetCmd> for SessionValidator {
    fn validate(&self, context: &Context, _cmd: &GetCmd) -> ValidationResult {
        log::info!("Running SessionValidator");
        if !context.ss.check_if_expired() {
            ValidationResult::Success
        } else {
            ValidationResult::Failure("Session expired ⛔".to_string())
        }
    }
}
