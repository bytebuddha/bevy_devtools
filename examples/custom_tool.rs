use bevy::prelude::*;
use bevy_devtools::{egui::Ui, DevTool, DevToolsExt, DevToolsSettings, DevToolsTab};

mod utils;

fn main() {
    App::build()
        .add_plugin(utils::ExamplePlugin)
        .add_plugin(bevy_devtools::DevToolsPlugin)
        .devtools_enabled()
        .devtools_active_tab(DevToolsTab::Tools)
        .devtools_tool(DevTool {
            label: Some("This tool is useless".into()),
            name: "blank_tool".into(),
            perform_icon: None,
            perform: None,
            render: render_blank_tool,
        })
        .devtools_tool(DevTool {
            label: Some("Hello World".into()),
            name: "hello_tool".into(),
            perform_icon: Some("Hello".into()),
            perform: Some(perform_hello),
            render: render_hello_tool,
        })
        .run()
}

fn render_blank_tool(ui: &mut Ui, _: &mut DevToolsSettings) {
    ui.label("Custom Tool");
}

fn render_hello_tool(ui: &mut Ui, _: &mut DevToolsSettings) {
    ui.label("Hello World Tool");
}

fn perform_hello(_: &mut World) {
    println!("Hello World");
}
