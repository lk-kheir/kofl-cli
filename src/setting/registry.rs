// src/setting/registry.rs

use std::collections::HashMap;
use crate::setting::core::Setting;
use crate::setting::session_duration::SessionDuration;

struct SettingsRegistry {
    settings: HashMap<String, Box<dyn Setting>>,
}


impl SettingsRegistry {
    pub fn new() -> Self {
        let mut settings = HashMap::new();
        
        // Initialize with default settings
        if let Ok(session_duration) = SessionDuration::new(SessionDuration::default()) {
            settings.insert(
                session_duration.name().to_string(), 
                Box::new(session_duration)
            );
        }
        
        Self { settings }
    }
    
    // Get immutable reference to a setting
    pub fn get(&self, name: &str) -> Option<&dyn Setting> {
        self.settings.get(name).map(|boxed| boxed.as_ref())
    }
    
    // Get mutable reference to a setting
    pub fn get_mut(&mut self, name: &str) -> Option<&mut dyn Setting> {
        self.settings.get_mut(name).map(|boxed| boxed.as_mut())
    }
}




