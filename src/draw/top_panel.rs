use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy_inspector_egui::bevy_egui::{
    egui::widgets::plot::{Curve, Plot},
    egui::Ui,
};

use crate::{helpers::DraculaEgui, DevToolsResources};

pub fn top_panel(ui: &mut Ui, resources: &mut DevToolsResources, diagnostics: &Diagnostics) {
    ui.group(|ui| {
        ui.columns(1, |ui| {
            ui[0].checkbox(&mut resources.always_visible, "Always Visible");
        });
        ui.group(|ui| {
            let fps = diagnostics
                .get(FrameTimeDiagnosticsPlugin::FPS)
                .unwrap()
                .value()
                .unwrap_or(0.0);
            resources.history.push_fps(fps);
            let avg = diagnostics
                .get(FrameTimeDiagnosticsPlugin::FRAME_TIME)
                .unwrap()
                .value()
                .unwrap_or(0.0);
            let count = diagnostics
                .get(FrameTimeDiagnosticsPlugin::FRAME_COUNT)
                .unwrap()
                .value()
                .unwrap_or(0.0);
            ui.columns(4, |ui| {
                ui[0].label(format!("FPS: {:.0}", fps.abs()));
                ui[1].label(format!("TIME: {:.4}", avg.abs()));
                ui[3].label(format!("Count: {}", count.abs()));
            });
            ui.end_row();
            ui.add(
                Plot::default()
                    .include_x(crate::consts::HISTORY_LENGTH as f32)
                    .height(50.0)
                    .show_x(false)
                    .curve(Curve::from_ys_f32(&resources.history.fps).color(DraculaEgui::PINK)),
            );
        });
    });
}
