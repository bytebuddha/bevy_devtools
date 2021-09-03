use bevy::diagnostic::Diagnostics;
use bevy_inspector_egui::egui::Ui;

mod assets;
mod ecs;

mod wgpu;

pub struct DiagnosticPanel(pub Vec<DiagnosticGroup>);

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
    pub render: fn(&mut Ui, &Diagnostics),
}

impl Default for DiagnosticPanel {
    fn default() -> DiagnosticPanel {
        DiagnosticPanel(vec![ecs::group(), assets::group(), wgpu::group()])
    }
}
