use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::{egui, EguiContext};

use crate::{DevToolsSettings, SettingValue};

mod tab_bar;
mod top_panel;

pub fn draw_debug_ui(world: &mut World) {
    let world_ptr = world as *mut _;
    let (enabled, always, active) = get_display_settings(world);

    let egui_context = world.get_resource::<EguiContext>().expect("EguiContext");
    let ctx = egui_context.ctx();

    if enabled || always {
        egui::Window::new("DevTools")
            .enabled(enabled || !always)
            .collapsible(true)
            .resizable(true)
            .show(ctx, |ui| {
                draw_devtools(egui_context, ui, active, world_ptr);
            });
    }
}

fn get_display_settings(world: &World) -> (bool, bool, usize) {
    let settings = world
        .get_resource::<DevToolsSettings>()
        .expect("Failed to get DevToolsSettings resource");
    let mut always_visible = false;
    let mut enabled = false;
    let mut active_tab = 0;
    if let Some(setting) = settings.get_key(&["devtools"]) {
        if let Some(group) = setting.get_group() {
            for child in group {
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
                if child.name == "active_tab" {
                    if let SettingValue::Integer(value) = child.value {
                        active_tab = value;
                    }
                }
            }
        } else {
            warn!("Settings key devtools is not a group");
        }
    } else {
        warn!("Settings key devtools was not found");
    }
    (enabled, always_visible, active_tab as usize)
}

fn draw_devtools(
    egui_context: &EguiContext,
    ui: &mut egui::Ui,
    active: usize,
    world_ptr: *mut World,
) {
    let world: &mut World = unsafe { &mut *world_ptr };
    let tabs = world.get_resource::<crate::tabs::DevToolsTabs>().unwrap();
    let world: &mut World = unsafe { &mut *world_ptr };
    top_panel::top_panel(ui, world);
    tab_bar::tab_bar(ui, world, tabs);
    ui.end_row();

    egui::ScrollArea::auto_sized().show(ui, |ui| {
        if let Some(tab) = tabs.0.get(active) {
            (tab.render)(egui_context, ui, world);
        }
    });
}
