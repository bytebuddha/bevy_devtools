use bevy::prelude::*;
use bevy_devtools::{DevToolsExt, Setting, SettingValue};

mod utils;

fn main() {
    App::build()
        .add_plugin(utils::ExamplePlugin)
        .add_plugin(bevy_devtools::DevToolsPlugin::<utils::ExampleState>::default())
        .devtools_enabled()
        .devtools_active_panel(3)
        .devtools_setting(Setting {
            hidden: false,
            name: "custom_setting".into(),
            label: None,
            value: SettingValue::Bool(false),
        })
        .run()
}
