use bevy::prelude::*;
use bevy_inspector_egui::egui::Ui;

use crate::{DevToolsSettings, DevToolsPanels};

pub fn tab_bar(ui: &mut Ui, world: &mut World, panels: &DevToolsPanels) {
    let mut settings = ignore_none_error!(
        world.get_resource_mut::<DevToolsSettings>(),
        "Failed to get DevToolsSettings resources"
    );
    if let Some(setting) = settings.get_key_mut(&["devtools", "active_panel"]) {
        if let Some(value) = setting.value.as_integer_mut() {
            ui.columns(panels.0.len(), |ui| {
                for (dex, panel) in panels.0.iter().enumerate() {
                    if ui[dex]
                        .selectable_label(dex as i32 == *value, &panel.icon)
                        .clicked()
                    {
                        *value = dex as i32;
                    }
                }
            });
        }
    }
}
