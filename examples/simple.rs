use bevy::prelude::*;
use bevy_devtools::{DevToolsExt, NoState};

mod utils;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_devtools::DevToolsPlugin::<NoState>::default())
        .add_startup_system(utils::spawn::spawn_world)
        .add_system(utils::rotates::rotator_system)
        .devtools_enabled()
        .run()
}
