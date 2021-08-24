use bevy::asset::diagnostic::AssetCountDiagnosticsPlugin;
use bevy::diagnostic::Diagnostics;
use bevy::prelude::*;
use bevy_inspector_egui::egui::Ui;

use super::{DiagnosticDisplay, DiagnosticGroup, DiagnosticRow};

pub fn group() -> DiagnosticGroup {
    DiagnosticGroup {
        name: "assets".into(),
        label: Some("Assets".into()),
        data: vec![DiagnosticRow {
            name: None,
            data: vec![
                DiagnosticDisplay {
                    render: render_texture_count,
                },
                DiagnosticDisplay {
                    render: render_color_texture_count,
                },
                DiagnosticDisplay {
                    render: render_materials_count,
                },
            ],
        }],
    }
}
fn render_color_texture_count(ui: &mut Ui, diagnostics: &Diagnostics) {
    let textures = diagnostic_value!(
        diagnostics,
        AssetCountDiagnosticsPlugin::<ColorMaterial>::diagnostic_id()
    );
    ui.label(format!("Colors: {:.0}", textures));
}

fn render_materials_count(ui: &mut Ui, diagnostics: &Diagnostics) {
    let materials = diagnostic_value!(
        diagnostics,
        AssetCountDiagnosticsPlugin::<StandardMaterial>::diagnostic_id()
    );
    ui.label(format!("Materials: {:.0}", materials));
}

fn render_texture_count(ui: &mut Ui, diagnostics: &Diagnostics) {
    let textures = diagnostic_value!(
        diagnostics,
        AssetCountDiagnosticsPlugin::<Texture>::diagnostic_id()
    );
    ui.label(format!("Textures: {:.0}", textures));
}
