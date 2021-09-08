use bevy::prelude::*;

pub mod rotates;

pub mod spawn;

mod fps;
pub use self::fps::FPSHistory;

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct ExampleState;

pub struct ExamplePlugin;

impl Plugin for ExamplePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugins(DefaultPlugins)
            .add_startup_system(spawn::spawn_world.system())
            .add_system(rotates::rotator_system.system())
            .add_state(ExampleState);
    }
}
