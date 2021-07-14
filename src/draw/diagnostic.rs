use bevy::diagnostic::Diagnostics;
use bevy_inspector_egui::egui::Ui;

use crate::DiagnosticGroup;

pub fn display_diagnostic(ui: &mut Ui, diagnostics: &Diagnostics, group: &DiagnosticGroup) {
    ui.group(|ui| {
        ui.heading(group.label.as_ref().unwrap_or(&group.name));
        ui.end_row();
        ui.group(|ui| {
            let length = group.data.len();
            for (dex, group) in group.data.iter().enumerate() {
                ui.columns(group.len(), |ui| {
                    for (dex, data) in group.iter().enumerate() {
                        (data.render)(&mut ui[dex], &diagnostics);
                    }
                });
                if dex + 1 != length {
                    ui.separator();
                }
            }
        });
    });
}
