use crate::cli::Command;
use crate::context::Context;
use crate::settings::Setting;
use log::{debug, info, error};

pub struct SettingsCmd {
    action: SettingsAction,
}

pub enum SettingsAction {
    List,
    Get(String),
    Set(String, String),
}

impl SettingsCmd {
    pub fn new_list() -> Self {
        Self { action: SettingsAction::List }
    }
    
    pub fn new_get(name: String) -> Self {
        Self { action: SettingsAction::Get(name) }
    }
    
    pub fn new_set(name: String, value: String) -> Self {
        Self { action: SettingsAction::Set(name, value) }
    }
}

impl Command for SettingsCmd {
    fn validate(&self, context: &Context) -> bool {
        match &self.action {
            SettingsAction::List => true,
            SettingsAction::Get(name) => {
                
                true
            },
            SettingsAction::Set(name, value) => {
                
                true
            }
        }
    }

    fn execute(&self, context: &Context) -> bool {
        match &self.action {
            SettingsAction::List => {
                // Get all settings
                true
            },
            SettingsAction::Get(name) => {
                // Get specific setting
                true
            },
            SettingsAction::Set(name, value) => {
                // Set specific setting
                true
            }
        }
    }

    fn display(&self) {
        debug!("Settings Command");
        ()
    }
}