use super::SettingValue;

#[derive(Debug, Clone, Default)]
pub struct Setting {
    pub hidden: bool,
    pub name: String,
    pub label: Option<String>,
    pub value: SettingValue,
}

// Constructors
impl Setting {
    pub fn new<S: Into<String>>(name: S) -> Setting {
        Setting {
            name: name.into(),
            ..Default::default()
        }
    }
    pub fn new_labeled<S: Into<String>>(name: S, label: S) -> Setting {
        Setting {
            name: name.into(),
            label: Some(label.into()),
            ..Default::default()
        }
    }
}

// Setter Methods
impl Setting {
    pub fn set_hidden(mut self, b: bool) -> Setting {
        self.hidden = b;
        self
    }

    pub fn set_label<S: Into<String>>(mut self, label: S) -> Setting {
        self.label = Some(label.into());
        self
    }

    pub fn set_value(mut self, value: SettingValue) -> Setting {
        self.value = value;
        self
    }

    pub fn set_value_group(mut self, value: Vec<Setting>) -> Setting {
        self.value = SettingValue::Group(value);
        self
    }

    pub fn set_value_string<S: Into<String>>(mut self, value: S) -> Setting {
        self.value = SettingValue::String(value.into());
        self
    }

    pub fn set_value_bool(mut self, b: bool) -> Setting {
        self.value = SettingValue::Bool(b);
        self
    }

    pub fn set_value_float(mut self, b: f32) -> Setting {
        self.value = SettingValue::Float(b);
        self
    }
    pub fn set_value_integer(mut self, b: i32) -> Setting {
        self.value = SettingValue::Integer(b);
        self
    }
}

// Getter Methods
impl Setting {
    pub fn get_label(&self) -> &str {
        self.label.as_ref().unwrap_or(&self.name)
    }

    pub fn get_named_child(&self, name: &str) -> Option<&Setting> {
        if let SettingValue::Group(children) = &self.value {
            for child in children.iter() {
                if child.name == name {
                    return Some(child);
                }
            }
        }
        None
    }

    pub fn get_named_child_mut(&mut self, name: &str) -> Option<&mut Setting> {
        if let SettingValue::Group(children) = &mut self.value {
            for child in children.iter_mut() {
                if child.name == name {
                    return Some(child);
                }
            }
        }
        None
    }

    pub fn get_group(&self) -> Option<&[Setting]> {
        match &self.value {
            SettingValue::Group(children) => Some(children),
            _ => None,
        }
    }

    pub fn get_group_mut(&mut self) -> Option<&mut [Setting]> {
        match &mut self.value {
            SettingValue::Group(children) => Some(children),
            _ => None,
        }
    }
}
