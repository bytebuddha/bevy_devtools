use bevy::diagnostic::Diagnostics;
use bevy::prelude::*;
use bevy_inspector_egui::egui::Ui;

mod assets;
mod ecs;

#[cfg(feature = "wgpu")]
mod wgpu;

pub struct DevToolsDiagnostics(pub Vec<DiagnosticGroup>);

pub struct DiagnosticGroup {
    pub name: String,
    pub label: Option<String>,
    pub data: Vec<DiagnosticRow>,
}

impl DiagnosticGroup {
    pub fn label(&self) -> &str {
        self.label.as_ref().unwrap_or(&self.name)
    }
}

pub struct DiagnosticRow {
    pub name: Option<String>,
    pub data: Vec<DiagnosticDisplay>,
}

pub struct DiagnosticDisplay {
    pub build: fn(&mut AppBuilder),
    pub render: fn(&mut Ui, &Diagnostics),
}

impl Default for DevToolsDiagnostics {
    fn default() -> DevToolsDiagnostics {
        DevToolsDiagnostics(vec![
            ecs::group(),
            assets::group(),
            #[cfg(feature = "wgpu")]
            wgpu::group(),
        ])
    }
}
