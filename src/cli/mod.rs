use crate::context::Context;
use crate::errors::{ErrorExecution, ErrorValidation};

pub trait Command {
    fn validate(&self, context: &Context) -> Result<(), ErrorValidation>;
    fn execute(&self, context: &Context) -> Result<(), ErrorExecution>;
    fn display(&self);
}

// Re-export commands
pub mod commands;
