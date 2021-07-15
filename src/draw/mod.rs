use bevy::prelude::*;
use bevy_inspector_egui::WorldUIContext;
use bevy_inspector_egui::bevy_egui::{egui, EguiContext};

use crate::{DevToolsState, DevToolsSettings, DevToolsTools, SettingValue};

mod diagnostic;
mod setting;
mod tab_bar;
mod tool;
mod top_panel;

pub fn draw_debug_ui(world: &mut World) {
    let world_ptr = world as *mut _;
    let (enabled, always, active) = {
        let resources = world.get_resource::<DevToolsState>().unwrap();
        let settings = world.get_resource::<DevToolsSettings>().unwrap();
        let mut always_visible = false;
        let mut enabled = false;
        if let Some(setting) = settings.named("devtools") {
            for child in setting.children().unwrap() {
                if child.name == "always-visible" {
                    if let SettingValue::Bool(value) = child.value {
                        always_visible = value;
                    }
                }
                if child.name == "enabled" {
                    if let SettingValue::Bool(value) = child.value {
                        enabled = value;
                    }
                }
            }
        }
        (enabled, always_visible, resources.active_tab)
    };

    let egui_context = world.get_resource::<EguiContext>().expect("EguiContext");
    let ctx = egui_context.ctx();

    if enabled || always {
        egui::Window::new("DevTools")
        .enabled(enabled || !always)
        .collapsible(true)
        .show(ctx, |ui| {
            let world: &mut World = unsafe { &mut *world_ptr };
            top_panel::top_panel(ui, world);
            tab_bar::tab_bar(ui, world);
            ui.end_row();

            egui::ScrollArea::auto_sized()
            .show(ui, |ui| {
                match active {
                    crate::helpers::Tab::Diagnostics => {
                        diagnostic::handle_diagnostics(ui, world);
                    }
                    crate::helpers::Tab::World => {
                        let params = world.get_resource::<bevy_inspector_egui::WorldInspectorParams>().unwrap();
                        let world: &mut World = unsafe { &mut *world_ptr };
                        let mut ui_context = WorldUIContext::new(Some(egui_context.ctx()), world);
                        ui_context.world_ui::<()>(ui, params);
                    }
                    crate::helpers::Tab::Tools => {
                        let devtools_tools = world.get_resource::<DevToolsTools>().unwrap();
                        let world: &mut World = unsafe { &mut *world_ptr };
                        let mut devtools_settings = world.get_resource_mut::<DevToolsSettings>().unwrap();
                        let world: &mut World = unsafe { &mut *world_ptr };
                        tool::handle_tools(ui, devtools_tools, &mut devtools_settings, world);
                    }
                    crate::helpers::Tab::Settings => {
                        setting::handle_settings(ui, world);
                    }
                }
            });
        });
    }

}
