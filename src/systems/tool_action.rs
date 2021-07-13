use bevy::prelude::*;
use bevy::app::Events;

use crate::PerformToolAction;

pub fn perform_tool_action(world: &mut World) {
    let events = {
        let mut reader: Mut<Events<PerformToolAction>> = world.get_resource_mut().unwrap();
        reader.drain().map(|x|x.0).collect::<Vec<crate::DevTool>>()
    };
    for event in events {
        if let Some(perform) = event.perform {
            (perform)(world);
        }
    }
}
