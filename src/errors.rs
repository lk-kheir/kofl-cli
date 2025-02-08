use std::fmt::{self};
use std::io;

#[derive(Debug)]
pub enum ErrorSetup {
    Session,
    DataBase,
}




impl fmt::Display for ErrorSetup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        match self {
            ErrorSetup::Session => write!(f, "Session set up failed"),
            ErrorSetup::DataBase => write!(f, "DataBase set up failed")
        }
        
    }
}


pub enum ErrorValidation {
    EmptyName,
    LongName,
    UnrespectedPasswordProtocol,
    StoragePathNotFound,
    UnprovidedMasterKey,
    AlreadyProvidedMasterKey,
    Temp,
}




impl fmt::Display for ErrorValidation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        match self {
            ErrorValidation::Temp => write!(f, "I really Dont konw."),
            ErrorValidation::LongName  => write!(f, "Long name expected no more than 60 caracters (example))."),
            ErrorValidation::EmptyName  => write!(f, "Provid a name to be associated with Your Password."),
            ErrorValidation::UnrespectedPasswordProtocol  => write!(f, "Password does not respect the the security protocol."),
            ErrorValidation::StoragePathNotFound => write!(f,"Storage Path not found."),
            ErrorValidation::UnprovidedMasterKey =>  write!(f,"Unprovided Master Key."),
            ErrorValidation::AlreadyProvidedMasterKey => write!(f,"Already provided Master Key ignoring init phase."),
        }
        
    }
}
pub enum ErrorExecution {
    IoError(io::Error),
    NoMatchingEntry,
    PasswordMismatch,
    Unknown,
    DecryptionError,
    EncryptionError,
    DatabaseError,
    AuthenticationFailed
}

impl From<io::Error> for ErrorExecution {
    fn from(err: io::Error) -> ErrorExecution {
        ErrorExecution::IoError(err)
    }
}

impl fmt::Display for ErrorExecution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorExecution::IoError(e) => write!(f, "I/O Error: {}", e),
            ErrorExecution::Unknown => write!(f, "Something unexpected happened during execution."),
            ErrorExecution::NoMatchingEntry => write!(f, "No matching entry found."),
            ErrorExecution::PasswordMismatch => write!(f, "unmatching passwords."),
            ErrorExecution::DecryptionError => write!(f, "DecryptionError occured."),
            ErrorExecution::EncryptionError => write!(f, "EncryptionError occured."),
            ErrorExecution::DatabaseError => write!(f, "DatabaseError occured."),
            ErrorExecution::AuthenticationFailed => write!(f, "Authentication failed."),

        }
    }
}
