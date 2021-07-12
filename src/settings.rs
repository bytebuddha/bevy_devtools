pub struct DevToolsSettings(pub Vec<DevToolsSetting>);

pub enum DevToolsSetting {
    Bool {
        name: String,
        label: Option<String>,
        value: bool
    },
    String {
        name: String,
        label: Option<String>,
        value: String
    },
    Group {
        name: String,
        label: Option<String>,
        children: Vec<DevToolsSetting>
    }
}

impl DevToolsSetting {

    pub fn name(&self) -> &str {
        match self {
            DevToolsSetting::Bool { name, .. } => name,
            DevToolsSetting::Group { name, .. } => name,
            DevToolsSetting::String { name, .. } => name
        }
    }

    pub fn children(&self) -> Option<&[DevToolsSetting]> {
        match self {
            DevToolsSetting::Bool { .. } => None,
            DevToolsSetting::String { .. } => None,
            DevToolsSetting::Group { children, .. } => Some(children)
        }
    }

    pub fn children_mut(&mut self) -> Option<&mut [DevToolsSetting]> {
        match self {
            DevToolsSetting::Bool { .. } => None,
            DevToolsSetting::String { .. } => None,
            DevToolsSetting::Group { children, .. } => Some(children)
        }
    }
}

impl Default for DevToolsSettings {
    fn default() -> DevToolsSettings {
        DevToolsSettings(vec![
            DevToolsSetting::Group {
                name: "devtools".into(),
                label: Some("DevTools".into()),
                children: vec![
                    DevToolsSetting::Group {
                        name: "gui".into(),
                        label: Some("Gui".into()),
                        children: vec![
                            DevToolsSetting::Bool {
                                name: "widgets-hover".into(),
                                label: Some("Show widgets on hover".into()),
                                value: false
                            },
                            DevToolsSetting::Bool {
                                name: "widgets-taller".into(),
                                label: Some("Show widgets that make their parent taller.".into()),
                                value: false
                            },
                            DevToolsSetting::Bool {
                                name: "widgets-wider".into(),
                                label: Some("Show widgets that make their parent wider.".into()),
                                value: false
                            },
                            DevToolsSetting::Bool {
                                name: "show-resize".into(),
                                label: Some("Show Resize.".into()),
                                value: false
                            }
                        ]
                    },
                    DevToolsSetting::Group {
                        name: "tools".into(),
                        label: Some("Tools".into()),
                        children: vec![
                            DevToolsSetting::String {
                                name: "save-scene".into(),
                                label: Some("Save Scene".into()),
                                value: "world.scn.ron".into()
                            }
                        ]
                    }
                ]
            }
        ])
    }
}
