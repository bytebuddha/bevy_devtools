use bevy::prelude::*;
use bevy_inspector_egui::egui::Ui;

#[cfg(feature = "debugdump")]
mod render_graph;
mod save_world;

pub struct PerformToolAction(pub Tool);

#[derive(Clone)]
pub struct Tools(pub Vec<Tool>);

#[derive(Clone)]
pub struct Tool {
    pub name: String,
    pub perform_icon: Option<String>,
    pub label: Option<String>,
    pub priority: usize,
    pub render: fn(&mut Ui, &mut crate::Settings),
    pub perform: Option<fn(&mut World)>,
}

impl Tool {
    pub fn label(&self) -> &str {
        self.label.as_ref().unwrap_or(&self.name)
    }
}

impl Default for Tools {
    fn default() -> Tools {
        Tools(vec![
            save_world::tool(),
            #[cfg(feature = "debugdump")]
            render_graph::tool(),
        ])
    }
}
