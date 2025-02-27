pub struct CONS;

impl CONS {
    pub const MIN_PASSWORD_LENGTH: usize = 8;
    pub const PASSWORD_UPPERCASE_REQ: &str = ".*[A-Z].*";
    pub const PASSWORD_LOWERCASE_REQ: &str = ".*[a-z].*";
    pub const PASSWORD_DIGIT_REQ: &str = ".*[0-9].*";
    pub const PASSWORD_SPECIAL_CHAR_REQ: &str = ".*[!@#\\$%\\^&\\*].*";
}