use crate::DevToolsSetting;

#[derive(Debug, Clone)]
pub enum SettingValue {
    Bool(bool),
    String(String),
    Float(f32),
    Group(Vec<DevToolsSetting>),
}

impl SettingValue {
    pub fn as_bool(&self) -> Option<bool> {
        if let SettingValue::Bool(value) = self {
            Some(*value)
        } else {
            None
        }
    }
    pub fn as_bool_mut(&mut self) -> Option<&mut bool> {
        if let SettingValue::Bool(ref mut value) = self {
            Some(value)
        } else {
            None
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        if let SettingValue::String(value) = self {
            Some(value)
        } else {
            None
        }
    }
    pub fn as_string_mut(&mut self) -> Option<&mut String> {
        if let SettingValue::String(ref mut value) = self {
            Some(value)
        } else {
            None
        }
    }
    pub fn as_float(&self) -> Option<f32> {
        if let SettingValue::Float(value) = self {
            Some(*value)
        } else {
            None
        }
    }
    pub fn as_float_mut(&mut self) -> Option<&mut f32> {
        if let SettingValue::Float(ref mut value) = self {
            Some(value)
        } else {
            None
        }
    }
}
