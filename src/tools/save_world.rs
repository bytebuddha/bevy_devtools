use bevy::prelude::*;
use bevy_inspector_egui::egui::Ui;

use std::fs::File;
use std::io::Write;

use crate::{DevTool, SettingValue};

pub fn tool() -> DevTool {
    DevTool {
        name: "save-scene".into(),
        label: Some("Save Scene".into()),
        perform: Some(perform),
        perform_icon: Some("💾".into()),
        render,
    }
}

pub fn render(ui: &mut Ui, settings: &mut crate::DevToolsSettings) {
    for setting in settings.0.iter_mut() {
        if setting.name == "devtools" {
            for child in setting.children_mut().unwrap() {
                if child.name == "tools" {
                    for child in child.children_mut().unwrap() {
                        if let SettingValue::String(value) = &mut child.value {
                            if child.name == "save-scene" {
                                ui.text_edit_singleline(value);
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn perform(world: &mut World) {
    let settings = world.get_resource::<crate::DevToolsSettings>().unwrap();
    for setting in settings.0.iter() {
        if setting.name == "devtools" {
            for child in setting.children().unwrap() {
                if child.name == "tools" {
                    for child in child.children().unwrap() {
                        if let SettingValue::String(value) = &child.value {
                            if child.name == "save-scene" {
                                let mut file = File::create(&value).unwrap();
                                let type_registry =
                                    world.get_resource::<bevy::reflect::TypeRegistry>().unwrap();
                                let scene = DynamicScene::from_world(world, type_registry);
                                let scene_data = scene.serialize_ron(type_registry).unwrap();
                                file.write_all(scene_data.as_bytes()).unwrap();
                            }
                        }
                    }
                }
            }
        }
    }
}
