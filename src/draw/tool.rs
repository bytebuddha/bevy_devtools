use bevy::prelude::*;
use bevy::app::Events;
use bevy_inspector_egui::egui::Ui;

use crate::{DevTool, DevToolsSettings, PerformToolAction, DevToolsTools};

pub fn handle_tools(ui: &mut Ui, tools: &DevToolsTools, settings: &mut DevToolsSettings, world: &mut World) {
    let mut tool_actions = world.get_resource_mut::<Events<PerformToolAction>>().unwrap();
    for tool in tools.0.iter() {
        display_tool(ui, settings, tool, &mut tool_actions);
    }
}

pub fn display_tool(
    ui: &mut Ui,
    settings: &mut DevToolsSettings,
    tool: &DevTool,
    tool_actions: &mut Events<PerformToolAction>,
) {
    ui.group(|ui| {
        if tool.perform.is_some() {
            ui.columns(2, |ui| {
                ui[0].heading(tool.label.as_ref().unwrap_or(&tool.name));
                if let Some(icon) = tool.perform_icon.as_ref() {
                    if ui[1].small_button(icon).clicked() {
                        tool_actions.send(PerformToolAction(tool.clone()));
                    }
                } else if ui[1].small_button("Perform").clicked() {
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
