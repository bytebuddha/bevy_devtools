use bevy::prelude::*;

use crate::{DevToolsResources, DevToolsSettings, SettingValue};

pub(crate) fn toggle_devtools(
    keys: Res<Input<KeyCode>>,
    resources: Res<DevToolsResources>,
    mut settings: ResMut<DevToolsSettings>
) {
    if keys.just_pressed(resources.toggle_key) {
//        resources.enabled = !resources.enabled;
        for setting in settings.0.iter_mut() {
            if setting.name == "devtools" {
                for child in setting.children_mut().unwrap() {
                    if child.name == "enabled" {
                        if let SettingValue::Bool(ref mut value) = child.value {
                            *value = !*value;
                        }
                    }
                }
            }
        }
    }
}
