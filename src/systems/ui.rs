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
                } else {
                    warn!("Unable to find settings path `devtools -> gui -> widgets-hover`");
                }
                if child.name == "widgets-taller" {
                    style.debug.show_expand_height = value;
                } else {
                    warn!("Unable to find settings path `devtools -> gui -> widgets-taller`");
                }
                if child.name == "widgets-wider" {
                    style.debug.show_expand_width = value;
                } else {
                    warn!("Unable to find settings path `devtools -> gui -> widgets-wider`");
                }
                if child.name == "show-resize" {
                    style.debug.show_resize = value;
                } else {
                    warn!("Unable to find settings path `devtools -> gui -> show-resize`");
                }
            } else {
                warn!("Settings field `{}` expected Bool type", child.name)
            }
        }
    } else {
        warn!("Unable to find settings path `devtools -> gui`");
    }
    ctx.set_style(style);
}
