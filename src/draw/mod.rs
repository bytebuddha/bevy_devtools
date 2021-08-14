use bevy::prelude::*;
use bevy_inspector_egui::{
    bevy_egui::{egui, EguiContext},
    world_inspector::{WorldInspectorParams, WorldUIContext},
};

use crate::{DevToolsLocation, DevToolsSettings, DevToolsState, DevToolsTools, SettingValue};

mod diagnostic;
mod setting;
mod tab_bar;
mod tool;
mod location;
mod top_panel;

pub fn draw_debug_ui(world: &mut World) {
    let world_ptr = world as *mut _;
    let (mut location, enabled, always, active) = {
        let resources = world.get_resource::<DevToolsState>().unwrap();
        let settings = world.get_resource::<DevToolsSettings>().unwrap();
        let mut always_visible = false;
        let mut enabled = false;
        if let Some(setting) = settings.get_key(&["devtools"]) {
            for child in setting.get_group().unwrap() {
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
        (
            resources.location,
            enabled,
            always_visible,
            resources.active_tab,
        )
    };

    #[cfg(feature = "puffin")]
    let profiler_enabled = {
        let settings = world.get_resource::<DevToolsSettings>().unwrap();
        let mut enabled = false;
        if let Some(setting) = settings.get_key(&["puffin"]) {
            for child in setting.children().unwrap() {
                if child.name == "enabled" {
                    if let SettingValue::Bool(value) = child.value {
                        enabled = value;
                    }
                }
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
                    .show(ctx, |ui| {
                        draw_devtools(egui_context, ui, &mut location, active, world_ptr);
                    });
            }
            DevToolsLocation::LeftSide => {
                egui::SidePanel::left("DevTools").show(ctx, |ui| {
                    ui.set_enabled(enabled || !always);
                    draw_devtools(egui_context, ui, &mut location, active, world_ptr);
                });
            }
            DevToolsLocation::RightSide => {
                egui::SidePanel::right("DevTools").show(ctx, |ui| {
                    ui.set_enabled(enabled || !always);
                    draw_devtools(egui_context, ui, &mut location, active, world_ptr);
                });
            }
        }
    }
    let mut resources = world.get_resource_mut::<DevToolsState>().unwrap();
    if resources.location != location {
        resources.location = location;
    }
}

fn draw_devtools(
    egui_context: &EguiContext,
    ui: &mut egui::Ui,
    location: &mut DevToolsLocation,
    active: crate::helpers::Tab,
    world_ptr: *mut World,
) {
    let world: &mut World = unsafe { &mut *world_ptr };
    location::draw_location(ui, location);
    top_panel::top_panel(ui, world);
    tab_bar::tab_bar(ui, world);
    ui.end_row();

    egui::ScrollArea::auto_sized().show(ui, |ui| match active {
        crate::helpers::Tab::Diagnostics => {
            diagnostic::handle_diagnostics(ui, world);
        }
        crate::helpers::Tab::World => {
            let settings = world.get_resource::<DevToolsSettings>().unwrap();
            let world: &mut World = unsafe { &mut *world_ptr };
            let mut params = world.get_resource_mut::<WorldInspectorParams>().unwrap();
            apply_settings(&mut params, settings);
            let world: &mut World = unsafe { &mut *world_ptr };
            let mut ui_context = WorldUIContext::new(world, Some(egui_context.ctx()));
            ui.group(|ui| {
                ui.columns(1, |ui| {
                    ui_context.world_ui::<()>(&mut ui[0], &mut params);
                })
            });
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
    });
}

pub fn apply_settings(params: &mut WorldInspectorParams, settings: &DevToolsSettings) {
    if let Some(setting) = settings.get_key(&["devtools"]) {
        if let Some(child) = setting.get_named_child("world") {
            for child in child.get_group().unwrap() {
                if let Some(value) = child.value.as_bool() {
                    if child.name == "despawnable" && params.despawnable_entities != value {
                        params.despawnable_entities = value;
                    }
                    if child.name == "sort" && params.sort_components != value {
                        params.sort_components = value;
                    }
                }
            }
        }
    }
}
