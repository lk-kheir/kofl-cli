use crate::{
    db::Db::Entry,
    utils::Utils::{get_backup_dir, get_home_dir},
};
use chrono::{DateTime, Utc};
use std::{fs, path::PathBuf};

pub struct Backup {
    backup_dir: PathBuf,
    backup_empty: bool,
}

impl Backup {
    pub fn new() -> Result<Backup, std::io::Error> {
        let backup_dir = get_backup_dir().ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Failed to create or access backup directory",
            )
        })?;

        Ok(Backup {
            backup_dir,
            backup_empty: false,
        })
    }

    pub fn create_new_backup(
        &self,
        kgc_file: &PathBuf,
        db_file: &PathBuf,
        checksumfile: &PathBuf,
    ) -> Result<(), std::io::Error> {
        // Format directory name as YYYY-MM-DD_HH_MM_SS
        let dir_name = Utc::now().format("%Y-%m-%d_%H_%M_%S").to_string();

        // Create backup directory path
        let backup_dir_path = self.backup_dir.join(&dir_name);

        // Create the directory
        fs::create_dir_all(&backup_dir_path)?;

        // Copy files to backup directory
        fs::copy(kgc_file, backup_dir_path.join(".kofl"))?;
        fs::copy(db_file, backup_dir_path.join("kofl.sqlite"))?;
        fs::copy(checksumfile, backup_dir_path.join(".kofl.checksum"))?;

        Ok(())
    }

    pub fn get_last_backup(&self) -> std::io::Result<Option<PathBuf>> {
        let mut entries = fs::read_dir(&self.backup_dir)?
            .filter_map(|e| e.ok())
            .inspect(|entry| println!("{:?}", entry.path()))
            .collect::<Vec<_>>();

        entries.sort_by_key(|dir| dir.metadata().and_then(|m| m.modified()).ok());

        if let Some(last_entry) = entries.last() {
            // println!("Last backup directory: {:?}", last_entry.file_name());
            // println!("Path: {:?}", last_entry.path());
            // println!("Modified: {:?}", last_entry.metadata()?.modified()?);

            // Print files inside the last backup directory
            for entry in fs::read_dir(last_entry.path())? {
                let file = entry?;
                println!("{:?}", file.path());
            }

            return Ok(Some(last_entry.path()));
        }

        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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

    fn create_test_files(temp_dir: &TempDir) -> (PathBuf, PathBuf, PathBuf) {
        let kgc_file = temp_dir.path().join(".kofl");
        let db_file = temp_dir.path().join("kofl.sqlite");
        let checksum_file = temp_dir.path().join(".kofl.checksum");

        // Create test files with some content
        fs::write(&kgc_file, "test config content").unwrap();
        fs::write(&db_file, "test database content").unwrap();
        fs::write(&checksum_file, "test checksum content").unwrap();

        (kgc_file, db_file, checksum_file)
    }

    #[test]
    fn test_backup_creation_success() {
        let temp_dir = tempfile::tempdir().unwrap();
        let home_guard = EnvGuard::new("HOME");
        home_guard.set_var(temp_dir.path().to_str().unwrap());

        let backup = Backup::new();
        assert!(backup.is_ok());

        let backup = backup.unwrap();
        assert!(backup.backup_dir.exists());
        assert!(backup.backup_dir.is_dir());
    }

    #[test]
    fn test_backup_dir_already_exists() {
        let temp_dir = tempfile::tempdir().unwrap();
        let home_guard = EnvGuard::new("HOME");
        home_guard.set_var(temp_dir.path().to_str().unwrap());

        // Create backup directory manually first
        let backup_path = temp_dir.path().join(".kofl_backups");
        fs::create_dir_all(&backup_path).unwrap();

        let backup = Backup::new();
        assert!(backup.is_ok());
    }

    #[test]
    fn test_backup_empty_flag() {
        let temp_dir = tempfile::tempdir().unwrap();
        let home_guard = EnvGuard::new("HOME");
        home_guard.set_var(temp_dir.path().to_str().unwrap());

        let backup = Backup::new().unwrap();
        assert!(!backup.backup_empty);
    }

    #[test]
    fn test_get_last_backup() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let backup_dir = temp_dir.path().join("backups");
        
        let backup = Backup {
            backup_dir: backup_dir.clone(),
            backup_empty: true,
        };

        let (kgc_file, db_file, checksum_file) = create_test_files(&temp_dir);
        let (kgc_file1, db_file1, checksum_file1) = create_test_files(&temp_dir);

        // Act
        let result = backup.create_new_backup(&kgc_file, &db_file, &checksum_file);
        assert!(result.is_ok(), "First backup creation should succeed");

        std::thread::sleep(std::time::Duration::from_secs(1)); // Ensure different timestamp

        let result1 = backup.create_new_backup(&kgc_file1, &db_file1, &checksum_file1);
        assert!(result1.is_ok(), "Second backup creation should succeed");

        let last_backup_path = backup.get_last_backup().expect("Failed to get last backup");

        // Assert
        let entries = fs::read_dir(&backup_dir).unwrap().collect::<Vec<_>>();
        assert_eq!(entries.len(), 2, "There should be two backup directories");

        // Check if the last backup path is correct
        assert!(last_backup_path.is_some(), "Last backup path should exist");
        let last_backup_path = last_backup_path.unwrap();
        assert!(last_backup_path.exists(), "Last backup path should exist on the filesystem");

        // Print the last backup directory for verification
        println!("Last backup directory: {:?}", last_backup_path);
    }

    #[test]
    fn test_create_new_backup_success() {
        // Arrange
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let backup_dir = temp_dir.path().join("backups");

        let backup = Backup {
            backup_dir: backup_dir.clone(),
            backup_empty: true,
        };

        let (kgc_file, db_file, checksum_file) = create_test_files(&temp_dir);

        // Act
        let result = backup.create_new_backup(&kgc_file, &db_file, &checksum_file);

        // Assert
        assert!(result.is_ok(), "Backup creation should succeed");

        // Check if backup directory exists
        let backup_dirs = fs::read_dir(&backup_dir).unwrap();
        let backup_dir_entry = backup_dirs.into_iter().next().unwrap().unwrap();

        // Verify directory name format
        let dir_name = backup_dir_entry.file_name();
        let dir_name_str = dir_name.to_str().unwrap();
        // println!("{}", dir_name_str);
        // assert!(dir_name_str.matches(r"d{4}-d{2}-d{2}_d{2}_d{2}_d{2}").count() == 1,
        //         "Backup directory name should match YYYY-MM-DD_HH_MM_SS format");

        // Verify files exist and content matches
        let backup_path = backup_dir_entry.path();
        assert!(
            backup_path.join(".kofl").exists(),
            "Config file should exist in backup"
        );
        assert!(
            backup_path.join("kofl.sqlite").exists(),
            "Database file should exist in backup"
        );
        assert!(
            backup_path.join(".kofl.checksum").exists(),
            "Checksum file should exist in backup"
        );

        // Verify content
        assert_eq!(
            fs::read_to_string(backup_path.join(".kofl")).unwrap(),
            "test config content"
        );
        assert_eq!(
            fs::read_to_string(backup_path.join("kofl.sqlite")).unwrap(),
            "test database content"
        );
        assert_eq!(
            fs::read_to_string(backup_path.join(".kofl.checksum")).unwrap(),
            "test checksum content"
        );
    }

    #[test]
    fn test_create_new_backup_with_missing_source_files() {
        // Arrange
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let backup_dir = temp_dir.path().join("backups");

        let backup = Backup {
            backup_dir: backup_dir.clone(),
            backup_empty: true,
        };

        let nonexistent_file = temp_dir.path().join("nonexistent");

        // Act
        let result =
            backup.create_new_backup(&nonexistent_file, &nonexistent_file, &nonexistent_file);

        // Assert
        assert!(
            result.is_err(),
            "Backup should fail with missing source files"
        );
    }

    #[test]
    fn test_create_new_backup_with_existing_backup() {
        // Arrange
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let backup_dir = temp_dir.path().join("backups");

        let backup = Backup {
            backup_dir: backup_dir.clone(),
            backup_empty: false,
        };

        let (kgc_file, db_file, checksum_file) = create_test_files(&temp_dir);

        // Create first backup
        backup
            .create_new_backup(&kgc_file, &db_file, &checksum_file)
            .unwrap();

        // Wait a second to ensure different timestamp
        std::thread::sleep(std::time::Duration::from_secs(1));

        // Create second backup
        let result = backup.create_new_backup(&kgc_file, &db_file, &checksum_file);

        // Assert
        assert!(result.is_ok(), "Second backup should succeed");

        // Check if both backups exist
        let backup_dirs: Vec<_> = fs::read_dir(&backup_dir)
            .unwrap()
            .map(|entry| entry.unwrap())
            .collect();

        assert_eq!(backup_dirs.len(), 2, "Should have two backup directories");

        // Verify different timestamps
        let first_backup = backup_dirs[0].file_name().into_string().unwrap();
        let second_backup = backup_dirs[1].file_name().into_string().unwrap();
        assert_ne!(
            first_backup, second_backup,
            "Backup directories should have different timestamps"
        );
    }

    #[test]
    fn test_create_new_backup_permissions() {
        // Skip this test on non-Unix platforms
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            // Arrange
            let temp_dir = TempDir::new().expect("Failed to create temp dir");
            let backup_dir = temp_dir.path().join("backups");

            fs::create_dir(&backup_dir).unwrap();
            fs::set_permissions(&backup_dir, fs::Permissions::from_mode(0o444)).unwrap();

            let backup = Backup {
                backup_dir: backup_dir.clone(),
                backup_empty: true,
            };

            let (kgc_file, db_file, checksum_file) = create_test_files(&temp_dir);

            // Act
            let result = backup.create_new_backup(&kgc_file, &db_file, &checksum_file);

            // Assert
            assert!(
                result.is_err(),
                "Backup should fail with read-only directory"
            );

            // Cleanup - restore permissions to allow cleanup
            fs::set_permissions(&backup_dir, fs::Permissions::from_mode(0o755)).unwrap();
        }
    }
}
