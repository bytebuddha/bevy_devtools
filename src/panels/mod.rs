use crate::bevy_egui::EguiContext;
use crate::egui::Ui;
use bevy::prelude::*;

mod diagnostics;
mod settings;
mod tools;
mod world;

pub struct DevToolsPanels(pub Vec<DevToolsPanel>);

impl Default for DevToolsPanels {
    fn default() -> DevToolsPanels {
        DevToolsPanels(vec![
            diagnostics::panel(),
            world::panel(),
            tools::panel(),
            settings::panel(),
        ])
    }
}

pub struct DevToolsPanel {
    pub icon: String,
    pub render: fn(&EguiContext, &mut Ui, &mut World),
}

impl DevToolsPanel {
    pub fn new<S: Into<String>>(icon: S) -> DevToolsPanel {
        DevToolsPanel {
            icon: icon.into(),
            render: empty_render,
        }
    }

    pub fn render(mut self, f: fn(&EguiContext, &mut Ui, &mut World)) -> DevToolsPanel {
        self.render = f;
        self
    }
}

pub fn empty_render(_: &EguiContext, _: &mut Ui, _: &mut World) {}
