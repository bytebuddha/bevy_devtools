use crate::bevy_egui::EguiContext;
use crate::egui::Ui;
use bevy::prelude::*;

mod diagnostics;
mod settings;
mod tools;
mod world;
mod location;
pub use self::location::PanelLocation;

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

impl Panels {

    pub fn get_location(&self, location: PanelLocation) -> Vec<(usize, &Panel)> {
        self.0.iter().enumerate().filter(|(_, x)|x.location == location).collect()
    }
    pub fn get_location_mut(&mut self, location: PanelLocation) -> Vec<(usize, &mut Panel)> {
        self.0.iter_mut().enumerate().filter(|(_, x)|x.location == location).collect()
    }
}

pub struct Panel {
    pub icon: String,
    pub location: PanelLocation,
    pub render: fn(&EguiContext, &mut Ui, &mut World),
}

impl Panel {
    pub fn new<S: Into<String>>(icon: S) -> Panel {
        Panel {
            icon: icon.into(),
            location: Default::default(),
            render: empty_render,
        }
    }

    pub fn render(mut self, f: fn(&EguiContext, &mut Ui, &mut World)) -> Panel {
        self.render = f;
        self
    }
}

pub fn empty_render(_: &EguiContext, _: &mut Ui, _: &mut World) {}
