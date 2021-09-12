use crate::bevy_egui::EguiContext;
use crate::egui::Ui;
use bevy::app::Events;
use bevy::prelude::*;

use crate::{Tool, Settings, Tools, PerformToolAction};

pub fn panel() -> super::Panel {
    super::Panel::new("🛠").render(draw)
}

pub fn draw(_: &EguiContext, ui: &mut Ui, world: &mut World) {
    let world_ptr = world as *mut _;
    let mut devtools_tools = ignore_none_error!(
        world.get_resource_mut::<Tools>(),
        "Failed to get Tools resource"
    );
    let world: &mut World = unsafe { &mut *world_ptr };
    let mut devtools_settings = ignore_none_error!(
        world.get_resource_mut::<Settings>(),
        "Failed to get Settings resource"
    );
    let world: &mut World = unsafe { &mut *world_ptr };
    let mut tool_actions = ignore_none_error!(
        world.get_resource_mut::<Events<PerformToolAction>>(),
        "Failed to get Events<PerformToolAction> resources"
    );
    if !devtools_tools.0.is_sorted_by_key(|x|x.priority) {
        devtools_tools.0.sort_by_key(|x|x.priority);
    }
    for tool in devtools_tools.0.iter() {
        display_tool(ui, &mut *devtools_settings, tool, &mut tool_actions);
    }
}

pub fn display_tool(
    ui: &mut Ui,
    settings: &mut Settings,
    tool: &Tool,
    tool_actions: &mut Events<PerformToolAction>,
) {
    ui.group(|ui| {
        if tool.perform.is_some() {
            ui.columns(2, |ui| {
                ui[0].heading(tool.label());
                let label = tool
                    .perform_icon
                    .as_ref()
                    .map(|x| x.as_str())
                    .unwrap_or("Perform");
                if ui[1].small_button(label).clicked() {
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
