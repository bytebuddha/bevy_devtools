use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiContext;

use crate::{helpers::DraculaEgui, DevToolsSettings};

pub fn apply_ui_settings(context: ResMut<EguiContext>, settings: Res<DevToolsSettings>) {
    #[cfg(feature = "puffin")]
    puffin_profiler::profile_function!();
    let ctx = context.ctx();
    let mut style = DraculaEgui::style();
    if let Some(setting) = settings.get_key(&["devtools", "gui"]) {
        for child in setting.children().unwrap() {
            if let Some(value) = child.value.as_bool() {
                if child.name == "widgets-hover" {
                    style.debug.debug_on_hover = value;
                }
                if child.name == "widgets-taller" {
                    style.debug.show_expand_height = value;
                }
                if child.name == "widgets-wider" {
                    style.debug.show_expand_width = value;
                }
                if child.name == "show-resize" {
                    style.debug.show_resize = value;
                }
            }
        }
    }
    ctx.set_style(style);
}
