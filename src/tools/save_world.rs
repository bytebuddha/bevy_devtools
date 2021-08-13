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
    #[cfg(feature = "puffin")]
    puffin_profiler::profile_function!();
    if let Some(setting) = settings.get_key_mut(&["devtools", "tools", "save-scene"]) {
        if let Some(value) = setting.value.as_string_mut() {
            ui.text_edit_singleline(value);
        }
    }
}

pub fn perform(world: &mut World) {
    #[cfg(feature = "puffin")]
    puffin_profiler::profile_function!();
    let settings = world.get_resource::<crate::DevToolsSettings>().unwrap();
    if let SettingValue::String(ref value) = settings.get_key(&["devtools", "tools", "save-scene"]).unwrap().value {
        if std::path::Path::new(value).exists() {
            std::fs::remove_file(&value).unwrap();
        }
        let mut file = File::create(&value).unwrap();
        let type_registry =
        world.get_resource::<bevy::reflect::TypeRegistry>().unwrap();
        let scene = DynamicScene::from_world(world, type_registry);
        let scene_data = scene.serialize_ron(type_registry).unwrap();
        file.write_all(scene_data.as_bytes()).unwrap();
    }
}
