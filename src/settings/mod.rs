mod value;
pub use self::value::SettingValue;

mod setting;
pub use self::setting::DevToolsSetting;

pub struct DevToolsSettings(pub Vec<DevToolsSetting>);

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
                        value: SettingValue::Group(vec![DevToolsSetting {
                            hidden: false,
                            name: "save-scene".into(),
                            label: Some("Save Scene".into()),
                            value: SettingValue::String("world.scn.ron".into()),
                        }]),
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
                        value: SettingValue::Float(0.0)
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
