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
    let groups = diagnostics
        .get(WgpuResourceDiagnosticsPlugin::BIND_GROUPS)
        .unwrap()
        .value()
        .unwrap_or(0.0);
    ui.label(format!("Groups: {:.0}", groups));
}

fn render_bind_group_ids(ui: &mut Ui, diagnostics: &Diagnostics) {
    let groups = diagnostics
        .get(WgpuResourceDiagnosticsPlugin::BIND_GROUP_IDS)
        .unwrap()
        .value()
        .unwrap_or(0.0);
    ui.label(format!("Ids: {:.0}", groups));
}

fn render_bind_group_layouts(ui: &mut Ui, diagnostics: &Diagnostics) {
    let groups = diagnostics
        .get(WgpuResourceDiagnosticsPlugin::BIND_GROUP_LAYOUTS)
        .unwrap()
        .value()
        .unwrap_or(0.0);
    ui.label(format!("Layouts: {:.0}", groups));
}

fn render_buffers(ui: &mut Ui, diagnostics: &Diagnostics) {
    let groups = diagnostics
        .get(WgpuResourceDiagnosticsPlugin::BUFFERS)
        .unwrap()
        .value()
        .unwrap_or(0.0);
    ui.label(format!("Buffers: {:.0}", groups));
}

fn render_samplers(ui: &mut Ui, diagnostics: &Diagnostics) {
    let groups = diagnostics
        .get(WgpuResourceDiagnosticsPlugin::SAMPLERS)
        .unwrap()
        .value()
        .unwrap_or(0.0);
    ui.label(format!("Samplers: {:.0}", groups));
}

fn render_render_pipelines(ui: &mut Ui, diagnostics: &Diagnostics) {
    let groups = diagnostics
        .get(WgpuResourceDiagnosticsPlugin::RENDER_PIPELINES)
        .unwrap()
        .value()
        .unwrap_or(0.0);
    ui.label(format!("Groups: {:.0}", groups));
}

fn render_shader_modules(ui: &mut Ui, diagnostics: &Diagnostics) {
    let groups = diagnostics
        .get(WgpuResourceDiagnosticsPlugin::SHADER_MODULES)
        .unwrap()
        .value()
        .unwrap_or(0.0);
    ui.label(format!("Shaders: {:.0}", groups));
}

fn render_wgpu_textures(ui: &mut Ui, diagnostics: &Diagnostics) {
    let textures = diagnostics
        .get(WgpuResourceDiagnosticsPlugin::TEXTURES)
        .unwrap()
        .value()
        .unwrap_or(0.0);
    ui.label(format!("Textures: {:.0}", textures));
}

fn render_wgpu_textures_views(ui: &mut Ui, diagnostics: &Diagnostics) {
    let textures = diagnostics
        .get(WgpuResourceDiagnosticsPlugin::TEXTURE_VIEWS)
        .unwrap()
        .value()
        .unwrap_or(0.0);
    ui.label(format!("Views: {:.0}", textures));
}

fn render_wgpu_surfaces(ui: &mut Ui, diagnostics: &Diagnostics) {
    let textures = diagnostics
        .get(WgpuResourceDiagnosticsPlugin::WINDOW_SURFACES)
        .unwrap()
        .value()
        .unwrap_or(0.0);
    ui.label(format!("Surfaces: {:.0}", textures));
}

fn render_wgpu_swapchains(ui: &mut Ui, diagnostics: &Diagnostics) {
    let textures = diagnostics
        .get(WgpuResourceDiagnosticsPlugin::SWAP_CHAINS)
        .unwrap()
        .value()
        .unwrap_or(0.0);
    let output = diagnostics
        .get(WgpuResourceDiagnosticsPlugin::SWAP_CHAIN_OUTPUTS)
        .unwrap()
        .value()
        .unwrap_or(0.0);
    ui.label(format!("Swap Chains: {:.0}/{:.0}", textures, output));
}
