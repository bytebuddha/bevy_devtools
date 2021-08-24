use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::egui::Ui;

pub fn top_panel(ui: &mut Ui, world: &mut World) {
    let diagnostics = ignore_none_error!(
        world.get_resource::<Diagnostics>(),
        "Failed to get Diagnostics resource"
    );
    let fps = diagnostic_value!(diagnostics, FrameTimeDiagnosticsPlugin::FPS);
    let avg = diagnostic_value!(diagnostics, FrameTimeDiagnosticsPlugin::FRAME_TIME);
    let count = diagnostic_value!(diagnostics, FrameTimeDiagnosticsPlugin::FRAME_COUNT);
    ui.group(|ui| {
        ui.columns(3, |ui| {
            let layout = crate::egui::Layout::from_main_dir_and_cross_align(
                crate::egui::Direction::TopDown,
                crate::egui::Align::Center,
            );
            ui[0].with_layout(layout, |ui| ui.label(format!("FPS:{:.0}", fps.abs())));
            ui[1].with_layout(layout, |ui| ui.label(format!("AVG:{:.4}", avg.abs())));
            ui[2].with_layout(layout, |ui| ui.label(format!("Count:{}", count.abs())));
        });
    });
}
