use bevy::prelude::*;

use crate::{DevToolsSettings, DevToolsState};

pub(crate) fn toggle_devtools(
    keys: Res<Input<KeyCode>>,
    resources: Res<DevToolsState>,
    mut settings: ResMut<DevToolsSettings>,
) {
    #[cfg(feature = "puffin")]
    puffin_profiler::profile_function!();

    if keys.just_pressed(resources.toggle_key) {
        if let Some(setting) = settings.get_key_mut(&["devtools", "enabled"]) {
            if let Some(value) = setting.value.as_bool_mut() {
                *value = !*value;
            } else {
                warn!("Settings field `{}` expected Bool type", setting.name)
            }
        } else {
            warn!("No Setting `devtools -> enabled` found")
        }
    }

    #[cfg(feature = "puffin")]
    if keys.just_pressed(resources.profiler_key) {
        if let Some(setting) = settings.get_key_mut(&["puffin", "enabled"]) {
            if let Some(value) = setting.value.as_bool_mut() {
                *value = !*value;
            } else {
                warn!("Settings field `{}` expected Bool type", setting.name)
            }
        } else {
            warn!("No Setting `devtools -> enabled` found")
        }
    }
}
