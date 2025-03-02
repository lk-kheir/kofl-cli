use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingKey {
    SessionDuration,
    FailedLoginLockoutDuration,
    MaxLoginAttempts,
    PasswordMinLength,
    RequireSpecialChars,
    // Add more settings as needed
}

impl Hash for SettingKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // We can use discriminant value as hash
        std::mem::discriminant(self).hash(state);
    }
}

impl Display for SettingKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SettingKey::SessionDuration => write!(f, "session_duration"),
            SettingKey::FailedLoginLockoutDuration => write!(f, "failed_login_lockout_duration"),
            SettingKey::MaxLoginAttempts => write!(f, "max_login_attempts"),
            SettingKey::PasswordMinLength => write!(f, "password_min_length"),
            SettingKey::RequireSpecialChars => write!(f, "require_special_chars"),
        }
    }
}

impl SettingKey {
    pub fn description(&self) -> &'static str {
        match self {
            SettingKey::SessionDuration => "How long a session remains active (in minutes)",
            SettingKey::FailedLoginLockoutDuration => "Duration of account lockout after failed login attempts (in minutes)",
            SettingKey::MaxLoginAttempts => "Maximum number of failed login attempts before lockout",
            SettingKey::PasswordMinLength => "Minimum length required for passwords",
            SettingKey::RequireSpecialChars => "Whether passwords must contain special characters",
        }
    }
}