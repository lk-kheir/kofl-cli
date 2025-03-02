use std::collections::HashMap;
use crate::setting::core::Setting;
use crate::setting::setting_value::SettingValue;
use crate::setting::setting_key::SettingKey;
use crate::setting::session_duration::SessionDuration;

pub struct SettingsRegistry {
    settings: HashMap<SettingKey, Box<dyn Setting>>,
}

impl SettingsRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            settings: HashMap::new(),
        };
        
        // Register default settings
        registry.register(Box::new(SessionDuration::new()));
        
        registry
    }
    
    pub fn register(&mut self, setting: Box<dyn Setting>) {
        self.settings.insert(setting.key(), setting);
    }
    
    pub fn get(&self, key: SettingKey) -> Option<&dyn Setting> {
        match self.settings.get(&key) {
            Some(boxed_setting) => Some(boxed_setting.as_ref()),
            None => None,
        }
    }
    
    pub fn get_mut(&mut self, key: SettingKey) -> Option<&mut (dyn Setting + '_)> {
        match self.settings.get_mut(&key) {
            Some(boxed_setting) => Some(boxed_setting.as_mut()),
            None => None,
        }
    }
    
    pub fn update(&mut self, key: SettingKey, value: SettingValue) -> Result<(), String> {
        match self.settings.get_mut(&key) {
            Some(setting) => setting.update(value),
            None => Err(format!("Setting '{:?}' not found", key)),
        }
    }
    
    // Convenience getters for common settings
    pub fn session_duration(&self) -> Option<u32> {
        self.get(SettingKey::SessionDuration)
            .and_then(|setting| setting.get_value().as_u32().ok())
    }
}