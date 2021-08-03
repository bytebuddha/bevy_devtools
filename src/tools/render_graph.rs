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
    if let Some(setting) = settings.named_mut("devtools") {
        if let Some(child) = setting.named_child_mut("tools") {
            if let Some(child) = child.named_child_mut("dot-render-graph") {
                if let Some(value) = child.value.as_string_mut() {
                    ui.text_edit_singleline(value);
                }
            }
        }
    }
}

pub fn perform(world: &mut World) {
    let settings = world.get_resource::<crate::DevToolsSettings>().unwrap();
    if let Some(setting) = settings.named("devtools") {
        if let Some(child) = setting.named_child("tools") {
            if let Some(child) = child.named_child("dot-render-graph") {
                if let SettingValue::String(value) = &child.value {
                    let render_graph = world.get_resource::<RenderGraph>().unwrap();
                    let mut file = File::create(&value).unwrap();
                    let dot = bevy_mod_debugdump::render_graph::render_graph_dot(render_graph);
                    file.write_all(dot.as_bytes()).unwrap();
                }
            }
        }
    }
}
