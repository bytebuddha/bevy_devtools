use bevy_inspector_egui::egui::Ui;

use crate::DevToolsSetting;

pub fn display_setting(ui: &mut Ui, setting: &mut DevToolsSetting) {
    match setting {
        DevToolsSetting::Group { name, label, children } => {
            ui.collapsing(label.as_ref().unwrap_or(name), |ui| {
                for child in children {
                    display_setting(ui, child);
                }
            });
        },
        DevToolsSetting::Bool { name, label, value } => {
            ui.checkbox(value, label.as_ref().unwrap_or(name),);
        },
        DevToolsSetting::String { name, label, value } => {
            ui.label(label.as_ref().unwrap_or(name));
            ui.end_row();
            ui.text_edit_singleline(value);
        }
    }
}
