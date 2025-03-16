use crate::context::Context;
use std::collections::HashMap;
use log::{debug, warn};

#[derive(Debug, Clone, Copy)]
pub enum Setting {
    SessionDuration,
    ClipboardTimeout,
    DefaultTimeout,
    EncryptionIterations,
    // Add other settings as needed
}

impl Setting {
    fn key(&self) -> &'static str {
        match self {
            Setting::SessionDuration => "session_duration",
            Setting::ClipboardTimeout => "clipboard_timeout", 
            Setting::DefaultTimeout => "default_timeout",
            Setting::EncryptionIterations => "encryption_iterations",
        }
    }
    
    fn default_value(&self) -> &'static str {
        match self {
            Setting::SessionDuration => "30",
            Setting::ClipboardTimeout => "10",
            Setting::DefaultTimeout => "120",
            Setting::EncryptionIterations => "100000",
        }
    }
    
    fn description(&self) -> &'static str {
        match self {
            Setting::SessionDuration => "Duration of session in minutes",
            Setting::ClipboardTimeout => "Time in seconds before clipboard is cleared",
            Setting::DefaultTimeout => "Default timeout for entries in seconds",
            Setting::EncryptionIterations => "Number of iterations for key derivation",
        }
    }
    
    fn validate(&self, value: &str) -> Result<(), String> {
        match self {
            Setting::SessionDuration => {
                match value.parse::<u32>() {
                    Ok(minutes) if minutes >= 1 && minutes <= 1440 => Ok(()),
                    Ok(_) => Err("Session duration must be between 1 and 1440 minutes".to_string()),
                    Err(_) => Err("Value must be a positive integer".to_string()),
                }
            },
            // Add validation for other settings
            _ => Ok(()),
        }
    }
}

pub struct SettingsManager {
    // We'll use a cache to avoid frequent database access
    cache: HashMap<String, String>,
}

impl SettingsManager {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    // Initialize default settings in the database
    pub fn initialize_defaults(&self, context: &Context) -> Result<(), String> {
        let settings = [
            Setting::SessionDuration,
            Setting::ClipboardTimeout,
            Setting::DefaultTimeout,
            Setting::EncryptionIterations,
        ];

        for setting in settings {
            if let Ok(None) = context.db.get_setting_value(setting.key()) {
                if let Err(e) = context.db.set_setting_value(setting.key(), setting.default_value()) {
                    warn!("Failed to set default setting {}: {}", setting.key(), e);
                    continue;
                }
            }
        }

        Ok(())
    }

    pub fn get_string(&mut self, context: &Context, setting: Setting) -> Result<String, String> {
        let key = setting.key();
        
        // Try cache first
        if let Some(value) = self.cache.get(key) {
            return Ok(value.clone());
        }

        // Try database
        match context.db.get_setting_value(key) {
            Ok(Some(value)) => {
                // Update cache
                self.cache.insert(key.to_string(), value.clone());
                Ok(value)
            },
            Ok(None) => Err(format!("Setting '{}' not found", key)),
            Err(e) => Err(format!("Database error: {}", e)),
        }
    }

    pub fn get_u32(&mut self, context: &Context, setting: Setting) -> Result<u32, String> {
        let string_value = self.get_string(context, setting)?;
        string_value.parse::<u32>()
            .map_err(|_| format!("Setting '{}' is not a valid u32", setting.key()))
    }

    pub fn set(&mut self, context: &Context, setting: Setting, value: &str) -> Result<(), String> {
        // Validate the setting
        setting.validate(value)?;
        
        // Update in database
        context.db.set_setting_value(setting.key(), value)
            .map_err(|e| format!("Database error: {}", e))?;
        
        // Update cache
        self.cache.insert(setting.key().to_string(), value.to_string());
        
        Ok(())
    }

    // Convenience methods for specific settings
    pub fn get_session_duration(&mut self, context: &Context) -> Result<u32, String> {
        self.get_u32(context, Setting::SessionDuration)
    }

    pub fn set_session_duration(&mut self, context: &Context, minutes: u32) -> Result<(), String> {
        self.set(context, Setting::SessionDuration, &minutes.to_string())
    }
}