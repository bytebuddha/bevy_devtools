use bevy::input::keyboard::KeyCode;

pub struct DevToolsState {
    pub history: crate::helpers::History,
    pub active_tab: crate::helpers::Tab,
    pub toggle_key: KeyCode,
}
