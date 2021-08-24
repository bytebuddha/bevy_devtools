use bevy::prelude::*;
use bevy_inspector_egui::{
    bevy_egui::{egui, EguiContext},
    world_inspector::{WorldInspectorParams, WorldUIContext},
};

use crate::DevToolsSettings;

pub fn tab() -> super::DevToolsTab {
    super::DevToolsTab::new("🗺").render(draw)
}

fn draw(ctx: &EguiContext, ui: &mut egui::Ui, world: &mut World) {
    let world_ptr = world as *mut _;
    let settings = ignore_none_error!(
        world.get_resource::<DevToolsSettings>(),
        "Failed to get DevToolsSettings resource"
    );
    let world: &mut World = unsafe { &mut *world_ptr };
    let mut params = ignore_none_error!(
        world.get_resource_mut::<WorldInspectorParams>(),
        "Failed to get WorldInspectorParams resource"
    );
    apply_settings(&mut params, settings);
    let world: &mut World = unsafe { &mut *world_ptr };
    ui.group(|ui| {
        ui.columns(1, |ui| {
            let mut ui_context = WorldUIContext::new(world, Some(ctx.ctx()));
            ui_context.world_ui::<()>(&mut ui[0], &mut params);
        })
    });
}

pub fn apply_settings(params: &mut WorldInspectorParams, settings: &DevToolsSettings) {
    if let Some(setting) = settings.get_key(&["devtools"]) {
        if let Some(child) = setting.get_named_child("world") {
            if let Some(group) = child.get_group() {
                for child in group {
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
}
