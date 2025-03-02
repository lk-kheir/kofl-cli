use crate::setting::setting_value::SettingValue;
use crate::setting::setting_key::SettingKey;

pub trait Setting {
    fn key(&self) -> SettingKey;
    fn default(&self) -> SettingValue;
    fn validate(&self, value: &SettingValue) -> Result<(), String>;
    fn update(&mut self, value: SettingValue) -> Result<(), String>;
    fn get_value(&self) -> SettingValue;
    
    // These can be derived from the enum
    fn name(&self) -> String {
        self.key().to_string()
    }
    
    fn description(&self) -> &'static str {
        self.key().description()
    }
}