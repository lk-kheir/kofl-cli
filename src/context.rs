use crate::config::Config::KoflGlobalConfig;
use crate::db::Db::Database;
use crate::errors::ErrorSetup;
use crate::session;
use rusqlite::Error;
use std::cell::RefCell;
use crate::session::Session;
use crate::session::SessionError;
use log::{debug, info, warn, error};
use colored::*;
use std::io::Write;

#[warn(unused_variables)]
#[warn(unused_imports)]
pub struct Context {
    pub kgc: RefCell<KoflGlobalConfig>,
    pub ss: Session,
    pub db: Database,
}

impl Context {
    pub fn new() -> Result<Self, ErrorSetup> {
        // Initialize the configuration
        let mut config = KoflGlobalConfig::new();
        config.load();

        // Wrap the configuration in a RefCell
        let c = RefCell::new(config);

        // Initialize the database
        let dbase = match Database::new(c.borrow().get_data_storage_path()) {
            Ok(database) => database,
            Err(err) => {
                error!("Error creating a connection to db: {}", err);
                return Err(ErrorSetup::DataBase);
            }
        };

        // Initialize the database schema
        let _ = dbase.initialize();

        // Initialize or load the session
        let user_login = match std::env::var("USER") {
            Ok(val) => val,
            Err(_) => String::from("default_user"),
        };
        
        let mut allow_recreation_of_session = false;
        let mut session;
        
        
        if (c.borrow().is_master_key_provided()) {
            allow_recreation_of_session = true;
            session = Session::new(user_login.clone(), true);
        }
        else {
            session = Session::new(user_login.clone(), false);
        }
        


        match session.load() {
            Ok(_) => {
                debug!("Successfully loaded the session file.");
            }
            Err(SessionError::SessionFileMissingError) => {
                debug!("Session config file missing, creating a new session.");
                if allow_recreation_of_session {
                    session = Session::new(user_login, true);
                    session.write_session_config_to_toml_file();
                }else {
                    session = Session::new(user_login, false);
                    session.write_session_config_to_toml_file(); 
                }
            }
            Err(SessionError::FailedLoadingError) => {
                debug!("Failed to load the session details, creating a new session.");
                if allow_recreation_of_session {
                    session = Session::new(user_login, true);
                    session.write_session_config_to_toml_file();
                }else {
                    session = Session::new(user_login, false);
                    session.write_session_config_to_toml_file(); 
                }
            }
            Err(SessionError::ExpiredSession) => {
                // warn!("Session expired");
                // if the session is expired we should ask the user to login again.
                // return Err(ErrorSetup::Session);
                // session = Session::new(user_login);
                // session.write_session_config_to_toml_file();
            }
            Err(_) => {
                // warn!("No existing session, creating a new session.");
                if allow_recreation_of_session {
                    session = Session::new(user_login, true);
                    session.write_session_config_to_toml_file();
                }else {
                    session = Session::new(user_login, false);
                    session.write_session_config_to_toml_file(); 
                }
            }
        }

        // Return the new Context
        Ok(Context { kgc: c, db: dbase, ss: session })
    }
}