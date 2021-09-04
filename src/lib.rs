pub extern crate bevy_inspector_egui;
pub use bevy_inspector_egui::bevy_egui;
pub use bevy_inspector_egui::bevy_egui::egui;

#[macro_use]
mod macros;
pub(crate) mod draw;
pub(crate) mod systems;

mod plugin;
pub use self::plugin::DevToolsPlugin;

mod app;
pub use self::app::DevToolsExt;

mod tools;
pub use self::tools::{Tool, Tools, PerformToolAction};

mod settings;
pub use self::settings::{Setting, Settings, SettingValue};

mod panels;
pub use self::panels::{Panel, Panels, PanelLocation};

mod diagnostics;
pub use self::diagnostics::{DiagnosticPanel, DiagnosticDisplay, DiagnosticGroup};

pub struct TopPanel(pub fn(&mut egui::Ui, &mut bevy::prelude::World));
