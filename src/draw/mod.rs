use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::{egui, EguiContext};

use crate::{DevToolsLocation, DevToolsSettings, DevToolsState, SettingValue};

mod location;
mod tab_bar;
mod top_panel;

pub fn draw_debug_ui(world: &mut World) {
    let world_ptr = world as *mut _;
    let (mut location, enabled, always, active) = {
        let resources = ignore_none_error!(
            world.get_resource::<DevToolsState>(),
            "Failed to get DevToolsState resources"
        );
        let settings = ignore_none_error!(
            world.get_resource::<DevToolsSettings>(),
            "Failed to get DevToolsSettings resource"
        );
        let mut always_visible = false;
        let mut enabled = false;
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
                }
            } else {
                warn!("Settings key devtools is not a group");
            }
        } else {
            warn!("Settings key devtools was not found");
        }
        (
            resources.location,
            enabled,
            always_visible,
            resources.active_tab,
        )
    };

    #[cfg(feature = "puffin")]
    let profiler_enabled = {
        let settings = ignore_none_error!(
            world.get_resource::<DevToolsSettings>(),
            "Failed to get DevToolsSettings resource"
        );
        let mut enabled = false;
        if let Some(setting) = settings.get_key(&["puffin", "enabled"]) {
            if let SettingValue::Bool(value) = setting.value {
                enabled = value;
            }
        }
        enabled
    };

    let egui_context = world.get_resource::<EguiContext>().expect("EguiContext");
    let ctx = egui_context.ctx();

    #[cfg(feature = "puffin")]
    if profiler_enabled {
        egui::Window::new("Profiler")
            .default_size([1024.0, 600.0])
            .show(ctx, |ui| puffin_egui::profiler_ui(ui));
    }

    if enabled || always {
        match location {
            DevToolsLocation::Window => {
                egui::Window::new("DevTools")
                    .enabled(enabled || !always)
                    .collapsible(true)
                    .resizable(true)
                    .show(ctx, |ui| {
                        draw_devtools(egui_context, ui, &mut location, active, world_ptr);
                    });
            }
            DevToolsLocation::LeftSide => {
                egui::SidePanel::left("DevTools")
                    .resizable(true)
                    .show(ctx, |ui| {
                        ui.set_enabled(enabled || !always);
                        draw_devtools(egui_context, ui, &mut location, active, world_ptr);
                    });
            }
            DevToolsLocation::RightSide => {
                egui::SidePanel::right("DevTools")
                    .resizable(true)
                    .show(ctx, |ui| {
                        ui.set_enabled(enabled || !always);
                        draw_devtools(egui_context, ui, &mut location, active, world_ptr);
                    });
            }
        }
    }
    let mut resources = ignore_none_error!(
        world.get_resource_mut::<DevToolsState>(),
        "Failed to get DevToolsState resource"
    );
    if resources.location != location {
        resources.location = location;
    }
}

fn draw_devtools(
    egui_context: &EguiContext,
    ui: &mut egui::Ui,
    location: &mut DevToolsLocation,
    active: usize,
    world_ptr: *mut World,
) {
    let world: &mut World = unsafe { &mut *world_ptr };
    let tabs = world.get_resource::<crate::tabs::DevToolsTabs>().unwrap();
    let world: &mut World = unsafe { &mut *world_ptr };
    top_panel::top_panel(ui, world);
    tab_bar::tab_bar(ui, world, location, tabs);
    ui.end_row();

    egui::ScrollArea::auto_sized().show(ui, |ui| {
        if let Some(tab) = tabs.0.get(active) {
            (tab.render)(egui_context, ui, world);
        }
    });
}
