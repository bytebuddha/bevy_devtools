use bevy::prelude::*;
use bevy_inspector_egui::egui::Ui;

use std::fs::File;
use std::io::Write;

use crate::Tool;

pub fn tool() -> Tool {
    Tool {
        name: "save-scene".into(),
        label: Some("Save Scene".into()),
        perform: Some(perform),
        priority: 10,
        perform_icon: Some("💾".into()),
        render,
    }
}

pub fn render(ui: &mut Ui, settings: &mut crate::Settings, _: &mut World) {
    if let Some(setting) = settings.get_key_mut(&["devtools", "tools", "save-scene"]) {
        if let Some(value) = setting.value.as_string_mut() {
            ui.text_edit_singleline(value);
        } else {
            warn!("Settings key devtools -> tools -> save-scene is not a String");
        }
    } else {
        warn!("Settings key devtools -> tools -> save-scene is not found");
    }
}

pub fn perform(world: &mut World) {
    let settings = ignore_none_error!(
        world.get_resource::<crate::Settings>(),
        "Failed to get Settings resource"
    );
    if let Some(setting) = settings.get_key(&["devtools", "tools", "save-scene"]) {
        if let Some(value) = setting.value.as_str() {
            if std::path::Path::new(value).exists() {
                ignore_error!(std::fs::remove_file(&value));
            }
            let mut file = ignore_error!(File::create(&value));
            let type_registry = ignore_none_error!(
                world.get_resource::<bevy::reflect::TypeRegistry>(),
                "Failed to get TypeRegistry resource"
            );
            let scene = DynamicScene::from_world(world, type_registry);
            let scene_data = ignore_error!(scene.serialize_ron(type_registry));
            ignore_error!(file.write_all(scene_data.as_bytes()));
        } else {
            warn!("Settings field devtools -> tools -> save-scene is not a string");
        }
    } else {
        warn!("Settings field devtools -> tools -> save-scene is not found");
    }
}
