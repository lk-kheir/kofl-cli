use crate::context::Context;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum ValidationType {
    SessionCheck, // this should be equivelant to loggedIn for now.
    MasterKeyCheck,
    RateLimitCheck,
    EntryExistsCheck,
    DuplicateEntryCheck,
    // more to come later
}

pub enum CommandType {
    GET_CMD,
    ADD_CMD,
    INIT_CMD,
    LOGIN_CMD
}

#[derive(Debug)]
pub enum ValidationResult {
    Success,
    Failure(String),
    Warning(String),
}

/// A generic trait for validators.
/// Each validator will implement this trait for one or more command types.
pub trait Validator<T> {
    fn validate(&self, context: &Context, cmd: &T) -> ValidationResult;
}
