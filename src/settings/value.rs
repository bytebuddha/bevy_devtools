use crate::DevToolsSetting;

pub enum SettingValue {
    Bool(bool),
    String(String),
    Float(f32),
    Group(Vec<DevToolsSetting>),
}
