use bevy::prelude::*;
use bevy_devtools::{bevy_egui::EguiContext, egui::Ui, DevToolsExt, Panel};

mod utils;

fn main() {
    App::new()
        .add_plugin(utils::ExamplePlugin)
        .add_plugin(bevy_devtools::DevToolsPlugin::<utils::ExampleState>::default())
        .devtools_enabled()
        .devtools_active_panel(4)
        .devtools_panel(Panel::new("☺").render(draw_panel))
        .run()
}

fn draw_panel(_: &EguiContext, ui: &mut Ui, _: &mut World) {
    ui.label("Hello, World!");
}
