use bevy::diagnostic::Diagnostics;
use bevy::diagnostic::EntityCountDiagnosticsPlugin;
use bevy_inspector_egui::egui::Ui;

use super::{DiagnosticDisplay, DiagnosticGroup, DiagnosticRow};

pub fn render_entity_count(ui: &mut Ui, diagnostics: &Diagnostics) {
    let entities = diagnostic_value!(diagnostics, EntityCountDiagnosticsPlugin::ENTITY_COUNT);
    ui.label(format!("Entities: {:.0}", entities));
}

pub fn group() -> super::DiagnosticGroup {
    DiagnosticGroup {
        name: "ecs".into(),
        label: Some("ECS".into()),
        data: vec![DiagnosticRow {
            name: None,
            data: vec![DiagnosticDisplay {
                render: render_entity_count,
            }],
        }],
    }
}
