use std::fmt::Display;

/// Represents any value type that a setting can have
#[derive(Debug, Clone)]
pub enum SettingValue {
    Integer(i64),
    UnsignedInteger(u64),
    Float(f64),
    Boolean(bool),
    String(String),
    Duration(std::time::Duration),
}

impl SettingValue {
    pub fn as_u32(&self) -> Result<u32, String> {
        match self {
            SettingValue::UnsignedInteger(val) => {
                if *val <= u32::MAX as u64 {
                    Ok(*val as u32)
                } else {
                    Err(format!("Value {} exceeds maximum u32 value", val))
                }
            }
            _ => Err(format!("Cannot convert {:?} to u32", self)),
        }
    }
}

impl Display for SettingValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SettingValue::Integer(v) => write!(f, "{}", v),
            SettingValue::UnsignedInteger(v) => write!(f, "{}", v),
            SettingValue::Float(v) => write!(f, "{}", v),
            SettingValue::Boolean(v) => write!(f, "{}", v),
            SettingValue::String(v) => write!(f, "{}", v),
            SettingValue::Duration(v) => write!(f, "{}s", v.as_secs()),
        }
    }
}