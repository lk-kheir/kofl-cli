use crate::cli::Command;
use crate::errors::{ErrorExecution, ErrorValidation};
use crate::context::Context;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use sha2::{Sha256, Digest};


use aes::cipher::KeyIvInit;
use ctr::Ctr32BE;
type Aes256Ctr = Ctr32BE<aes::Aes256>;


pub struct LogInCmd {
    // for now is emty 
}

impl LogInCmd {
    pub fn new() -> Self {
        LogInCmd{}
    }

}

impl Command for LogInCmd {
    fn execute(&self, context: &Context) -> Result<(), ErrorExecution>  {
            Ok(())
    }

    fn validate(&self, context: &Context) -> Result<(), ErrorValidation>  {
        Ok(())
    }

    fn display(&self) {
        println!("Login Command");
        ()
    }
}

