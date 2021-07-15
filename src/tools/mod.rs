use bevy::prelude::*;
use bevy_inspector_egui::egui::Ui;

mod save_world;

pub struct PerformToolAction(pub DevTool);

#[derive(Clone)]
pub struct DevToolsTools(pub Vec<DevTool>);

#[derive(Clone)]
pub struct DevTool {
    pub name: String,
    pub perform_icon: Option<String>,
    pub label: Option<String>,
    pub render: fn(&mut Ui, &mut crate::DevToolsSettings),
    pub perform: Option<fn(&mut World)>,
}

impl Default for DevToolsTools {
    fn default() -> DevToolsTools {
        DevToolsTools(vec![save_world::tool()])
    }
}
