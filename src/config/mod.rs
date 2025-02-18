pub mod Config {

    use crate::utils::Utils::{check_existing_config, get_config_path, get_home_dir};
    use crate::backup::Backup;
    use log::{debug, error};
    use serde::{Deserialize, Serialize};
    use sha2::{Digest, Sha256};
    use std::env;
    use std::fmt::Debug;
    use std::fs;
    use std::path::PathBuf;
    use toml;

    #[derive(Serialize, Deserialize)]
    pub struct KoflGlobalConfig {
        config_path: PathBuf,
        data_storage_path: PathBuf,
        user_id: String,
        username: String,
        salt: String,
        hashed_pwd: String,
        master_key_provided: bool,
    }

    impl KoflGlobalConfig {
        pub fn new() -> KoflGlobalConfig {
            let home_dir = get_home_dir().expect("Home directory not found");
            let key = "USER";
            KoflGlobalConfig {
                config_path: home_dir.join(".kofl"),
                data_storage_path: home_dir.join("kofl.sqlite"),
                user_id: String::from("1234567"),
                username: match env::var(key) {
                    Ok(val) => val,
                    Err(_) => String::from("default_user"),
                },
                salt: String::from(""),
                hashed_pwd: String::from(""),
                master_key_provided: false,
            }
        }

        pub fn get_user_id(&self) -> &str {
            &self.user_id
        }

        pub fn get_data_storage_path<'a>(&'a self) -> &'a PathBuf {
            &self.data_storage_path
        }

        pub fn get_config_path<'a>(&'a self) -> &'a PathBuf {
            &self.config_path
        }

        pub fn set_config_path(&mut self, path: PathBuf) {
            self.config_path = path;
        }

        pub fn set_data_storage_path(&mut self, path: PathBuf) {
            self.data_storage_path = path;
        }

        pub fn set_salt(&mut self, salt_val: String) {
            self.salt = salt_val.clone();
        }

        pub fn get_salt(&self) -> String {
            self.salt.clone()
        }

        pub fn get_user_login(&self) -> String {
            self.username.clone()
        }

        pub fn set_master_key_hash(&mut self, hash_val: String) {
            self.hashed_pwd = hash_val.clone();
        }
        pub fn get_hashed_pwd(&self) -> String {
            self.hashed_pwd.clone()
        }

        pub fn set_master_key_provided(&mut self, is_set: bool) {
            self.master_key_provided = true;
        }
        pub fn is_master_key_provided(&self) -> bool {
            self.master_key_provided
        }

        pub fn get_config_checksum(&self) -> String {
            let content =
                fs::read_to_string(self.get_config_path()).unwrap_or_else(|_| String::new());

            let mut hasher = Sha256::new();
            hasher.update(content.as_bytes());
            hex::encode(hasher.finalize())
        }

        fn verify_integrity(&self) -> bool {
            // Read stored checksum from a separate file
            let checksum_path = self.get_config_path().with_extension("checksum");

            if let Ok(stored_checksum) = fs::read_to_string(&checksum_path) {
                let current_checksum = self.get_config_checksum();
                return stored_checksum == current_checksum;
            }
            false
        }

        pub fn load(&mut self) {
            if !check_existing_config() {
                println!("no existing config");
                self.write_config_to_toml_file();
                return;
            }

            if !self.verify_integrity() {
                error!("Config file integrity check failed! Possible tampering detected.");
            }

            match self.read_config_from_toml_file() {
                Ok(config) => {
                    *self = config;
                    // create a backup for now this is only for testing;
                    // let bc = Backup::new().unwrap();
                    // bc.create_new_backup(&self.get_config_path(), &self.get_data_storage_path(), &self.get_config_path().with_extension("checksum"));

                },
                Err(e) => {
                    error!("Failed to load config: {}", e);
                    // Handle error appropriately
                }
            }
        }

        pub fn update(&self) {
            self.write_config_to_toml_file();
            // Save checksum
            let checksum = self.get_config_checksum();
            let checksum_path = self.get_config_path().with_extension("checksum");
            fs::write(checksum_path, checksum).expect("Failed to write checksum file");
        }

        pub fn serialize_to_toml(&self) -> String {
            toml::to_string(self).expect("could not serialize struct into toml string")
        }

        pub fn write_config_to_toml_file(&self) {
            let toml_str = self.serialize_to_toml();
            debug!("toml str =\n{}", toml_str);
            let config_pth = &self.config_path;
            fs::write(config_pth, toml_str).expect("could not create toml file for config");
        }

        pub fn read_config_from_toml_file(
            &self,
        ) -> Result<KoflGlobalConfig, Box<dyn std::error::Error>> {
            let config_pth = &self.config_path;
            let toml_str = fs::read_to_string(config_pth)?;
            let config: KoflGlobalConfig = toml::from_str(&toml_str)?;
            Ok(config)
        }
    }

    impl Debug for KoflGlobalConfig {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "\nKofl Global Configuration:\n\
             ├─ User Info:\n\
             │  ├─ Username: {}\n\
             │  └─ User ID: {}\n\
             ├─ Paths:\n\
             │  ├─ Config: {}\n\
             │  └─ Storage: {}\n\
             ├─ Security:\n\
             │  ├─ Master Key Set: {}\n\
             │  ├─ Salt Present: {}\n\
             │  └─ Password Hash: {}\n\
             └─ Status: {}\n",
                self.username,
                self.user_id,
                self.config_path.display(),
                self.data_storage_path.display(),
                if self.master_key_provided {
                    "Yes"
                } else {
                    "No"
                },
                if !self.salt.is_empty() { "Yes" } else { "No" },
                if !self.hashed_pwd.is_empty() {
                    "Set"
                } else {
                    "Not Set"
                },
                if self.master_key_provided {
                    "Configured"
                } else {
                    "Needs Setup"
                }
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Config::KoflGlobalConfig;
    use serial_test::serial;
    use std::path::PathBuf;
    use std::{env, fs};
    use tempfile::TempDir;

    struct EnvGuard {
        key: &'static str,
        original: Option<String>,
    }

    impl EnvGuard {
        fn new(key: &'static str) -> Self {
            let original = env::var(key).ok();
            Self { key, original }
        }

        fn set_var(&self, value: &str) {
            env::set_var(self.key, value);
        }

        fn remove_var(&self) {
            env::remove_var(self.key);
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            match &self.original {
                Some(original) => env::set_var(self.key, original),
                None => env::remove_var(self.key),
            }
        }
    }

    // Add this setup function
    fn setup_test_env() -> EnvGuard {
        let guard = EnvGuard::new("USER");
        guard.set_var("lk-kheir"); // Use the correct username
        guard
    }

    fn get_expected_home() -> PathBuf {
        env::var("HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("/home/default"))
    }

    // Helper function to create a config with temp directory
    fn create_test_config(temp_dir: &TempDir) -> KoflGlobalConfig {
        let mut config = KoflGlobalConfig::new();
        let config_path = temp_dir.path().join(".kofl");
        let storage_path = temp_dir.path().join("kofl.sqlite");
        config.set_config_path(config_path);
        config.set_data_storage_path(storage_path);

        config
    }

    // Helper function to create a valid TOML config file
    fn create_valid_config_file(path: &PathBuf) {
        let username = env::var("USER").unwrap_or_else(|_| "default_user".to_string());
        let config_content = format!(
            r#"
                config_path = "/tmp/test/.kofl"
                data_storage_path = "/tmp/test/kofl.sqlite"
                user_id = "1234567"
                username = "{}"
                salt = "test_salt"
                hashed_pwd = "test_hash"
                master_key_provided = true
            "#,
            username
        );
        fs::write(path, config_content).expect("Failed to write test config file");
    }

    // Helper function to create an invalid TOML config file
    fn create_invalid_config_file(path: &PathBuf) {
        let invalid_content = r#"
                This is not a valid TOML file
                config_path = /invalid/path
                missing quotes and equals signs
            "#;
        fs::write(path, invalid_content).expect("Failed to write invalid test config file");
    }


    #[test]
    #[serial]
    fn test_verify_integrity()
    {
        let _guard = EnvGuard::new("USER");
        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        let config = KoflGlobalConfig::new();

        std::fs::write(config.get_config_path(), String::from("test"));

        //println!("content is {}", std::fs::read_to_string(config.get_config_path()).unwrap());

        let res_check_sum = config.get_config_checksum();

        // println!("{}", res_check_sum);
        assert_eq!("9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08", res_check_sum);


        // simlate the modif of a file

        std::fs::write(config.get_config_path(), String::from("testAgain"));
        let res_check_sum = config.get_config_checksum();
        // println!("{}", res_check_sum);
        assert_ne!("9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08", res_check_sum);


    }


    #[test]
    #[serial]
    fn test_new_config_default_values() {
        // Set up environment
        let _guard = EnvGuard::new("USER");
        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        // Create config with temp directory
        let config = create_test_config(&temp_dir);

        // Assert default values using temp paths
        let expected_config_path = temp_dir.path().join(".kofl");
        let expected_storage_path = temp_dir.path().join("kofl.sqlite");

        assert_eq!(config.get_config_path(), &expected_config_path);
        assert_eq!(config.get_data_storage_path(), &expected_storage_path);
        assert_eq!(config.get_salt(), "");
        assert_eq!(config.get_hashed_pwd(), "");
        assert!(!config.is_master_key_provided());
    }

    #[test]
    #[serial]
    fn test_new_config_with_user_env() {
        // Arrange
        env::set_var("USER", "lk-kheir");

        // Act
        let config = KoflGlobalConfig::new();

        // Assert
        assert_eq!(config.get_user_login(), "lk-kheir");

        // Cleanup
        env::remove_var("USER");
    }

    #[test]
    #[serial]
    fn test_new_config_without_user_env() {
        // Arrange
        env::remove_var("USER");

        // Act
        let config = KoflGlobalConfig::new();

        // Assert
        assert_eq!(config.get_user_login(), "default_user");
    }

    #[test]
    #[serial]
    fn test_new_config_paths_correctness() {
        // Set up environment
        let _guard = EnvGuard::new("USER");
        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        // Create config with temp directory
        let config = create_test_config(&temp_dir);

        // Assert path properties
        let config_path = config.get_config_path();
        let data_storage_path = config.get_data_storage_path();

        assert!(config_path.is_absolute(), "Config path should be absolute");
        assert!(
            data_storage_path.is_absolute(),
            "Data storage path should be absolute"
        );

        assert_eq!(
            config_path.file_name().unwrap().to_str().unwrap(),
            ".kofl",
            "Config file should be named .kofl"
        );
        assert_eq!(
            data_storage_path.file_name().unwrap().to_str().unwrap(),
            "kofl.sqlite",
            "Database file should be named kofl.sqlite"
        );
    }

    // Test path structure validity
    #[test]
    fn test_path_structure_validity() {
        // Set up environment
        let _guard = EnvGuard::new("USER");
        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        // Create config with temp directory
        let config = create_test_config(&temp_dir);

        let config_path = config.get_config_path();
        let data_storage_path = config.get_data_storage_path();

        // Test path validity
        assert!(!config_path.to_str().unwrap().contains(".."));
        assert!(!data_storage_path.to_str().unwrap().contains(".."));

        assert!(config_path.is_absolute());
        assert!(data_storage_path.is_absolute());

        // Test path components
        let config_components: Vec<_> = config_path.components().collect();
        let storage_components: Vec<_> = data_storage_path.components().collect();

        assert!(
            config_components.len() >= 2,
            "Path should have at least 2 components"
        );
        assert!(
            storage_components.len() >= 2,
            "Path should have at least 2 components"
        );
    }

    #[test]
    fn test_salt_operations() {
        // Arrange
        let mut config = KoflGlobalConfig::new();
        let test_salt = "VkfXMtmXShVXBBkv".to_string();

        // Act & Assert
        // Test initial empty state
        assert!(
            config.get_salt().is_empty(),
            "Salt should be empty initially"
        );

        // Test setting salt
        config.set_salt(test_salt.clone());
        assert_eq!(config.get_salt(), test_salt, "Salt should match set value");

        // Test updating salt
        let new_salt = "NewSaltValue123456".to_string();
        config.set_salt(new_salt.clone());
        assert_eq!(config.get_salt(), new_salt, "Salt should be updated");

        // Test empty salt
        config.set_salt("".to_string());
        assert!(
            config.get_salt().is_empty(),
            "Salt should be empty after clearing"
        );
    }

    #[test]
    fn test_master_key_hash_operations() {
        // Arrange
        let mut config = KoflGlobalConfig::new();
        let test_hash =
            "0302c4c69140fccfa36fb4ce6bcaed58fa65d221b6a8a6d5f2a183c056653c39".to_string();

        // Act & Assert
        // Test initial empty state
        assert!(
            config.get_hashed_pwd().is_empty(),
            "Hash should be empty initially"
        );

        // Test setting hash
        config.set_master_key_hash(test_hash.clone());
        assert_eq!(
            config.get_hashed_pwd(),
            test_hash,
            "Hash should match set value"
        );

        // Test updating hash
        let new_hash = "newhashnewhashnewhashnewhashnewhashnewhashnewhashnewha".to_string();
        config.set_master_key_hash(new_hash.clone());
        assert_eq!(config.get_hashed_pwd(), new_hash, "Hash should be updated");

        // Test empty hash
        config.set_master_key_hash("".to_string());
        assert!(
            config.get_hashed_pwd().is_empty(),
            "Hash should be empty after clearing"
        );
    }

    #[test]
    fn test_master_key_provided_flag() {
        // Arrange
        let mut config = KoflGlobalConfig::new();

        // Act & Assert
        // Test initial state
        assert!(
            !config.is_master_key_provided(),
            "Master key should not be provided initially"
        );

        // Test setting to true
        config.set_master_key_provided(true);
        assert!(
            config.is_master_key_provided(),
            "Master key should be marked as provided"
        );

        // Test setting to false
        config.set_master_key_provided(false);
        assert!(
            config.is_master_key_provided(),
            "Master key provided cannot be unset once set"
        );
    }

    #[test]
    fn test_security_state_combinations() {
        // Arrange
        let mut config = KoflGlobalConfig::new();
        let test_salt = "TestSalt123456789".to_string();
        let test_hash = "testhash123456789".to_string();

        // Act & Assert
        // Test initial state
        assert!(
            !config.is_master_key_provided(),
            "Should not be provided initially"
        );
        assert!(
            config.get_salt().is_empty(),
            "Salt should be empty initially"
        );
        assert!(
            config.get_hashed_pwd().is_empty(),
            "Hash should be empty initially"
        );

        // Test partial setup (only salt)
        config.set_salt(test_salt.clone());
        assert!(
            !config.is_master_key_provided(),
            "Should still not be provided"
        );
        assert!(!config.get_salt().is_empty(), "Salt should be set");
        assert!(
            config.get_hashed_pwd().is_empty(),
            "Hash should still be empty"
        );

        // Test complete setup
        config.set_master_key_hash(test_hash.clone());
        config.set_master_key_provided(true);
        assert!(config.is_master_key_provided(), "Should be provided now");
        assert_eq!(config.get_salt(), test_salt, "Salt should remain set");
        assert_eq!(config.get_hashed_pwd(), test_hash, "Hash should be set");
    }

    #[test]
    fn test_security_data_consistency() {
        // Set up environment - we need both USER and HOME
        let user_guard = EnvGuard::new("USER");
        let home_guard = EnvGuard::new("HOME");

        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        // Set HOME to our temp directory
        home_guard.set_var(temp_dir.path().to_str().unwrap());
        user_guard.set_var("lk-kheir");

        // Create initial config with temp directory
        let mut config = create_test_config(&temp_dir);

        // Set test values
        let test_salt = "asdfasdf".to_string();
        let test_hash = "asdfasdf".to_string();

        config.set_salt(test_salt.clone());
        config.set_master_key_hash(test_hash.clone());
        config.set_master_key_provided(true);

        // Save to temp file
        config.update();

        println!("Config after update: {:?}", config);
        println!("Config file path: {:?}", config.get_config_path());

        // Create new config instance with same temp directory
        let mut new_config = create_test_config(&temp_dir);
        new_config.load();

        println!("New config after load: {:?}", new_config);
        println!("New config file path: {:?}", new_config.get_config_path());

        // Assert persistence
        assert_eq!(new_config.get_salt(), test_salt, "Salt should persist");
        assert_eq!(
            new_config.get_hashed_pwd(),
            test_hash,
            "Hash should persist"
        );
        assert!(
            new_config.is_master_key_provided(),
            "Master key provided flag should persist"
        );
    }

    #[test]
    fn test_invalid_security_operations() {
        // Arrange
        let mut config = KoflGlobalConfig::new();

        // Test empty strings
        config.set_salt("".to_string());
        assert!(
            config.get_salt().is_empty(),
            "Salt should allow empty string"
        );

        config.set_master_key_hash("".to_string());
        assert!(
            config.get_hashed_pwd().is_empty(),
            "Hash should allow empty string"
        );

        // Test very long values
        let long_string = "a".repeat(1000);
        config.set_salt(long_string.clone());
        assert_eq!(
            config.get_salt().len(),
            1000,
            "Salt should handle long strings"
        );

        config.set_master_key_hash(long_string.clone());
        assert_eq!(
            config.get_hashed_pwd().len(),
            1000,
            "Hash should handle long strings"
        );
    }

    #[test]
    #[serial]
    fn test_load_with_existing_config() {
        let user_guard = EnvGuard::new("USER");
        let home_guard = EnvGuard::new("HOME");

        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        // Set HOME to our temp directory and USER to test user
        home_guard.set_var(temp_dir.path().to_str().unwrap());
        user_guard.set_var("lk-kheir");

        // Arrange
        let mut config = create_test_config(&temp_dir);
        let config_path = config.get_config_path().clone();

        // Create valid config file in temp directory
        create_valid_config_file(&config_path);

        println!("Test directory: {:?}", temp_dir.path());
        println!("Config path: {:?}", config_path);
        println!("Initial config: {:?}", config);   

        // Act
        config.load();

        println!("Config after load: {:?}", config);

        // Assert
        assert_eq!(
            config.get_user_login(),
            "lk-kheir", // Use the exact username we set
            "User login should match environment"
        );
        assert_eq!(
            config.get_salt(),
            "test_salt",
            "Salt should match test value"
        );
        assert_eq!(
            config.get_hashed_pwd(),
            "test_hash",
            "Hash should match test value"
        );
        assert!(
            config.is_master_key_provided(),
            "Master key should be marked as provided"
        );
    }

    #[test]
    #[serial]
    fn test_load_without_existing_config() {
        let _guard = setup_test_env();
        // Arrange
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let mut config = create_test_config(&temp_dir);

        println!("temp_dir = {:?}", temp_dir);
        println!("config = {:?}", config);

        // Act
        config.update();
        config.load();

        // Assert
        assert!(
            config.get_config_path().exists(),
            "Config file should be created"
        );
        assert_eq!(
            config.get_user_login(),
            env::var("USER").unwrap_or_else(|_| "default_user".to_string())
        );
        assert!(config.get_salt().is_empty());
        assert!(config.get_hashed_pwd().is_empty());
        assert!(!config.is_master_key_provided());
    }

    #[test]
    #[serial]
    fn test_load_with_invalid_config() {
        let _guard = setup_test_env();
        // Arrange
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let mut config = create_test_config(&temp_dir);
        let config_path = config.get_config_path().clone();
        create_invalid_config_file(&config_path);

        // Act
        config.update();
        config.load();

        // Assert
        // Should fall back to default values
        assert_eq!(
            config.get_user_login(),
            env::var("USER").unwrap_or_else(|_| "default_user".to_string())
        );
        assert!(config.get_salt().is_empty());
        assert!(config.get_hashed_pwd().is_empty());
        assert!(!config.is_master_key_provided());
    }

    #[test]
    fn test_update_config() {
        // Arrange
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let mut config = create_test_config(&temp_dir);

        // Set some values
        config.set_salt("new_salt".to_string());
        config.set_master_key_hash("new_hash".to_string());
        config.set_master_key_provided(true);

        // Act
        config.update();

        // Assert
        // Read the file directly and verify contents
        let config_content =
            fs::read_to_string(config.get_config_path()).expect("Failed to read config file");

        assert!(config_content.contains("new_salt"));
        assert!(config_content.contains("new_hash"));
        assert!(config_content.contains("master_key_provided = true"));
    }

    #[test]
    fn test_serialize_to_toml() {
        // Arrange
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let mut config = create_test_config(&temp_dir);

        // Set some test values
        config.set_salt("test_salt".to_string());
        config.set_master_key_hash("test_hash".to_string());
        config.set_master_key_provided(true);

        // Act
        let toml_str = config.serialize_to_toml();

        // Assert
        assert!(toml_str.contains("test_salt"));
        assert!(toml_str.contains("test_hash"));
        assert!(toml_str.contains("master_key_provided = true"));
        assert!(toml_str.contains(&config.get_user_login()));
    }

    #[test]
    fn test_write_and_read_config_file() {
        // Arrange
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let mut config = create_test_config(&temp_dir);

        // Set test values
        config.set_salt("test_salt".to_string());
        config.set_master_key_hash("test_hash".to_string());
        config.set_master_key_provided(true);

        // Act - Write
        config.write_config_to_toml_file();

        // Act - Read
        let read_result = config.read_config_from_toml_file();

        // Assert
        assert!(read_result.is_ok(), "Should successfully read config file");
        let read_config = read_result.unwrap();

        assert_eq!(read_config.get_salt(), "test_salt");
        assert_eq!(read_config.get_hashed_pwd(), "test_hash");
        assert!(read_config.is_master_key_provided());
    }

    #[test]
    fn test_read_config_file_permissions() {
        // Arrange
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let config = create_test_config(&temp_dir);
        let config_path = config.get_config_path();

        // Create config file with restricted permissions
        create_valid_config_file(&config_path);

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&config_path, fs::Permissions::from_mode(0o000))
                .expect("Failed to set permissions");
        }

        // Act
        let read_result = config.read_config_from_toml_file();

        // Assert
        assert!(
            read_result.is_err(),
            "Should fail to read with no permissions"
        );

        // Cleanup - restore permissions to allow cleanup
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&config_path, fs::Permissions::from_mode(0o644))
                .expect("Failed to restore permissions");
        }
    }
}
