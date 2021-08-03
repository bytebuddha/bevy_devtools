use bevy::prelude::*;
use bevy_inspector_egui::egui::DragValue;
use bevy_inspector_egui::egui::Ui;

use crate::{DevToolsSetting, DevToolsSettings, SettingValue};

pub fn handle_settings(ui: &mut Ui, world: &mut World) {
    let show_hidden = {
        let mut show_hidden = false;
        let settings = world.get_resource::<DevToolsSettings>().unwrap();
        if let Some(setting) = settings.named("devtools") {
            if let Some(child) = setting.named_child("settings") {
                if let Some(child) = child.named_child("show-hidden") {
                    if let Some(value) = child.value.as_bool() {
                        show_hidden = value;
                    }
                }
            }
        }
        show_hidden
    };
    let mut settings = world.get_resource_mut::<DevToolsSettings>().unwrap();
    if show_hidden {
        for setting in settings.0.iter_mut() {
            display_setting(ui, setting, true);
        }
    } else {
        for setting in settings.0.iter_mut().filter(|x| !x.hidden) {
            display_setting(ui, setting, false);
        }
    }
}

pub fn display_setting(ui: &mut Ui, setting: &mut DevToolsSetting, force: bool) {
    let label = setting.label.as_ref().unwrap_or(&setting.name);
    match &mut setting.value {
        SettingValue::Group(group) => {
            ui.collapsing(label, |ui| {
                for child in group.iter_mut() {
                    if !child.hidden || force {
                        display_setting(ui, child, !child.hidden || force);
                    }
                }
            });
        }
        SettingValue::Bool(ref mut data) => {
            ui.checkbox(data, label);
        }
        SettingValue::Float(ref mut float) => {
            let value = DragValue::new(float).speed(0.1);
            ui.horizontal(|ui| {
                ui.label(label);
                ui.add(value);
            });
        }
        SettingValue::String(ref mut data) => {
            ui.label(label);
            ui.end_row();
            ui.text_edit_singleline(data);
        }
    }
}
