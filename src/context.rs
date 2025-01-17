use crate::config::Config::KoflGlobalConfig;
use crate::db::Db::Database;
use rusqlite::Error;
use std::cell::RefCell;
use crate::Session;

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


        let mut s  = Session::new();
        s.load();
  
        // if (s.check_if_expired()) {
        //     println!("session expired run cargo login");
        // }else {
        //     println!("session still valid");
        // }


        // Return the new Context
        Ok(Context { kgc: c, db: dbase, ss: s })
    }
}