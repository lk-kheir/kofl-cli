use crate::config::Config::KoflGlobalConfig;
use crate::db::Db::{Database, Entry};
use rusqlite::Error;

#[warn(unused_variables)]
#[warn(unused_imports)]
pub struct Context {
    pub kgc: KoflGlobalConfig,
    pub db: Database
    // I can add more state here
}


impl Context {
    pub fn new() -> Result<Self, Error> {
        let mut c = KoflGlobalConfig::new();
        c.load();
        let dbase = match Database::new(c.get_data_storage_path()) {
            Ok(database) => database,
            Err(err) => {
                eprintln!("error creating a connection to db: {}", err);
                return Err(err);
            }
        };

        let _  = dbase.initialize();
        

        Ok(Context { kgc: (c), db: dbase })
    }
}