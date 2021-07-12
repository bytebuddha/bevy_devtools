#[derive(Default)]
pub struct SaveWorldRequest(pub Option<String>);

pub struct DevToolsResources {
    pub history: crate::helpers::History,
    pub save_scene_name: String,
    pub active_tab: usize,
    pub always_visible: bool,
    pub enabled: bool
}

impl Default for DevToolsResources {
    fn default() -> DevToolsResources {
        DevToolsResources {
            history: Default::default(),
            active_tab: 0,
            save_scene_name: "world.scn.ron".into(),
            always_visible: false,
            enabled: false
        }
    }
}
