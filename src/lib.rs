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
pub use self::tools::{DevTool, DevToolsTools, PerformToolAction};

mod settings;
pub use self::settings::{DevToolsSetting, DevToolsSettings, SettingValue};

mod tabs;
pub use self::tabs::{DevToolsTab, DevToolsTabs};

mod diagnostics;
pub use self::diagnostics::{DevToolsDiagnostics, DiagnosticDisplay, DiagnosticGroup};
