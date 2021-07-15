use bevy::prelude::*;
use crate::world::WorldInspectorParams;

use crate::DevToolsSettings;

pub fn world_settings(settings: Res<DevToolsSettings>, mut conf: ResMut<WorldInspectorParams>) {
    if let Some(setting) = settings.named("devtools") {
        if let Some(child) = setting.named_child("world") {
            for child in child.children().unwrap() {
                if let Some(value) = child.value.as_bool() {
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

pub fn initial_world_settings(mut settings: ResMut<DevToolsSettings>, conf: ResMut<WorldInspectorParams>) {
    if let Some(setting) = settings.named_mut("devtools") {
        if let Some(child) = setting.named_child_mut("world") {
            if let Some(value) = child.value.as_bool_mut() {
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
