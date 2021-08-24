use crate::bevy_egui::EguiContext;
use crate::egui::Ui;
use bevy::app::Events;
use bevy::prelude::*;

use crate::{DevTool, DevToolsSettings, DevToolsTools, PerformToolAction};

pub fn tab() -> super::DevToolsTab {
    super::DevToolsTab::new("🛠").render(draw)
}

pub fn draw(_: &EguiContext, ui: &mut Ui, world: &mut World) {
    #[cfg(feature = "puffin")]
    puffin_profiler::profile_function!();
    let world_ptr = world as *mut _;
    let devtools_tools = ignore_none_error!(
        world.get_resource::<DevToolsTools>(),
        "Failed to get DevToolsSettings resource"
    );
    let world: &mut World = unsafe { &mut *world_ptr };
    let mut devtools_settings = ignore_none_error!(
        world.get_resource_mut::<DevToolsSettings>(),
        "Failed to get DevToolsSettings resource"
    );
    let world: &mut World = unsafe { &mut *world_ptr };
    let mut tool_actions = ignore_none_error!(
        world.get_resource_mut::<Events<PerformToolAction>>(),
        "Failed to get Events<PerformToolAction> resources"
    );
    for tool in devtools_tools.0.iter() {
        display_tool(ui, &mut *devtools_settings, tool, &mut tool_actions);
    }
}

pub fn display_tool(
    ui: &mut Ui,
    settings: &mut DevToolsSettings,
    tool: &DevTool,
    tool_actions: &mut Events<PerformToolAction>,
) {
    #[cfg(feature = "puffin")]
    puffin_profiler::profile_function!();
    ui.group(|ui| {
        if tool.perform.is_some() {
            ui.columns(2, |ui| {
                ui[0].heading(tool.label());
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
                ui[0].heading(tool.label());
            });
        }
        (tool.render)(ui, settings);
    });
}