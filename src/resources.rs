use bevy::input::keyboard::KeyCode;

#[derive(Default)]
pub struct SaveWorldRequest(pub Option<String>);

pub struct DevToolsResources {
    pub history: crate::helpers::History,
    pub save_scene_name: String,
    pub active_tab: crate::helpers::Tab,
    pub always_visible: bool,
    pub enabled: bool,
    pub toggle_key: KeyCode
}

impl Default for DevToolsResources {
    fn default() -> DevToolsResources {
        DevToolsResources {
            history: Default::default(),
            active_tab: Default::default(),
            save_scene_name: "world.scn.ron".into(),
            always_visible: false,
            enabled: false,
            toggle_key: KeyCode::F11
        }
    }
}
