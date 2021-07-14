use bevy::prelude::*;
use bevy_inspector_egui::egui::Ui;

use crate::{DevTool, DevToolsSettings, PerformToolAction};

pub fn display_tool(
    ui: &mut Ui,
    settings: &mut DevToolsSettings,
    tool: &DevTool,
    tool_actions: &mut EventWriter<PerformToolAction>,
) {
    ui.group(|ui| {
        if tool.perform.is_some() {
            ui.columns(2, |ui| {
                ui[0].heading(tool.label.as_ref().unwrap_or(&tool.name));
                if let Some(icon) = tool.perform_icon.as_ref() {
                    if ui[1].button(icon).clicked() {
                        tool_actions.send(PerformToolAction(tool.clone()));
                    }
                } else if ui[1].button("Perform").clicked() {
                    tool_actions.send(PerformToolAction(tool.clone()));
                }
            });
        } else {
            ui.columns(1, |ui| {
                ui[0].heading(tool.label.as_ref().unwrap_or(&tool.name));
            });
        }
        (tool.render)(ui, settings);
    });
}
