use bevy::diagnostic::Diagnostics;
use bevy_inspector_egui::egui::Ui;

use crate::DiagnosticGroup;

pub fn display_diagnostic(ui: &mut Ui, diagnostics: &Diagnostics, group: &DiagnosticGroup) {
    ui.group(|ui| {
        ui.heading(group.label.as_ref().unwrap_or(&group.name));
        ui.end_row();
        for group in group.data.iter() {
            ui.columns(group.len(), |ui| {
                for (dex, data) in group.iter().enumerate() {
                    (data.render)(&mut ui[dex], &diagnostics);
                }
            });
        }
    });
}
