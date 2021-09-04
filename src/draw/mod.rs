use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::{egui, EguiContext};

use crate::{Settings, SettingValue, PanelLocation, Panels};

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

        draw_windows(egui_context, world_ptr);
    }
}

fn draw_windows(egui_context: &EguiContext, world_ptr: *mut World) {
    let world: &mut World = unsafe { &mut *world_ptr };
    let mut panels = world.get_resource_mut::<Panels>().expect("Failed to get panels resource");
    let world: &mut World = unsafe { &mut *world_ptr };
    for (_, panel) in panels.get_location_mut(PanelLocation::Window) {
        let mut is_open = true;
        egui::Window::new(&panel.icon)
            .collapsible(true)
            .open(&mut is_open)
            .show(egui_context.ctx(), |ui| {
                (panel.render)(egui_context, ui, world);
            });
        if !is_open {
            panel.location = PanelLocation::Widget;
        }
    }
}

fn get_display_settings(world: &World) -> (bool, usize) {
    let settings = world
        .get_resource::<Settings>()
        .expect("Failed to get Settings resource");
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
    let mut panels = world.get_resource_mut::<crate::panels::Panels>().unwrap();
    let world: &mut World = unsafe { &mut *world_ptr };
    if let Some(res) = world.get_resource::<crate::TopPanel>() {
        (res.0)(ui, world);
    } else {
        top_panel::top_panel(ui, world);
    }
    tab_bar::tab_bar(ui, world, &panels);
    ui.end_row();
    ui.columns(1, |ui| {
        if ui[0].button("Seperate").clicked() {
            for (dex, panel) in panels.get_location_mut(PanelLocation::Widget) {
                if dex == active {
                    panel.location = PanelLocation::Window;
                }
            }
        }
    });
    ui.end_row();

    egui::ScrollArea::auto_sized().show(ui, |ui| {
        let panels = panels.get_location_mut(PanelLocation::Widget);
        if let Some((_, panel)) = panels.iter().filter(|x| x.0 == active).next() {
            (panel.render)(egui_context, ui, world);
        } else {
            let mut settings = world.get_resource_mut::<Settings>().unwrap();
            set_active_panel(&mut settings, panels.first().unwrap().0);
        }
    });
}

fn set_active_panel(settings: &mut Settings, active: usize) {
    if let Some(setting) = settings.get_key_mut(&["devtools", "active_panel"]) {
        if let Some(b) = setting.value.as_integer_mut() {
            *b = active as i32;
        }
    }
}
