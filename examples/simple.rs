use bevy::prelude::*;
use bevy_devtools::DevToolsExt;

mod utils;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_devtools::DevToolsPlugin::<()>::default())
        .add_startup_system(utils::spawn::spawn_world.system())
        .add_system(utils::rotates::rotator_system.system())
        .devtools_enabled()
        .run()
}
