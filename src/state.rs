use bevy::input::keyboard::KeyCode;

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
pub enum DevToolsLocation {
    Window,
    RightSide,
    LeftSide,
}

pub struct DevToolsState {
    pub active_tab: usize,
    pub location: DevToolsLocation,
    pub history: crate::helpers::History,
    pub toggle_key: KeyCode,
    #[cfg(feature = "puffin")]
    pub profiler_key: KeyCode,
}
