// src/validator/session.rs

use crate::validator::core::{Validator, ValidationResult};
use crate::context::Context;
use crate::cli::commands::{GetCmd, LogInCmd};

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

impl Validator<LogInCmd> for SessionValidator {
    fn validate(&self, context: &Context, _cmd: &LogInCmd) -> ValidationResult {
        log::info!("Running SessionValidator");
        if context.ss.check_if_expired() {
            ValidationResult::Success // means that session expired and it makes sense to allow login command
        } else {
            ValidationResult::Failure("Non expired session , already loggedIn ✅".to_string())
        }
    }
}
