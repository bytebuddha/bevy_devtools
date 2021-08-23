use bevy_inspector_egui::egui::Ui;

use crate::DevToolsLocation;

pub fn draw_location(ui: &mut Ui, location: &mut DevToolsLocation) {
    ui.horizontal(|ui| {
        if ui
            .selectable_label(*location == DevToolsLocation::LeftSide, "«")
            .clicked()
        {
            *location = DevToolsLocation::LeftSide;
        }
        if ui
            .selectable_label(*location == DevToolsLocation::Window, "🗗")
            .clicked()
        {
            *location = DevToolsLocation::Window;
        }
        if ui
            .selectable_label(*location == DevToolsLocation::RightSide, "»")
            .clicked()
        {
            *location = DevToolsLocation::RightSide;
        }
    });
}
