use bevy::prelude::*;
use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy_inspector_egui::bevy_egui::{
    egui::widgets::plot::{Line, Plot, Value, Values},
    egui::Ui,
};

use crate::{helpers::DraculaEgui, DevToolsResources};

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
        let mut resources = world.get_resource_mut::<DevToolsResources>().unwrap();
        resources.history.push_fps(fps);
        fps
    };
    let diagnostics = world.get_resource::<Diagnostics>().unwrap();
    let resources = world.get_resource::<DevToolsResources>().unwrap();
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
            ui[0].label(format!("FPS:{:.0}", fps.abs()));
            ui[1].label(format!("AVG:{:.4}", avg.abs()));
            ui[2].label(format!("Count:{}", count.abs()));
        });
        ui.end_row();
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
