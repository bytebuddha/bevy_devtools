use bevy::prelude::*;
use bevy_devtools::DevToolsExt;

mod utils;

fn main() {
    App::build()
        .add_plugin(utils::ExamplePlugin)
        .add_plugin(bevy_devtools::DevToolsPlugin)
        .devtools_enabled()
        .run()
}
