use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiContext;

use crate::{helpers::DraculaEgui, DevToolsSettings, SettingValue};

pub fn apply_ui_settings(context: ResMut<EguiContext>, settings: Res<DevToolsSettings>) {
    let ctx = context.ctx();
    let mut style = DraculaEgui::style();
    for setting in settings.0.iter() {
        if setting.name == "devtools" {
            for child in setting.children().unwrap() {
                if child.name == "gui" {
                    for child in child.children().unwrap() {
                        if let SettingValue::Bool(value) = child.value {
                            if child.name == "widgets-hover" {
                                style.debug.show_widgets = value;
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
            }
        }
    }
    ctx.set_style(style);
}
