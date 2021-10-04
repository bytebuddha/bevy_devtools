use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::egui::{self, Ui};
use bevy::ecs::component::Component;

use std::fmt::Debug;
use std::hash::Hash;

pub fn top_panel<T: Debug + Clone + Eq + Hash + Component>(ui: &mut Ui, world: &mut World) {
    let diagnostics = ignore_none_error!(
        world.get_resource::<Diagnostics>(),
        "Failed to get Diagnostics resource"
    );
    let fps = diagnostic_value!(diagnostics, FrameTimeDiagnosticsPlugin::FPS);
    let avg = diagnostic_value!(diagnostics, FrameTimeDiagnosticsPlugin::FRAME_TIME);
    let count = diagnostic_value!(diagnostics, FrameTimeDiagnosticsPlugin::FRAME_COUNT);
    ui.group(|ui| {
        let layout = egui::Layout::top_down(egui::Align::Center);
        if let Some(state) = world.get_resource::<State<T>>() {
            ui.columns(2, |ui| {
                ui[0].with_layout(layout.clone(), |ui| {
                    ui.label(format!("{:?}", state.current()));
                });
                ui[1].with_layout(layout, |ui| {
                    ui.label(format!("⏱: {:.0}/{:.3} 🖩 {:.0}", fps.abs(), avg.abs(), count.abs()));
                });
            });
        } else {
            ui.with_layout(layout, |ui| {
                ui.label(format!("⏱: {:.0}/{:.3} 🖩 {:.0}", fps.abs(), avg.abs(), count.abs()));
            });
        }
    });
}
