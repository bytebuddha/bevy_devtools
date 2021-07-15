use bevy::prelude::*;
use bevy_inspector_egui::egui::Ui;
use bevy_inspector_egui::egui::DragValue;

use crate::{DevToolsSetting, SettingValue, DevToolsSettings};

pub fn handle_settings(ui: &mut Ui, world: &mut World) {
    let show_hidden = {
        let mut show_hidden = false;
        let settings = world.get_resource::<DevToolsSettings>().unwrap();
        for setting in settings.0.iter() {
            if setting.name == "devtools" {
                for child in setting.children().unwrap() {
                    if child.name == "settings" {
                        for child in child.children().unwrap() {
                            if let SettingValue::Bool(value) = child.value {
                                if child.name == "show-hidden" {
                                    show_hidden = value;
                                }
                            }
                        }
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
    match &mut setting.value {
        SettingValue::Group(group) => {
            ui.collapsing(setting.label.as_ref().unwrap_or(&setting.name), |ui| {
                for child in group.iter_mut() {
                    if !child.hidden || force {
                        display_setting(ui, child, !child.hidden || force);
                    }
                }
            });
        }
        SettingValue::Bool(ref mut data) => {
            ui.checkbox(data, setting.label.as_ref().unwrap_or(&setting.name));
        }
        SettingValue::Float(ref mut float) => {
            ui.label(setting.label.as_ref().unwrap_or(&setting.name));
            ui.add(DragValue::new(float).speed(0.1));
        }
        SettingValue::String(ref mut data) => {
            ui.label(setting.label.as_ref().unwrap_or(&setting.name));
            ui.end_row();
            ui.text_edit_singleline(data);
        }
    }
}
