use bevy::diagnostic::Diagnostics;
use bevy::wgpu::diagnostic::WgpuResourceDiagnosticsPlugin;
use bevy_inspector_egui::egui::Ui;

use super::{DiagnosticDisplay, DiagnosticGroup, DiagnosticRow};

pub fn group() -> DiagnosticGroup {
    DiagnosticGroup {
        name: "wgpu".into(),
        label: Some("WGPU".into()),
        data: vec![
            DiagnosticRow {
                name: None,
                data: vec![
                    DiagnosticDisplay {
                        render: render_bind_groups,
                    },
                    DiagnosticDisplay {
                        render: render_bind_group_ids,
                    },
                    DiagnosticDisplay {
                        render: render_bind_group_layouts,
                    },
                ],
            },
            DiagnosticRow {
                name: None,
                data: vec![
                    DiagnosticDisplay {
                        render: render_buffers,
                    },
                    DiagnosticDisplay {
                        render: render_render_pipelines,
                    },
                ],
            },
            DiagnosticRow {
                name: None,
                data: vec![
                    DiagnosticDisplay {
                        render: render_samplers,
                    },
                    DiagnosticDisplay {
                        render: render_shader_modules,
                    },
                ],
            },
            DiagnosticRow {
                name: None,
                data: vec![
                    DiagnosticDisplay {
                        render: render_wgpu_swapchains,
                    },
                    DiagnosticDisplay {
                        render: render_wgpu_textures,
                    },
                ],
            },
            DiagnosticRow {
                name: None,
                data: vec![
                    DiagnosticDisplay {
                        render: render_wgpu_textures_views,
                    },
                    DiagnosticDisplay {
                        render: render_wgpu_surfaces,
                    },
                ],
            },
        ],
    }
}

fn render_bind_groups(ui: &mut Ui, diagnostics: &Diagnostics) {
    let groups = diagnostic_value!(diagnostics, WgpuResourceDiagnosticsPlugin::BIND_GROUPS);
    ui.label(format!("Groups: {:.0}", groups));
}

fn render_bind_group_ids(ui: &mut Ui, diagnostics: &Diagnostics) {
    let groups = diagnostic_value!(diagnostics, WgpuResourceDiagnosticsPlugin::BIND_GROUP_IDS);
    ui.label(format!("Ids: {:.0}", groups));
}

fn render_bind_group_layouts(ui: &mut Ui, diagnostics: &Diagnostics) {
    let groups = diagnostic_value!(
        diagnostics,
        WgpuResourceDiagnosticsPlugin::BIND_GROUP_LAYOUTS
    );
    ui.label(format!("Layouts: {:.0}", groups));
}

fn render_buffers(ui: &mut Ui, diagnostics: &Diagnostics) {
    let groups = diagnostic_value!(diagnostics, WgpuResourceDiagnosticsPlugin::BUFFERS);
    ui.label(format!("Buffers: {:.0}", groups));
}

fn render_samplers(ui: &mut Ui, diagnostics: &Diagnostics) {
    let groups = diagnostic_value!(diagnostics, WgpuResourceDiagnosticsPlugin::SAMPLERS);
    ui.label(format!("Samplers: {:.0}", groups));
}

fn render_render_pipelines(ui: &mut Ui, diagnostics: &Diagnostics) {
    let groups = diagnostic_value!(diagnostics, WgpuResourceDiagnosticsPlugin::RENDER_PIPELINES);
    ui.label(format!("Groups: {:.0}", groups));
}

fn render_shader_modules(ui: &mut Ui, diagnostics: &Diagnostics) {
    let groups = diagnostic_value!(
        diagnostics,
        WgpuResourceDiagnosticsPlugin::SWAP_CHAIN_OUTPUTS
    );
    ui.label(format!("Shaders: {:.0}", groups));
}

fn render_wgpu_textures(ui: &mut Ui, diagnostics: &Diagnostics) {
    let textures = diagnostic_value!(diagnostics, WgpuResourceDiagnosticsPlugin::TEXTURES);
    ui.label(format!("Textures: {:.0}", textures));
}

fn render_wgpu_textures_views(ui: &mut Ui, diagnostics: &Diagnostics) {
    let textures = diagnostic_value!(diagnostics, WgpuResourceDiagnosticsPlugin::TEXTURE_VIEWS);
    ui.label(format!("Views: {:.0}", textures));
}

fn render_wgpu_surfaces(ui: &mut Ui, diagnostics: &Diagnostics) {
    let textures = diagnostic_value!(diagnostics, WgpuResourceDiagnosticsPlugin::WINDOW_SURFACES);
    ui.label(format!("Surfaces: {:.0}", textures));
}

fn render_wgpu_swapchains(ui: &mut Ui, diagnostics: &Diagnostics) {
    let textures = diagnostic_value!(diagnostics, WgpuResourceDiagnosticsPlugin::SWAP_CHAINS);
    let output = diagnostic_value!(
        diagnostics,
        WgpuResourceDiagnosticsPlugin::SWAP_CHAIN_OUTPUTS
    );
    ui.label(format!("Swap Chains: {:.0}/{:.0}", textures, output));
}
