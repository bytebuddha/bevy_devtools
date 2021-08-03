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
        if let Some(setting) = settings.named_mut("devtools") {
            if let Some(setting) = setting.named_child_mut("enabled") {
                if let Some(value) = setting.value.as_bool_mut() {
                    *value = !*value;
                }
            }
        }
    }

    #[cfg(feature = "puffin")]
    if keys.just_pressed(resources.profiler_key) {
        if let Some(setting) = settings.named_mut("puffin") {
            if let Some(setting) = setting.named_child_mut("enabled") {
                if let Some(value) = setting.value.as_bool_mut() {
                    *value = !*value;
                }
            }
        }
    }
}
