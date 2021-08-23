use bevy::prelude::*;

mod rotates;

mod spawn;

pub struct ExamplePlugin;

impl Plugin for ExamplePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugins(DefaultPlugins)
            .add_startup_system(spawn::spawn_world.system())
            .add_system(rotates::rotator_system.system());
    }
}
