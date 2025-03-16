pub mod Db {
    use rusqlite::{params, Connection, Result};
    use std::path::PathBuf;
    


    #[warn(unused_variables)]
    #[warn(unused_imports)]

    pub struct Database {
        pub connection: Connection
    }

    impl Database {
        pub fn new(path :&PathBuf) -> Result<Self, rusqlite::Error>
        {
            Ok(Database {
                connection: Connection::open(path)?
            })
        }
        // set up the db schema
        pub fn initialize(&self) -> Result<(), rusqlite::Error> {
            self.connection.execute_batch(
                "
                CREATE TABLE IF NOT EXISTS entry (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    ent_name TEXT NOT NULL,
                    password_hash TEXT NOT NULL,
                    timestamp TEXT NOT NULL
                );
                "
            )?;

            self.connection.execute_batch(
                "
                CREATE TABLE IF NOT EXISTS settings (
                    key TEXT PRIMARY KEY,
                    value TEXT NOT NULL,
                    description TEXT
                );
                "
            )?;

            Ok(())
        }
        pub fn add_entry(&self, entry: Entry) -> Result<(), rusqlite::Error> {
            self.connection.execute(
                "INSERT INTO entry (ent_name, password_hash, timestamp) VALUES (?1, ?2, ?3)",
                params![entry.ent_name, entry.password_hash, entry.timestamp],
            )?;
            Ok(())
        }

        pub fn get_entry_by_name(&self, ent_name: &str) -> Result<Entry, rusqlite::Error> {
            let mut stmt = self.connection.prepare("SELECT id, ent_name, password_hash, timestamp FROM entry WHERE ent_name = ?1")?;
            let entry = stmt.query_row(params![ent_name], |row| {
                Ok(Entry {
                    id: row.get(0)?,
                    ent_name: row.get(1)?,
                    password_hash: row.get(2)?,
                    timestamp: row.get(3)?,
                })
            })?;
            Ok(entry)
        }

        pub fn get_entry(&self, id: u32) -> Result<Entry, rusqlite::Error> {
            let mut stmt = self.connection.prepare("SELECT id, ent_name, password_hash, timestamp FROM entry WHERE id = ?1")?;
            let entry = stmt.query_row(params![id], |row| {
                Ok(Entry {
                    id: row.get(0)?,
                    ent_name: row.get(1)?,
                    password_hash: row.get(2)?,
                    timestamp: row.get(3)?,
                })
            })?;
            Ok(entry)
        }

        pub fn update_entry(&self, id: u32, entry: Entry) -> Result<(), rusqlite::Error> {
            self.connection.execute(
                "UPDATE entry SET ent_name = ?1, password_hash = ?2, timestamp = ?3 WHERE id = ?4",
                params![entry.ent_name, entry.password_hash, entry.timestamp, id],
            )?;
            Ok(())
        }

        pub fn delete_entry(&self, id: u32) -> Result<(), rusqlite::Error> {
            self.connection.execute("DELETE FROM entry WHERE id = ?1", params![id])?;
            Ok(())
        }

        pub fn entry_exist(&self, ent_name: String) -> Result<(bool), rusqlite::Error> {
            let mut stmt = self.connection.prepare("SELECT COUNT(*) FROM entry WHERE ent_name = ?1")?;
            let count: i32 = stmt.query_row(params![ent_name], |row| row.get(0))?;
            Ok(count > 0)

        }

        pub fn list_entries(&self) -> Result<Vec<Entry>, rusqlite::Error> {
            let mut stmt = self.connection.prepare("SELECT id, ent_name, password_hash, timestamp FROM entry")?;
            let entry_iter = stmt.query_map([], |row| {
                Ok(Entry {
                    id: row.get(0)?,
                    ent_name: row.get(1)?,
                    password_hash: row.get(2)?,
                    timestamp: row.get(3)?,
                })
            })?;
    
            let mut entries = Vec::new();
            for entry in entry_iter {
                entries.push(entry?);
            }
            Ok(entries)
        }


        pub fn list_settings(&self) -> Result<Vec<Setting>, rusqlite::Error> {
            let mut stmt = self.connection.prepare("SELECT key, value, description from settings")?;
            let setting_iter = stmt.query_map([], |row| {
                Ok(Setting {
                    key: row.get(0)?,
                    value: row.get(1)?,
                    desc: row.get(2)?,
                })
            })?;
    
            let mut settings = Vec::new();
            for setting in setting_iter {
                settings.push(setting?);
            }
            Ok(settings)
        }
        
    
        pub fn get_setting_value(&self, key: &str) -> Result<Option<String>> {
            let mut stmt = self.connection.prepare("SELECT value FROM settings WHERE key = ?1")?;
            let result = stmt.query_row(&[key], |row| row.get(0))?;
            Ok(result)
        }
    
        pub fn set_setting_value(&self, key: &str, value: &str) -> Result<()> {
            self.connection.execute("INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)", &[key, value])?;
            Ok(())
        }
    
        pub fn get_setting_description(&self, key: &str) -> Result<Option<String>> {
            let mut stmt = self.connection.prepare("SELECT description FROM settings WHERE key = ?1")?;
            let result = stmt.query_row(&[key], |row| row.get(0))?;
            Ok(result)
        }
    
        pub fn initialize_default_settings(self) -> Result<()> {
            // Insert default settings if they don't exist
            Ok(())
        }
    
    
    
    }

    pub struct Entry {
        pub id: u32,
        pub ent_name: String, 
        pub password_hash: String,
        pub timestamp: String,
    }

    impl Entry {
        pub fn new(id: u32, ent_name: String, password_hash: String) -> Self
        {
            Entry {
                id,
                ent_name,
                password_hash,
                timestamp: String::from("12-12-12")
            }
        }
    }

    pub struct Setting {
        pub key : String,
        pub value: String,
        pub desc: String,
    }

    impl Setting{
        pub fn new(key: String, value: String, desc: String) -> Self
        {
            Setting {
                key,
                value,
                desc,
            }
        }
    }

}