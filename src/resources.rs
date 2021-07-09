#[derive(Default)]
pub struct SaveWorldRequest(pub Option<String>);

pub struct DevToolsResources {
    pub history: super::History,
    pub save_scene_name: String,
    pub active_tab: usize,
    pub always_visible: bool,
    pub egui: EguiSettings,
}

#[derive(Default)]
pub struct EguiSettings {
    pub show_widgets_on_hover: bool,
    pub expand_width: bool,
    pub expand_height: bool,
    pub show_resize: bool,
}

impl Default for DevToolsResources {
    fn default() -> DevToolsResources {
        DevToolsResources {
            history: Default::default(),
            active_tab: 0,
            save_scene_name: "world.scn.ron".into(),
            always_visible: false,
            egui: EguiSettings::default(),
        }
    }
}
