use bevy::input::keyboard::KeyCode;

pub(crate) struct DevToolsResources {
    pub history: crate::helpers::History,
    pub active_tab: crate::helpers::Tab,
    pub toggle_key: KeyCode,
}
