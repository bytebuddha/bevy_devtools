use super::SettingValue;

pub struct DevToolsSetting {
    pub hidden: bool,
    pub name: String,
    pub label: Option<String>,
    pub value: SettingValue
}

impl DevToolsSetting {
    pub fn children(&self) -> Option<&[DevToolsSetting]> {
        match &self.value {
            SettingValue::Bool(_) => None,
            SettingValue::String(_) => None,
            SettingValue::Group(children) => Some(&children)
        }
    }

    pub fn children_mut(&mut self) -> Option<&mut [DevToolsSetting]> {
        match &mut self.value {
            SettingValue::Bool(_) => None,
            SettingValue::String(_) => None,
            SettingValue::Group(children) => Some(children)
        }
    }
}
