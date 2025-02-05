use std::env::home_dir;
use std::fmt::Debug;
use crate::utils::Utils::{check_existing_session_config, get_home_dir};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;
use toml;
use chrono::{DateTime, TimeZone, Utc};
use crate::cli::Command;
use crate::errors::{ErrorExecution, ErrorValidation};
use crate::context::Context;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use sha2::{Sha256, Digest};
use aes::cipher::KeyIvInit;
use ctr::Ctr32BE;

type Aes256Ctr = Ctr32BE<aes::Aes256>;

pub enum SessionStatus {
    Active,
    Expired,
    Invalid,
    RequiresReauth
}

pub enum SessionError {
    ExpiredSession,
    InvalidUser,
    FilePermissionError,
    SessionCreationError,
    SessionFileMissingError,
    FailedLoadingError,
    AuthenticationRequired
}

#[derive(Serialize, Deserialize)]
pub struct Session {
    session_path: PathBuf,
    session_id: String,
    user_login: String,
    created_at: DateTime<Utc>,
    expires_at: DateTime<Utc>,
    last_activity: DateTime<Utc>,
    is_active: bool
}

impl Session {
    pub fn new(user_login: String) -> Self {
        let now = Utc::now();
        let home_dir = get_home_dir().expect("Home directory not found");
        Session {
            session_path: home_dir.join(".kofl_session"),
            session_id: thread_rng()
                .sample_iter(&Alphanumeric)
                .take(32)
                .map(char::from)
                .collect(),
            user_login,
            created_at: now,
            expires_at: now + chrono::Duration::seconds(10),
            last_activity: now,
            is_active: true
        }
    }

    pub fn load(&mut self) -> Result<(), SessionError> {
        if check_existing_session_config() {
            match self.read_config_from_toml_file() {
                Ok(config) => {
                    *self = config; // mutating the self with Session  serialized
                    if self.check_if_expired() {
                        return Err(SessionError::ExpiredSession);
                    }
                    self.last_activity = Utc::now();
                    self.update();
                    Ok(())
                }
                Err(e) => {
                    Err(SessionError::FailedLoadingError)
                }
            }
        } else {
            return Err(SessionError::SessionFileMissingError);
        }
    }

    pub fn update(&self) {
        self.write_session_config_to_toml_file();
    }

    fn serialize_to_toml(&self) -> String {
        toml::to_string(self).expect("could not serialize struct into toml string")
    }

    pub fn write_session_config_to_toml_file(&self) {
        let toml_str = self.serialize_to_toml();
        // println!("toml str =\n{}", toml_str);
        let config_pth = &self.session_path;
        fs::write(config_pth, toml_str).expect("could not create toml file for session config");
    }

    pub fn read_config_from_toml_file(&self) -> Result<Session, Box<dyn std::error::Error>> {
        let session_pth = &self.session_path;
        let toml_str = fs::read_to_string(session_pth)?;
        let config = toml::from_str(&toml_str)?;
        Ok(config)
    }

    pub fn check_if_expired(&self) -> bool {
        self.expires_at < Utc::now()
    }
}

impl Debug for Session {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\nSession Info:\n\
             ├─ Session file Path: {}\n\
             ├─ ID: {}\n\
             ├─ User: {}\n\
             ├─ Created: {}\n\
             ├─ Expires: {}\n\
             ├─ Last Activity: {}\n\
             └─ Active: {}\n",
            self.session_path.display(),
            self.session_id,
            self.user_login,
            self.created_at.format("%Y-%m-%d %H:%M:%S"),
            self.expires_at.format("%Y-%m-%d %H:%M:%S"),
            self.last_activity.format("%Y-%m-%d %H:%M:%S"),
            if self.is_active { "Yes" } else { "No" }
        )
    }
}



/*

    1. Simple Session Creation & Storage
   - Session struct with basic fields
   - Save as plain JSON file
   - Basic timestamp validation
   - Store in ~/.kofl/session

2. Basic Session Validation
   - Check if expired
   - Check if exists
   - No encryption yet


   1. Simple Login Flow
   - Check if config exists
   - Basic password verification
   - Create session
   - Save session

2. Basic Error Handling
   - Session not found
   - Session expired
   - Config missing


   1. Command Prerequisites
   - Check session exists
   - Validate expiration
   - Update last activity

2. Session Management
   - Load session before commands
   - Save session after commands

 */

/*
    Implement in this order:
1. new() - Create session with:
   - Random session_id (using your existing Alphanumeric)
   - Current UTC time
   - User from context
   - 15-minute expiration

2. save() - Secure storage:
   - Encrypt session data
   - Generate HMAC
   - Set correct file permissions (600)
   - Write to ~/.kofl/session

3. load() - Secure retrieval:
   - Check file permissions
   - Verify HMAC
   - Decrypt data
   - Validate timestamps
   - Check user matches

4. is_valid() - Multiple checks:
   - Session exists
   - Not expired
   - User matches
   - HMAC valid
   - File permissions correct


    LogInCmd should:
1. Validate:
   - Check if already logged in
   - Verify master password exists
   - Check config is initialized

2. Execute:
   - Verify master password
   - Create new Session
   - Save session securely
   - Update last activity



   Add these security measures:
- HMAC generation/verification
- Session data encryption/decryption
- Timestamp validation
- File permission management
- User validation


User Flow:
1. kofl login
2. Verify master password
3. Create new session
4. Encrypt & save session
5. Return success

Command Flow:
1. Check session exists
2. Validate session
3. Update last activity
4. Execute command
5. Save updated session


Add specific errors for:
- Session expired
- Session tampered
- Invalid permissions
- Authentication failed
- Session creation failed


 */