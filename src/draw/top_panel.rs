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
            ui[0].label(format!("FPS:{:.0}", fps.abs()));
            ui[1].label(format!("AVG:{:.4}", avg.abs()));
            ui[2].label(format!("Count:{}", count.abs()));
        });
    });
}
