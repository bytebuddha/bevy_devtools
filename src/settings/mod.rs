use bevy::prelude::*;

mod value;
pub use self::value::SettingValue;

mod setting;
pub use self::setting::DevToolsSetting;

#[derive(Clone)]
pub struct DevToolsSettings(pub Vec<DevToolsSetting>);

impl DevToolsSettings {
    pub fn get_key(&self, keys: &[&str]) -> Option<&DevToolsSetting> {
        let mut current_index = 0;
        for setting in &self.0 {
            if setting.name == keys[current_index] {
                let mut current_setting = setting;
                loop {
                    current_index += 1;
                    if current_index >= keys.len() {
                        return Some(&current_setting);
                    } else {
                        if let Some(setting) = current_setting.named_child(keys[current_index]) {
                            current_setting = setting;
                            continue;
                        } else {
                            warn!("Setting has no child Named: {}", keys[current_index]);
                        }
                    }
                }
            }
        }
        None
    }

    pub fn get_key_mut(&mut self, keys: &[&str]) -> Option<&mut DevToolsSetting> {
        for item in self.0.iter_mut() {
            if &item.name == keys[0] {
                if keys.len() == 1 {
                    return Some(item);
                } else {
                    let mut current: Option<&mut DevToolsSetting> = Some(item);
                    for key in &keys[1..] {
                        if let Some(setting) = current {
                            current = setting.named_child_mut(key);
                        } else {
                            unreachable!()
                        }
                    }
                    return current;
                }
            }
        }
        None
    }
}

impl Default for DevToolsSettings {
    fn default() -> DevToolsSettings {
        DevToolsSettings(vec![
            DevToolsSetting {
                hidden: false,
                name: "devtools".into(),
                label: Some("DevTools".into()),
                value: SettingValue::Group(vec![
                    DevToolsSetting::labeled("enabled", "Enabled"),
                    DevToolsSetting::labeled("always-visible", "Always Visible"),
                    DevToolsSetting {
                        hidden: false,
                        name: "settings".into(),
                        label: Some("Settings".into()),
                        value: SettingValue::Group(vec![
                            DevToolsSetting::labeled("show-hidden", "Show hidden")
                        ]),
                    },
                    DevToolsSetting {
                        hidden: false,
                        name: "gui".into(),
                        label: Some("Gui".into()),
                        value: SettingValue::Group(vec![
                            DevToolsSetting::labeled("widgets-hover", "Show widgets on hover"),
                            DevToolsSetting::labeled("widgets-taller", "Show widgets that make their parent taller."),
                            DevToolsSetting::labeled("widgets-wider", "Show widgets that make their parent wider."),
                            DevToolsSetting::labeled("show-resize", "Show Resize")
                        ]),
                    },
                    DevToolsSetting {
                        hidden: false,
                        name: "world".into(),
                        label: Some("World".into()),
                        value: SettingValue::Group(vec![
                            DevToolsSetting::labeled("despawnable", "Despawnable Entities"),
                            DevToolsSetting::labeled("sort", "Sort Components")
                        ]),
                    },
                    DevToolsSetting {
                        hidden: true,
                        name: "tools".into(),
                        label: Some("Tools".into()),
                        value: SettingValue::Group(vec![
                            DevToolsSetting {
                                hidden: false,
                                name: "save-scene".into(),
                                label: Some("Save Scene".into()),
                                value: SettingValue::String("world.scn.ron".into()),
                            },
                            #[cfg(feature = "debugdump")]
                            DevToolsSetting {
                                hidden: false,
                                name: "dot-render-graph".into(),
                                label: Some("Render Graph".into()),
                                value: SettingValue::String("render-graph.dot".into()),
                            },
                        ]),
                    },
                ]),
            },
            #[cfg(feature = "puffin")]
            DevToolsSetting {
                hidden: false,
                name: "puffin".into(),
                label: Some("Profiler".into()),
                value: SettingValue::Group(vec![
                    DevToolsSetting::labeled("enabled", "Enabled"),
                ])
            },
            #[cfg(feature = "rapier3d")]
            DevToolsSetting {
                hidden: false,
                name: "rapier".into(),
                label: Some("Rapier".into()),
                value: SettingValue::Group(vec![
                    DevToolsSetting {
                        hidden: false,
                        name: "query_pipeline_active".into(),
                        label: Some("Query pipeline active".into()),
                        value: SettingValue::Bool(true),
                    },
                    DevToolsSetting {
                        hidden: false,
                        name: "physics_pipeline_active".into(),
                        label: Some("Physics pipeline active".into()),
                        value: SettingValue::Bool(true),
                    },
                ]),
            },
        ])
    }
}
