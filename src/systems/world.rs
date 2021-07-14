use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorParams;

use crate::{DevToolsSettings, SettingValue};

pub fn world_settings(settings: Res<DevToolsSettings>, mut conf: ResMut<WorldInspectorParams>) {
    for setting in settings.0.iter() {
        if setting.name == "devtools" {
            for child in setting.children().unwrap() {
                if child.name == "world" {
                    for child in child.children().unwrap() {
                        if let SettingValue::Bool(value) = child.value {
                            if child.name == "despawnable" && conf.despawnable_entities != value {
                                    conf.despawnable_entities = value;
                            }
                            if child.name == "sort" && conf.sort_components != value {
                                    conf.sort_components = value;
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn initial_world_settings(mut settings: ResMut<DevToolsSettings>, conf: ResMut<WorldInspectorParams>) {
    for setting in settings.0.iter_mut() {
        if setting.name == "devtools" {
            for child in setting.children_mut().unwrap() {
                if child.name == "world" {
                    for child in child.children_mut().unwrap() {
                        if let SettingValue::Bool(ref mut value) = child.value {
                            if child.name == "despawnable" {
                                *value = conf.despawnable_entities;
                            }
                            if child.name == "sort" {
                                *value = conf.sort_components;
                            }
                        }
                    }
                }
            }
        }
    }
}
