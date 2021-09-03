use crate::{bevy_egui::EguiContext, egui::Ui, DiagnosticPanel, DiagnosticGroup};
use bevy::diagnostic::Diagnostics;
use bevy::prelude::*;

use super::Panel;

pub fn panel() -> Panel {
    Panel::new("🔍").render(draw)
}

pub fn draw(_: &EguiContext, ui: &mut Ui, world: &mut World) {
    let devtools_diagnostics = ignore_none_error!(
        world.get_resource::<DiagnosticPanel>(),
        "Failed to get DiagnosticPanel resource"
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
