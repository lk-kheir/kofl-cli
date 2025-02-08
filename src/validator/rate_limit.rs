// src/validator/rate_limit.rs

use crate::validator::core::{Validator, ValidationResult};
use crate::context::Context;

pub struct RateLimitValidator {}

impl<T> Validator<T> for RateLimitValidator {
    fn validate(&self, _context: &Context, _cmd: &T) -> ValidationResult {
        log::info!("Running RateLimitValidator");
        ValidationResult::Success
    }
}
