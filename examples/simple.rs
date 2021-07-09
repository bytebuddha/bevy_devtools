use bevy::prelude::*;
use bevy_devtools::{DevToolsPlugin, DevToolsEvent};

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(DevToolsPlugin)
        .add_startup_system(start_debug.system())
        .run()
}

fn start_debug(mut events: EventWriter<DevToolsEvent>) {
    std::thread::sleep(std::time::Duration::from_secs(5));
    events.send(DevToolsEvent::Start);
}
