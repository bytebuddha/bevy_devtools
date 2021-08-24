use bevy::app::Events;
use bevy::prelude::*;

use crate::PerformToolAction;

pub fn perform_tool_action(world: &mut World) {
    let events = {
        let mut reader: Mut<Events<PerformToolAction>> = ignore_none_error!(
            world.get_resource_mut(),
            "Failed to get Events<PerformToolAction> resource"
        );
        reader.drain().map(|x| x.0).collect::<Vec<crate::DevTool>>()
    };
    for event in events {
        if let Some(perform) = event.perform {
            (perform)(world);
        }
    }
}
