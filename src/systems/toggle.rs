use bevy::prelude::*;

use crate::DevToolsResources;

pub fn toggle_devtools(
    keys: Res<Input<KeyCode>>,
    mut resources: ResMut<DevToolsResources>
) {
    if keys.just_pressed(resources.toggle_key) {
        resources.enabled = !resources.enabled;
    }
}
