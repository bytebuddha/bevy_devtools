use bevy::prelude::*;
use bevy_inspector_egui::egui::Ui;

use crate::DevToolsSetting;

pub struct PerformToolAction(pub DevTool);

pub struct DevToolsTools(pub Vec<DevTool>);

#[derive(Clone)]
pub struct DevTool {
    pub name: String,
    pub perform_icon: Option<String>,
    pub label: Option<String>,
    pub render: fn(&mut Ui, &mut crate::DevToolsSettings),
    pub perform: Option<fn(&mut World)>
}

impl Default for DevToolsTools {
    fn default() -> DevToolsTools {
        DevToolsTools(vec![
            DevTool {
                name: "save-scene".into(),
                label: Some("Save Scene".into()),
                perform: Some(perform),
                perform_icon: Some("💾".into()),
                render,
            }
        ])
    }
}

fn render(ui: &mut Ui, settings: &mut crate::DevToolsSettings) {
    for setting in settings.0.iter_mut() {
        if setting.name() == "devtools" {
            for child in setting.children_mut().unwrap() {
                if child.name() == "tools" {
                    for child in child.children_mut().unwrap() {
                           if let DevToolsSetting::String { name, value, .. } = child {
                               if name == "save-scene" {
                                   ui.text_edit_singleline(value);
                               }

                           }
                    }
                }
            }
        }
    }
}

use std::fs::File;
use std::io::Write;

fn perform(world: &mut World) {
    let settings = world.get_resource::<crate::DevToolsSettings>().unwrap();
    for setting in settings.0.iter() {
        if setting.name() == "devtools" {
            for child in setting.children().unwrap() {
                if child.name() == "tools" {
                    for child in child.children().unwrap() {
                           if let DevToolsSetting::String { name, value, .. } = child {
                               if name == "save-scene" {
                                   let mut file = File::create(&value).unwrap();
                                   let type_registry = world.get_resource::<bevy::reflect::TypeRegistry>().unwrap();
                                   let scene = DynamicScene::from_world(&world, &type_registry);
                                   let scene_data = scene.serialize_ron(&type_registry).unwrap();
                                   file.write_all(scene_data.as_bytes()).unwrap();
                               }

                           }
                    }
                }
            }
        }
    }
}
