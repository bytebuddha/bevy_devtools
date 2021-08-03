use bevy::diagnostic::Diagnostics;
use bevy::prelude::*;
use bevy::wgpu::diagnostic::WgpuResourceDiagnosticsPlugin;
use bevy_inspector_egui::egui::Ui;

use super::{DiagnosticDisplay, DiagnosticGroup};

pub fn group() -> DiagnosticGroup {
    DiagnosticGroup {
        name: "wgpu".into(),
        label: Some("WGPU".into()),
        data: vec![
            vec![
                DiagnosticDisplay {
                    build: build_wgpu,
                    render: render_bind_groups,
                },
                DiagnosticDisplay {
                    build: build_wgpu,
                    render: render_bind_group_ids,
                },
                DiagnosticDisplay {
                    build: build_wgpu,
                    render: render_bind_group_layouts,
                },
            ],
            vec![
                DiagnosticDisplay {
                    build: build_wgpu,
                    render: render_buffers,
                },
                DiagnosticDisplay {
                    build: build_wgpu,
                    render: render_render_pipelines,
                },
            ],
            vec![
                DiagnosticDisplay {
                    build: build_wgpu,
                    render: render_samplers,
                },
                DiagnosticDisplay {
                    build: build_wgpu,
                    render: render_shader_modules,
                },
            ],
            vec![
                DiagnosticDisplay {
                    build: build_wgpu,
                    render: render_wgpu_swapchains,
                },
                DiagnosticDisplay {
                    build: build_wgpu,
                    render: render_wgpu_textures,
                },
            ],
            vec![
                DiagnosticDisplay {
                    build: build_wgpu,
                    render: render_wgpu_textures_views,
                },
                DiagnosticDisplay {
                    build: build_wgpu,
                    render: render_wgpu_surfaces,
                },
            ],
        ],
    }
}

fn build_wgpu(app: &mut AppBuilder) {
    app.add_plugin(WgpuResourceDiagnosticsPlugin);
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
