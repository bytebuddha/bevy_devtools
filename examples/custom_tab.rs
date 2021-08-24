use bevy::prelude::*;
use bevy_devtools::{bevy_egui::EguiContext, egui::Ui, DevToolsExt, DevToolsTab};

mod utils;

fn main() {
    App::build()
        .add_plugin(utils::ExamplePlugin)
        .add_plugin(bevy_devtools::DevToolsPlugin)
        .devtools_enabled()
        .devtools_active_tab(4)
        .devtools_tab(DevToolsTab::new("☺").render(draw_tab))
        .run()
}

fn draw_tab(_: &EguiContext, ui: &mut Ui, _: &mut World) {
    ui.label("Hello, World!");
}
