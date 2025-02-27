
use crate::validator::core::{Validator, ValidationResult};
use crate::context::Context;
use crate::cli::commands::{AddCmd, GetCmd, UpdateCmd};

pub struct EntryExistsValidator {}

// For GetCmd: succeed if the entry exists.
impl Validator<GetCmd> for EntryExistsValidator {
    fn validate(&self, context: &Context, cmd: &GetCmd) -> ValidationResult {
        log::debug!("Running EntryExistsValidator for GetCmd");
        match context.db.entry_exist(cmd.ent_name.clone()) {
            Ok(exists) => {
                if exists {
                    ValidationResult::Success
                } else {
                    ValidationResult::Failure("No entry found with similar name ⛔".to_string())
                }
            }
            Err(_) => ValidationResult::Failure("Error during DB check ⛔".to_string()),
        }
    }
}

// For AddCmd: fail if the entry already exists.
impl Validator<AddCmd> for EntryExistsValidator {
    fn validate(&self, context: &Context, cmd: &AddCmd) -> ValidationResult {
        log::debug!("Running EntryExistsValidator for AddCmd");
        match context.db.entry_exist(cmd.name.clone()) {
            Ok(exists) => {
                if exists {
                    ValidationResult::Failure("Entry already exists ⛔".to_string())
                } else {
                    ValidationResult::Success
                }
            }
            Err(_) => ValidationResult::Failure("Error during DB check ⛔".to_string()),
        }
    }
}

// entry has to exist
impl Validator<UpdateCmd> for EntryExistsValidator {
    fn validate(&self, context: &Context, cmd: &UpdateCmd) -> ValidationResult {
        log::debug!("Running EntryExistsValidator for UpdateCmd");
        match context.db.entry_exist(cmd.name.clone()) {
            Ok(exists) => {
                if exists {
                    ValidationResult::Success
                } else {
                    ValidationResult::Failure("Entry already exists ⛔".to_string())
                }
            }
            Err(_) => ValidationResult::Failure("Error during DB check ⛔".to_string()),
        }
    }
}
