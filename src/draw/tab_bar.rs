use bevy::prelude::*;
use bevy_inspector_egui::egui::Ui;

use crate::{Settings, Panels, PanelLocation};

pub fn tab_bar(ui: &mut Ui, world: &mut World, panels: &Panels) {
    let mut settings = ignore_none_error!(
        world.get_resource_mut::<Settings>(),
        "Failed to get Settings resources"
    );
    if let Some(setting) = settings.get_key_mut(&["devtools", "active_panel"]) {
        if let Some(value) = setting.value.as_integer_mut() {
            let columns = panels.get_location(PanelLocation::Widget);
            ui.columns(columns.len(), |ui| {
                for (col_dex, (dex, panel)) in columns.iter().enumerate() {
                    if ui[col_dex]
                        .selectable_label(*dex as i32 == *value, &panel.icon)
                        .clicked()
                    {
                        *value = *dex as i32;
                    }
                }
            });
        }
    }
}
