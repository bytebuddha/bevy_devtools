use super::SettingValue;

#[derive(Debug, Clone, Default)]
pub struct DevToolsSetting {
    pub hidden: bool,
    pub name: String,
    pub label: Option<String>,
    pub value: SettingValue,
}

// Constructors
impl DevToolsSetting {
    pub fn new<S: Into<String>>(name: S) -> DevToolsSetting {
        DevToolsSetting {
            name: name.into(),
            ..Default::default()
        }
    }
    pub fn new_labeled<S: Into<String>>(name: S, label: S) -> DevToolsSetting {
        DevToolsSetting {
            name: name.into(),
            label: Some(label.into()),
            ..Default::default()
        }
    }
}

// Setter Methods
impl DevToolsSetting {

    pub fn set_hidden(mut self, b: bool) -> DevToolsSetting {
        self.hidden = b;
        self
    }

    pub fn set_value(mut self, value: SettingValue) -> DevToolsSetting {
        self.value = value;
        self
    }

    pub fn set_value_group(mut self, value: Vec<DevToolsSetting>) -> DevToolsSetting {
        self.value = SettingValue::Group(value);
        self
    }

    pub fn set_value_string<S: Into<String>>(mut self, value: S) -> DevToolsSetting {
        self.value = SettingValue::String(value.into());
        self
    }

    pub fn set_value_bool(mut self, b: bool) -> DevToolsSetting {
        self.value = SettingValue::Bool(b);
        self
    }

    pub fn set_value_float(mut self, b: f32) -> DevToolsSetting {
        self.value = SettingValue::Float(b);
        self
    }
}

// Getter Methods
impl DevToolsSetting {

    pub fn get_label(&self) -> &str {
        self.label.as_ref().unwrap_or(&self.name)
    }

    pub fn get_named_child(&self, name: &str) -> Option<&DevToolsSetting> {
        if let SettingValue::Group(children) = &self.value {
            for child in children.iter() {
                if child.name == name {
                    return Some(child);
                }
            }
        }
        None
    }

    pub fn get_named_child_mut(&mut self, name: &str) -> Option<&mut DevToolsSetting> {
        if let SettingValue::Group(children) = &mut self.value {
            for child in children.iter_mut() {
                if child.name == name {
                    return Some(child);
                }
            }
        }
        None
    }

    pub fn get_group(&self) -> Option<&[DevToolsSetting]> {
        match &self.value {
            SettingValue::Bool(_) => None,
            SettingValue::String(_) => None,
            SettingValue::Float(_) => None,
            SettingValue::Group(children) => Some(children),
        }
    }

    pub fn get_group_mut(&mut self) -> Option<&mut [DevToolsSetting]> {
        match &mut self.value {
            SettingValue::Bool(_) => None,
            SettingValue::String(_) => None,
            SettingValue::Float(_) => None,
            SettingValue::Group(children) => Some(children),
        }
    }
}
