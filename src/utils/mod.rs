
pub mod Utils {

    use std::env;
    use std::path::PathBuf;
    use std::fs;
    
    /// Utility function to get the user's home directory.
    /// Returns a PathBuf representing the home directory path.
    pub fn get_home_dir() -> Option<PathBuf> {
        env::home_dir()
    }

    pub fn get_backup_dir() -> Option<PathBuf> {
        let home_dir = get_home_dir().expect("Could not find home directory");
        let back_dir = home_dir.join(".kofl_backups");
    
        if back_dir.is_dir() {
            Some(back_dir)
        } else {
            // Try to create the backup directory
            match std::fs::create_dir_all(&back_dir) {
                Ok(_) => Some(back_dir),
                Err(e) => {
                    eprintln!("Failed to create backup directory: {}", e);
                    None
                }
            }
        }
    }
    
    /// Utility function to create a config path in the user's home directory.
    /// Takes a filename as a parameter and returns the full path.
    pub fn get_config_path(filename: &str) -> Option<PathBuf> {
        if let Some(mut home_dir) = get_home_dir() {
            home_dir.push(filename);
            Some(home_dir)
        } else {
            None
        }
    }

    pub fn check_existing_config() -> bool {
        if let Some(home_dir) = get_home_dir() {
            // println!("home dir = {:?}", home_dir.join(".kofl"));
            return fs::exists(home_dir.join(".kofl")).unwrap();
        }
        false
    }

    pub fn check_existing_session_config() -> bool {
        if let Some(home_dir) = get_home_dir() {
            // println!("home dir = {:?}", home_dir.join(".kofl"));
            return fs::exists(home_dir.join(".kofl_session")).unwrap();
        }
        false
    }
}