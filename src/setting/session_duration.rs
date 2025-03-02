use crate::setting::core::Setting;
use crate::setting::setting_value::SettingValue;
use crate::setting::setting_key::SettingKey;

pub struct SessionDuration {
    value: u32,  // minutes
}

impl SessionDuration {
    pub fn new() -> Self {
        Self { value: 30 } // Default value
    }
    
    pub fn get(&self) -> u32 {
        self.value
    }
}

impl Setting for SessionDuration {
    fn key(&self) -> SettingKey { 
        SettingKey::SessionDuration
    }
    
    fn default(&self) -> SettingValue { 
        SettingValue::UnsignedInteger(30) 
    }
    
    fn validate(&self, value: &SettingValue) -> Result<(), String> {
        match value.as_u32() {
            Ok(minutes) => {
                if minutes < 1 || minutes > 1440 {  // Max 24 hours
                    Err("Session duration must be between 1 and 1440 minutes (24 hours)".to_string())
                } else {
                    Ok(())
                }
            },
            Err(e) => Err(e),
        }
    }

    fn update(&mut self, value: SettingValue) -> Result<(), String> {
        self.validate(&value)?;
        self.value = value.as_u32()?;
        Ok(())
    }
    
    fn get_value(&self) -> SettingValue {
        SettingValue::UnsignedInteger(self.value as u64)
    }
}