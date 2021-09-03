use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiContext;

use crate::Settings;

pub fn apply_ui_settings(context: ResMut<EguiContext>, settings: Res<Settings>) {
    let ctx = context.ctx();
    if let Some(setting) = settings.get_key(&["devtools", "gui"]) {
        if let Some(group) = setting.get_group() {
            for child in group {
                if let Some(value) = child.value.as_bool() {
                    let mut style = (*context.ctx().style()).clone();
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
                    ctx.set_style(style);
                } else {
                    warn!("Settings field `{}` expected Bool type", child.name)
                }
            }
        } else {
            warn!("Settings key devtools -> gui is not a group");
        }
    } else {
        warn!("Unable to find settings key `devtools -> gui`");
    }
}
