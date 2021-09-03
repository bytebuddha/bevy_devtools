use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::{egui, EguiContext};

use crate::{DevToolsSettings, SettingValue};

mod tab_bar;
mod top_panel;

pub fn draw_debug_ui(world: &mut World) {
    let world_ptr = world as *mut _;
    let (enabled, active) = get_display_settings(world);

    let egui_context = world.get_resource::<EguiContext>().expect("EguiContext");
    let ctx = egui_context.ctx();

    if enabled {
        egui::Window::new("DevTools")
            .enabled(enabled)
            .collapsible(true)
            .resizable(true)
            .show(ctx, |ui| {
                draw_devtools(egui_context, ui, active, world_ptr);
            });
    }
}

fn get_display_settings(world: &World) -> (bool, usize) {
    let settings = world
        .get_resource::<DevToolsSettings>()
        .expect("Failed to get DevToolsSettings resource");
    let mut enabled = false;
    let mut active_panel = 0;
    if let Some(setting) = settings.get_key(&["devtools"]) {
        if let Some(group) = setting.get_group() {
            for child in group {
                if child.name == "enabled" {
                    if let SettingValue::Bool(value) = child.value {
                        enabled = value;
                    }
                }
                if child.name == "active_panel" {
                    if let SettingValue::Integer(value) = child.value {
                        active_panel = value;
                    }
                }
            }
        } else {
            warn!("Settings key devtools is not a group");
        }
    } else {
        warn!("Settings key devtools was not found");
    }
    (enabled, active_panel as usize)
}

fn draw_devtools(
    egui_context: &EguiContext,
    ui: &mut egui::Ui,
    active: usize,
    world_ptr: *mut World,
) {
    let world: &mut World = unsafe { &mut *world_ptr };
    let panels = world.get_resource::<crate::panels::DevToolsPanels>().unwrap();
    let world: &mut World = unsafe { &mut *world_ptr };
    top_panel::top_panel(ui, world);
    tab_bar::tab_bar(ui, world, panels);
    ui.end_row();

    egui::ScrollArea::auto_sized().show(ui, |ui| {
        if let Some(panel) = panels.0.get(active) {
            (panel.render)(egui_context, ui, world);
        }
    });
}
