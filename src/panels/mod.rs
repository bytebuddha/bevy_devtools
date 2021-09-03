use crate::bevy_egui::EguiContext;
use crate::egui::Ui;
use bevy::prelude::*;

mod diagnostics;
mod settings;
mod tools;
mod world;

pub struct Panels(pub Vec<Panel>);

impl Default for Panels {
    fn default() -> Panels {
        Panels(vec![
            diagnostics::panel(),
            world::panel(),
            tools::panel(),
            settings::panel(),
        ])
    }
}

pub struct Panel {
    pub icon: String,
    pub render: fn(&EguiContext, &mut Ui, &mut World),
}

impl Panel {
    pub fn new<S: Into<String>>(icon: S) -> Panel {
        Panel {
            icon: icon.into(),
            render: empty_render,
        }
    }

    pub fn render(mut self, f: fn(&EguiContext, &mut Ui, &mut World)) -> Panel {
        self.render = f;
        self
    }
}

pub fn empty_render(_: &EguiContext, _: &mut Ui, _: &mut World) {}
