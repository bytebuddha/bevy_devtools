use bevy::prelude::*;
use bevy_devtools::{DevToolsExt, DevToolsSetting, DevToolsTab, SettingValue};

mod utils;

fn main() {
    App::build()
        .add_plugin(utils::ExamplePlugin)
        .add_plugin(bevy_devtools::DevToolsPlugin)
        .devtools_enabled()
        .devtools_active_tab(DevToolsTab::Settings)
        .devtools_setting(DevToolsSetting {
            hidden: false,
            name: "custom_setting".into(),
            label: None,
            value: SettingValue::Bool(false),
        })
        .run()
}
