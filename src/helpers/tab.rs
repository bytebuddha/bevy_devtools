#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
pub enum Tab {
    Diagnostics,
    World,
    Tools,
    Settings,
}

impl Tab {
    pub fn icon(&self) -> &'static str {
        match self {
            Tab::Diagnostics => "🔍",
            Tab::World => "🗺",
            Tab::Tools => "🛠",
            Tab::Settings => "⚙",
        }
    }
}

impl Default for Tab {
    fn default() -> Tab {
        Tab::Diagnostics
    }
}
