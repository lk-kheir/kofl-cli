use crate::context::Context;
use crate::errors::{ErrorExecution, ErrorValidation};

pub trait Command {
    fn validate(&self, _context: &Context) -> bool;
    fn execute(&self, context: &Context) -> bool;
    fn display(&self);
}

// Re-export commands
pub mod commands;
