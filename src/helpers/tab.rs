#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
pub enum DevToolsTab {
    Diagnostics,
    World,
    Tools,
    Settings,
}

impl DevToolsTab {
    pub fn icon(&self) -> &'static str {
        match self {
            DevToolsTab::Diagnostics => "🔍",
            DevToolsTab::World => "🗺",
            DevToolsTab::Tools => "🛠",
            DevToolsTab::Settings => "⚙",
        }
    }
}

impl Default for DevToolsTab {
    fn default() -> DevToolsTab {
        DevToolsTab::World
    }
}
