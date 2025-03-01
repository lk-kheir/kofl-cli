///src/setting/session_duration.rs

struct SessionDuration {
    value: u32,  // minutes
}

impl Setting for SessionDuration {
    type Value = u32;
    
    fn name(&self) -> &str { "session_duration" }
    fn description(&self) -> &str { "How long a session remains active (in minutes)" }
    fn default() -> u32 { 30 }  // 30 minutes by default
    fn validate(&self, value: &u32) -> Result<(), String> {
        if *value < 1 || *value > 1440 {  // Max 24 hours
            Err("Session duration must be between 1 and 1440 minutes (24 hours)".to_string())
        } else {
            Ok(())
        }
    }

    fn update(&mut self, value: Self::Value) -> Result<(), String> {
        self.validate(&value)?;
        self.value = value;
        Ok(())
    }
}