use super::SettingValue;

#[derive(Debug, Clone)]
pub struct DevToolsSetting {
    pub hidden: bool,
    pub name: String,
    pub label: Option<String>,
    pub value: SettingValue,
}

impl DevToolsSetting {
    pub fn label(&self) -> &str {
        self.label.as_ref().unwrap_or(&self.name)
    }

    pub fn named_child(&self, name: &str) -> Option<&DevToolsSetting> {
        match &self.value {
            SettingValue::Group(children) => {
                for child in children.iter() {
                    if child.name == name {
                        return Some(child);
                    }
                }
            },
            _ => {}
        }
        None
    }

    pub fn named_child_mut(&mut self, name: &str) -> Option<&mut DevToolsSetting> {
        match &mut self.value {
            SettingValue::Group(children) => {
                for child in children.iter_mut() {
                    if child.name == name {
                        return Some(child);
                    }
                }
            },
            _ => {}
        }
        None
    }

    pub fn children(&self) -> Option<&[DevToolsSetting]> {
        match &self.value {
            SettingValue::Bool(_) => None,
            SettingValue::String(_) => None,
            SettingValue::Float(_) => None,
            SettingValue::Group(children) => Some(children),
        }
    }

    pub fn children_mut(&mut self) -> Option<&mut [DevToolsSetting]> {
        match &mut self.value {
            SettingValue::Bool(_) => None,
            SettingValue::String(_) => None,
            SettingValue::Float(_) => None,
            SettingValue::Group(children) => Some(children),
        }
    }
}
