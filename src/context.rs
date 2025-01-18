use crate::config::Config::KoflGlobalConfig;
use crate::db::Db::Database;
use rusqlite::Error;
use std::cell::RefCell;
use crate::session::Session;
use crate::session::SessionError;

#[warn(unused_variables)]
#[warn(unused_imports)]
pub struct Context {
    pub kgc: RefCell<KoflGlobalConfig>,
    pub ss: Session,
    pub db: Database,
}

impl Context {
    pub fn new() -> Result<Self, Error> {
        // Initialize the configuration
        let mut config = KoflGlobalConfig::new();
        config.load();

        // Wrap the configuration in a RefCell
        let c = RefCell::new(config);

        // Initialize the database
        let dbase = match Database::new(c.borrow().get_data_storage_path()) {
            Ok(database) => database,
            Err(err) => {
                eprintln!("error creating a connection to db: {}", err);
                return Err(err);
            }
        };

        // Initialize the database schema
        let _ = dbase.initialize();

        // Initialize or load the session
        let user_login = match std::env::var("USER") {
            Ok(val) => val,
            Err(_) => String::from("default_user"),
        };

        let mut session = Session::new(user_login.clone());

        match session.load() {
            Ok(_) => {
                println!("Successfull file Loading");
            }
            Err(SessionError::SessionFileMissingError) => {
                println!("Config session missing, Attempting to create a new one");
                session = Session::new(user_login);
                session.write_session_config_to_toml_file();
            }
            Err(SessionError::FailedLoadingError) => {
                println!("Failed to load the session details, Attempting to create a new one");
                session = Session::new(user_login);
                session.write_session_config_to_toml_file();
            }
            Err(SessionError::ExpiredSession) => {
                println!("Session expired, Attempting to create a new one");
                session = Session::new(user_login);
                session.write_session_config_to_toml_file();
            }
            Err(_) => {
                println!("No existing session, creating a new session.");
                session = Session::new(user_login);
                session.write_session_config_to_toml_file();
            }
        }

        // Return the new Context
        Ok(Context { kgc: c, db: dbase, ss: session })
    }
}