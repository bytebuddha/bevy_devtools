mod value;
pub use self::value::SettingValue;

mod setting;
pub use self::setting::DevToolsSetting;

#[derive(Clone)]
pub struct DevToolsSettings(pub Vec<DevToolsSetting>);

impl DevToolsSettings {
    pub fn named(&self, name: &str) -> Option<&DevToolsSetting> {
        for setting in &self.0 {
            if setting.name == name {
                return Some(setting);
            }
        }
        None
    }

    pub fn named_mut(&mut self, name: &str) -> Option<&mut DevToolsSetting> {
        for setting in &mut self.0 {
            if setting.name == name {
                return Some(setting);
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
                    DevToolsSetting {
                        hidden: false,
                        name: "enabled".into(),
                        label: Some("Enabled".into()),
                        value: SettingValue::Bool(false),
                    },
                    DevToolsSetting {
                        hidden: false,
                        name: "always-visible".into(),
                        label: Some("Always Visible".into()),
                        value: SettingValue::Bool(false),
                    },
                    DevToolsSetting {
                        hidden: false,
                        name: "settings".into(),
                        label: Some("Settings".into()),
                        value: SettingValue::Group(vec![DevToolsSetting {
                            hidden: false,
                            name: "show-hidden".into(),
                            label: Some("Show Hidden".into()),
                            value: SettingValue::Bool(false),
                        }]),
                    },
                    DevToolsSetting {
                        hidden: false,
                        name: "gui".into(),
                        label: Some("Gui".into()),
                        value: SettingValue::Group(vec![
                            DevToolsSetting {
                                hidden: false,
                                name: "widgets-hover".into(),
                                label: Some("Show widgets on hover".into()),
                                value: SettingValue::Bool(false),
                            },
                            DevToolsSetting {
                                hidden: false,
                                name: "widgets-taller".into(),
                                label: Some("Show widgets that make their parent taller.".into()),
                                value: SettingValue::Bool(false),
                            },
                            DevToolsSetting {
                                hidden: false,
                                name: "widgets-wider".into(),
                                label: Some("Show widgets that make their parent wider.".into()),
                                value: SettingValue::Bool(false),
                            },
                            DevToolsSetting {
                                hidden: false,
                                name: "show-resize".into(),
                                label: Some("Show Resize.".into()),
                                value: SettingValue::Bool(false),
                            },
                        ]),
                    },
                    DevToolsSetting {
                        hidden: false,
                        name: "world".into(),
                        label: Some("World".into()),
                        value: SettingValue::Group(vec![
                            DevToolsSetting {
                                hidden: false,
                                name: "despawnable".into(),
                                label: Some("Despawnable Entities".into()),
                                value: SettingValue::Bool(false),
                            },
                            DevToolsSetting {
                                hidden: false,
                                name: "sort".into(),
                                label: Some("Sort Components.".into()),
                                value: SettingValue::Bool(false),
                            },
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
                            #[cfg(feature = "bevy_mod_debugdump")]
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
            #[cfg(feature = "rapier")]
            DevToolsSetting {
                hidden: false,
                name: "rapier".into(),
                label: Some("Rapier".into()),
                value: SettingValue::Group(vec![
                    DevToolsSetting {
                        hidden: false,
                        name: "scale".into(),
                        label: Some("Scale".into()),
                        value: SettingValue::Float(0.0),
                    },
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
