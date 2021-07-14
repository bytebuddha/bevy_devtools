use crate::DevToolsSetting;

pub enum SettingValue {
    Bool(bool),
    String(String),
    Group(Vec<DevToolsSetting>),
}
