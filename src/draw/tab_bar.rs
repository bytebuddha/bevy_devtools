use bevy::prelude::*;
use bevy_inspector_egui::egui::Ui;

use crate::{DevToolsSettings, DevToolsTabs};

pub fn tab_bar(ui: &mut Ui, world: &mut World, tabs: &DevToolsTabs) {
    let mut settings = ignore_none_error!(
        world.get_resource_mut::<DevToolsSettings>(),
        "Failed to get DevToolsState resources"
    );
    if let Some(setting) = settings.get_key_mut(&["devtools", "active_tab"]) {
        if let Some(value) = setting.value.as_integer_mut() {
            ui.columns(2, |ui| {
                ui[1].columns(tabs.0.len(), |ui| {
                    for (dex, tab) in tabs.0.iter().enumerate() {
                        if ui[dex]
                            .selectable_label(dex as i32 == *value, &tab.icon)
                            .clicked()
                        {
                            *value = dex as i32;
                        }
                    }
                });
            });
        }
    }
}
