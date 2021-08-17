use bevy::diagnostic::Diagnostics;
use bevy::diagnostic::EntityCountDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_inspector_egui::egui::Ui;

use super::{DiagnosticDisplay, DiagnosticGroup, DiagnosticRow};

pub fn render_entity_count(ui: &mut Ui, diagnostics: &Diagnostics) {
    let entities = diagnostic_value!(diagnostics, EntityCountDiagnosticsPlugin::ENTITY_COUNT);
    ui.label(format!("Entities: {:.0}", entities));
}

pub fn build_entity_count(app: &mut AppBuilder) {
    app.add_plugin(EntityCountDiagnosticsPlugin);
}

pub fn group() -> super::DiagnosticGroup {
    DiagnosticGroup {
        name: "ecs".into(),
        label: Some("ECS".into()),
        data: vec![
            DiagnosticRow {
                name: None,
                data: vec![
                    DiagnosticDisplay {
                        build: build_entity_count,
                        render: render_entity_count
                    }
                ]
            }
        ]
    }
}
