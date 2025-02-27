use crate::constants::CONS;
use crate::validator::core::{Validator, ValidationResult};
use crate::context::Context;
use crate::cli::commands::AddCmd;
use regex::Regex;

pub struct PasswordRequirementValidator {}

impl PasswordRequirementValidator {
    fn has_repeated_sequence(password: &str, length: usize) -> bool {
        let chars: Vec<char> = password.chars().collect();
        for i in 0..=chars.len().saturating_sub(length) {
            if chars[i..i + length].windows(2).all(|w| w[0] == w[1]) {
                return true;
            }
        }
        false
    }
}


impl Validator<AddCmd> for PasswordRequirementValidator {
    fn validate(&self, _context: &Context, cmd: &AddCmd) -> ValidationResult {
        log::debug!("Running PasswordRequirementValidator");
        if cmd.password.len() < CONS::MIN_PASSWORD_LENGTH {
            let message = format!(
                "Password requirements failed: Minimum length is {} but the provided password is {} characters long",
                CONS::MIN_PASSWORD_LENGTH,
                cmd.password.len()
            );
            return ValidationResult::Failure(message);
        }

        let uppercase_regex = Regex::new(CONS::PASSWORD_UPPERCASE_REQ).unwrap();
        if !uppercase_regex.is_match(&cmd.password) {
            let message = "Password requirements failed: At least one uppercase letter (A-Z) is required".to_string();
            // log::error!("{}", message);
            return ValidationResult::Failure(message);
        }

        let lowercase_regex = Regex::new(CONS::PASSWORD_LOWERCASE_REQ).unwrap();
        if !lowercase_regex.is_match(&cmd.password) {
            let message = "Password requirements failed: At least one lowercase letter (a-z) is required".to_string();
            // log::error!("{}", message);
            return ValidationResult::Failure(message);
        }

        let digit_regex = Regex::new(CONS::PASSWORD_DIGIT_REQ).unwrap();
        if !digit_regex.is_match(&cmd.password) {
            let message = "Password requirements failed: At least one digit (0-9) is required".to_string();
            // log::error!("{}", message);
            return ValidationResult::Failure(message);
        }

        let special_char_regex = Regex::new(CONS::PASSWORD_SPECIAL_CHAR_REQ).unwrap();
        if !special_char_regex.is_match(&cmd.password) {
            let message = "Password requirements failed: At least one special character (e.g., !, @, #, $, etc.) is required".to_string();
            // log::error!("{}", message);
            return ValidationResult::Failure(message);
        }

        if PasswordRequirementValidator::has_repeated_sequence(&cmd.password, 4) {
            let message = "Password requirements failed: Password should not have a sequence of repeated characters (e.g., 'aaaa', '1111')".to_string();
            // log::error!("{}", message);
            return ValidationResult::Failure(message);
        }


        ValidationResult::Success
    }
}