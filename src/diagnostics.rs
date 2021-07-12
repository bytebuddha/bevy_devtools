use bevy::prelude::*;
use bevy_inspector_egui::egui::Ui;
use bevy::diagnostic::Diagnostics;
use bevy::diagnostic::EntityCountDiagnosticsPlugin;
use bevy::asset::diagnostic::AssetCountDiagnosticsPlugin;

pub struct DevToolsDiagnostics(pub Vec<DiagnosticGroup>);

pub struct DiagnosticGroup {
    pub name: String,
    pub label: Option<String>,
    pub data: Vec<Vec<DiagnosticDisplay>>
}

pub struct DiagnosticDisplay {
    pub build: fn(&mut AppBuilder),
    pub render: fn(&mut Ui, &Diagnostics)
}

impl Default for DevToolsDiagnostics {
    fn default() -> DevToolsDiagnostics {
        DevToolsDiagnostics(vec![
            DiagnosticGroup {
                name: "ecs".into(),
                label: Some("ECS".into()),
                data: vec![
                    vec![
                         DiagnosticDisplay {
                             build: build_entity_count,
                             render: render_entity_count
                         }
                    ]
                ]
            },
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
        ])
    }
}

fn build_entity_count(app: &mut AppBuilder) {
    app.add_plugin(EntityCountDiagnosticsPlugin);
}

fn render_entity_count(ui: &mut Ui, diagnostics: &Diagnostics) {
    let entities = diagnostics
        .get(EntityCountDiagnosticsPlugin::ENTITY_COUNT)
        .unwrap()
        .value()
        .unwrap_or(0.0);

    ui.label(format!("Entities: {:.0}", entities));
}

fn build_texture_count(app: &mut AppBuilder) {
    app.add_plugin(AssetCountDiagnosticsPlugin::<Texture>::default());
}

fn render_texture_count(ui: &mut Ui, diagnostics: &Diagnostics) {
    let textures = diagnostics.get(AssetCountDiagnosticsPlugin::<Texture>::diagnostic_id()).unwrap().value().unwrap_or(0.0);
    ui.label(format!("Textures: {:.0}", textures));
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
    let materials = diagnostics.get(AssetCountDiagnosticsPlugin::<ColorMaterial>::diagnostic_id()).unwrap().value().unwrap_or(0.0);
    ui.label(format!("Materials: {:.0}", materials));
}
