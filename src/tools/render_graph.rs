use bevy::prelude::*;
use bevy::render::render_graph::RenderGraph;
use bevy_inspector_egui::egui::Ui;

use std::fs::File;
use std::io::Write;

use crate::{DevTool, SettingValue};

pub fn tool() -> DevTool {
    DevTool {
        name: "dot-render-graph".into(),
        label: Some("Render Graph".into()),
        perform: Some(perform),
        perform_icon: Some("💾".into()),
        render,
    }
}

pub fn render(ui: &mut Ui, settings: &mut crate::DevToolsSettings) {
    #[cfg(feature = "puffin")]
    puffin_profiler::profile_function!();
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
    #[cfg(feature = "puffin")]
    puffin_profiler::profile_function!();
    let settings = ignore_none_error!(
        world.get_resource::<crate::DevToolsSettings>(),
        "Failed to get DevToolsSettings resource"
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
