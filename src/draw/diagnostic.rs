use bevy::prelude::*;
use bevy::diagnostic::Diagnostics;
use bevy_inspector_egui::egui::Ui;

use crate::{DiagnosticGroup, DevToolsDiagnostics};

pub fn handle_diagnostics(ui: &mut Ui, world: &mut World) {
    let devtools_diagnostics = world.get_resource::<DevToolsDiagnostics>().unwrap();
    let diagnostics = world.get_resource::<Diagnostics>().unwrap();
    for group in devtools_diagnostics.0.iter() {
        display_diagnostic(ui, diagnostics, group);
    }
}

pub fn display_diagnostic(ui: &mut Ui, diagnostics: &Diagnostics, group: &DiagnosticGroup) {
    ui.group(|ui| {
        ui.heading(group.label());
        ui.end_row();
        ui.group(|ui| {
            let length = group.data.len();
            for (dex, group) in group.data.iter().enumerate() {
                ui.columns(group.len(), |ui| {
                    for (dex, data) in group.iter().enumerate() {
                        (data.render)(&mut ui[dex], diagnostics);
                    }
                });
                if dex + 1 != length {
                    ui.separator();
                }
            }
        });
    });
}
