use bevy::prelude::*;
use bevy_devtools::{DevToolsExt, DevToolsSetting, SettingValue};

mod utils;

fn main() {
    App::build()
        .add_plugin(utils::ExamplePlugin)
        .add_plugin(bevy_devtools::DevToolsPlugin)
        .devtools_enabled()
        .devtools_active_panel(3)
        .devtools_setting(DevToolsSetting {
            hidden: false,
            name: "custom_setting".into(),
            label: None,
            value: SettingValue::Bool(false),
        })
        .run()
}
