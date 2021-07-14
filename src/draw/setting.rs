use bevy_inspector_egui::egui::Ui;

use crate::{DevToolsSetting, SettingValue};

pub fn display_setting(ui: &mut Ui, setting: &mut DevToolsSetting) {
    match &mut setting.value {
        SettingValue::Group(group) => {
            ui.collapsing(setting.label.as_ref().unwrap_or(&setting.name), |ui| {
                for child in  group.iter_mut() {
                    display_setting(ui, child);
                }
            });
        },
        SettingValue::Bool(ref mut data) => {
            ui.checkbox(data, setting.label.as_ref().unwrap_or(&setting.name));
        },
        SettingValue::String(ref mut data) => {
            ui.label(setting.label.as_ref().unwrap_or(&setting.name));
            ui.end_row();
            ui.text_edit_singleline(data);
        }
    }
}
