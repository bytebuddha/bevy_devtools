#[derive(PartialEq, Clone, Copy)]
pub enum PanelLocation {
    Widget,
    Window
}

impl Default for PanelLocation {
    fn default() -> PanelLocation {
        PanelLocation::Widget
    }
}
