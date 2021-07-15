use bevy::prelude::*;
use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy_inspector_egui::bevy_egui::{
    egui::widgets::plot::{Line, Plot, Value, Values},
    egui::Ui,
};

use crate::{helpers::DraculaEgui, DevToolsState};

pub fn top_panel(ui: &mut Ui, world: &mut World) {
    let fps = {
        let fps = {
            let diagnostics = world.get_resource::<Diagnostics>().unwrap();
            diagnostics
                .get(FrameTimeDiagnosticsPlugin::FPS)
                .unwrap()
                .value()
                .unwrap_or(0.0)
        };
        let mut resources = world.get_resource_mut::<DevToolsState>().unwrap();
        resources.history.push_fps(fps);
        fps
    };
    let diagnostics = world.get_resource::<Diagnostics>().unwrap();
    let resources = world.get_resource::<DevToolsState>().unwrap();
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
    ui.group(|ui| {
        ui.columns(3, |ui| {
            let layout = crate::egui::Layout::from_main_dir_and_cross_align(
                crate::egui::Direction::TopDown,
                crate::egui::Align::Center
            );
            ui[0].with_layout(layout.clone(), |ui| ui.label(format!("FPS:{:.0}", fps.abs())));
            ui[1].with_layout(layout.clone(), |ui| ui.label(format!("AVG:{:.4}", avg.abs())));
            ui[2].with_layout(layout, |ui| ui.label(format!("Count:{}", count.abs())));
        });
        ui.end_row();
        ui.separator();
        ui.add(
            Plot::new("fps-plot")
            .include_x(crate::consts::HISTORY_LENGTH as f32)
            .height(50.0)
            .show_x(false)
            .line(Line::new(Values::from_values(
                resources.history.fps.iter().enumerate()
                .map(|(x, y)| Value { x: x as f64, y: *y as f64 })
                .collect::<Vec<Value>>()
            )).color(DraculaEgui::PINK)),
        );
    });
}
