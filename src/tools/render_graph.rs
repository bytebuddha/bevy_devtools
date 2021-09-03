use bevy::prelude::*;
use bevy::render::render_graph::RenderGraph;
use bevy_inspector_egui::egui::Ui;

use std::fs::File;
use std::io::Write;

use crate::{Tool, SettingValue};

pub fn tool() -> Tool {
    Tool {
        name: "dot-render-graph".into(),
        label: Some("Render Graph".into()),
        perform: Some(perform),
        perform_icon: Some("💾".into()),
        render,
    }
}

pub fn render(ui: &mut Ui, settings: &mut crate::Settings) {
    if let Some(setting) = settings.get_key_mut(&["devtools", "tools", "dot-render-graph"]) {
        if let Some(value) = setting.value.as_string_mut() {
            ui.text_edit_singleline(value);
        } else {
            warn!("Setting key devtools -> tools -> dot-render-graph is not a String.");
        }
    } else {
        warn!("Setting key devtools -> tools -> dot-render-graph does not exist");
    }
}

pub fn perform(world: &mut World) {
    let settings = ignore_none_error!(
        world.get_resource::<crate::Settings>(),
        "Failed to get Settings resource"
    );
    let setting = ignore_none_error!(
        settings.get_key(&["devtools", "tools", "dot-render-graph"]),
        "Unable to find Setting key devtools -> tools -> dot-render-graph"
    );
    if let SettingValue::String(ref value) = setting.value {
        let render_graph = ignore_none_error!(
            world.get_resource::<RenderGraph>(),
            "Failed to get RenderGraph resource"
        );
        if std::path::Path::new(value).exists() {
            ignore_error!(std::fs::remove_file(&value));
        }
        let mut file = ignore_error!(File::create(&value));
        let dot = bevy_mod_debugdump::render_graph::render_graph_dot(render_graph);
        ignore_error!(file.write_all(dot.as_bytes()));
    }
}
