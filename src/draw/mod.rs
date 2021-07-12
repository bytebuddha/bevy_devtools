use bevy::diagnostic::Diagnostics;
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::{
    egui,
    EguiContext,
};

use crate::{ DevToolsTools, DevToolsSettings, DevToolsResources, PerformToolAction };

mod diagnostic;
mod setting;
mod tab_bar;
mod tool;
mod top_panel;

pub fn draw_debug_ui(
    context: Res<EguiContext>,
    diagnostics: Res<Diagnostics>,
    mut resources: ResMut<DevToolsResources>,
    mut settings: ResMut<DevToolsSettings>,
    devtools_diagnostics: Res<crate::DevToolsDiagnostics>,
    tools: Res<DevToolsTools>,
    mut tool_actions: EventWriter<PerformToolAction>
) {
    if resources.always_visible || resources.enabled {
        let mut pos = context.ctx().available_rect().right_top();
        pos.y += 15.0;
        pos.x -= 370.0;
        egui::Window::new("DevTools")
            .default_pos(pos)
            .enabled(resources.enabled || !resources.always_visible)
            .collapsible(true)
            .show(context.ctx(), |ui| {
                top_panel::top_panel(ui, &mut resources, &diagnostics);
                tab_bar::tab_bar(ui, &mut resources);
                ui.end_row();
                match resources.active_tab {
                    crate::helpers::Tab::Diagnostics => {
                        for group in devtools_diagnostics.0.iter() {
                            diagnostic::display_diagnostic(ui, &diagnostics, group);
                        }
                    },
                    crate::helpers::Tab::Tools => {
                        for tool in tools.0.iter() {
                            tool::display_tool(ui, &mut settings, tool, &mut tool_actions);
                        }
                    }
                    crate::helpers::Tab::Settings => {
                        for setting in settings.0.iter_mut() {
                             setting::display_setting(ui, setting);
                        }
                    }
                }
            });
    }
}
