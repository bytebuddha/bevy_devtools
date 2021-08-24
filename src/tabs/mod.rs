use crate::bevy_egui::EguiContext;
use crate::egui::Ui;
use bevy::prelude::*;

mod diagnostics;
mod settings;
mod tools;
mod world;

pub struct DevToolsTabs(pub Vec<DevToolsTab>);

impl Default for DevToolsTabs {
    fn default() -> DevToolsTabs {
        DevToolsTabs(vec![
            diagnostics::tab(),
            world::tab(),
            tools::tab(),
            settings::tab(),
        ])
    }
}

pub struct DevToolsTab {
    pub icon: String,
    pub render: fn(&EguiContext, &mut Ui, &mut World),
}

impl DevToolsTab {
    pub fn new<S: Into<String>>(icon: S) -> DevToolsTab {
        DevToolsTab {
            icon: icon.into(),
            render: empty_render,
        }
    }

    pub fn render(mut self, f: fn(&EguiContext, &mut Ui, &mut World)) -> DevToolsTab {
        self.render = f;
        self
    }
}

pub fn empty_render(_: &EguiContext, _: &mut Ui, _: &mut World) {}
