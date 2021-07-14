use bevy::prelude::*;
use bevy::diagnostic::Diagnostics;
use bevy::asset::diagnostic::AssetCountDiagnosticsPlugin;
use bevy_inspector_egui::egui::Ui;

use super::{DiagnosticDisplay, DiagnosticGroup};

pub fn group() -> DiagnosticGroup {
    DiagnosticGroup {
        name: "assets".into(),
        label: Some("Assets".into()),
        data: vec![
            vec![
                DiagnosticDisplay {
                    build: build_texture_count,
                    render: render_texture_count
                },
                DiagnosticDisplay {
                    build: build_color_texture_count,
                    render: render_color_texture_count
                },
                DiagnosticDisplay {
                    build: build_materials_count,
                    render: render_materials_count
                }
            ]
        ]
    }
}

fn build_color_texture_count(app: &mut AppBuilder) {
    app.add_plugin(AssetCountDiagnosticsPlugin::<ColorMaterial>::default());
}

fn render_color_texture_count(ui: &mut Ui, diagnostics: &Diagnostics) {
    let textures = diagnostics.get(AssetCountDiagnosticsPlugin::<ColorMaterial>::diagnostic_id()).unwrap().value().unwrap_or(0.0);
    ui.label(format!("Colors: {:.0}", textures));
}


fn build_materials_count(app: &mut AppBuilder) {
    app.add_plugin(AssetCountDiagnosticsPlugin::<StandardMaterial>::default());
}

fn render_materials_count(ui: &mut Ui, diagnostics: &Diagnostics) {
    let materials = diagnostics.get(AssetCountDiagnosticsPlugin::<StandardMaterial>::diagnostic_id()).unwrap().value().unwrap_or(0.0);
    ui.label(format!("Materials: {:.0}", materials));
}

fn build_texture_count(app: &mut AppBuilder) {
    app.add_plugin(AssetCountDiagnosticsPlugin::<Texture>::default());
}

fn render_texture_count(ui: &mut Ui, diagnostics: &Diagnostics) {
    let textures = diagnostics.get(AssetCountDiagnosticsPlugin::<Texture>::diagnostic_id()).unwrap().value().unwrap_or(0.0);
    ui.label(format!("Textures: {:.0}", textures));
}
