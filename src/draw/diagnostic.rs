use bevy::diagnostic::Diagnostics;
use bevy::prelude::*;
use bevy_inspector_egui::egui::Ui;

use crate::{DevToolsDiagnostics, DiagnosticGroup};

pub fn handle_diagnostics(ui: &mut Ui, world: &mut World) {
    #[cfg(feature = "puffin")] puffin_profiler::profile_function!();
    let devtools_diagnostics = ignore_none_error!(
        world.get_resource::<DevToolsDiagnostics>(),
        "Failed to get DevToolDiagnostics resource"
    );
    let diagnostics = ignore_none_error!(
        world.get_resource::<Diagnostics>(),
        "Failed to get Diagnostics resource"
    );
    for group in devtools_diagnostics.0.iter() {
        display_diagnostic(ui, diagnostics, group);
    }
}

pub fn display_diagnostic(ui: &mut Ui, diagnostics: &Diagnostics, group: &DiagnosticGroup) {
    #[cfg(feature = "puffin")] puffin_profiler::profile_function!();
    ui.group(|ui| {
        ui.heading(group.label());
        ui.end_row();
        let length = group.data.len();
        for (dex, group) in group.data.iter().enumerate() {
            if let Some(name) = group.name.as_ref() {
                ui.heading(name);
                ui.end_row();
            }
            ui.columns(group.data.len(), |ui| {
                for (dex, data) in group.data.iter().enumerate() {
                    (data.render)(&mut ui[dex], diagnostics);
                }
            });
            if dex + 1 != length {
                ui.separator();
            }
        }
    });
}
