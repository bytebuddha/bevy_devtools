pub extern crate bevy_inspector_egui;
pub use bevy_inspector_egui::bevy_egui;
pub use bevy_inspector_egui::bevy_egui::egui;

pub mod consts;
pub mod draw;
pub mod helpers;
pub mod systems;

mod plugin;
pub use self::plugin::DevToolsPlugin;

mod tools;
pub use self::tools::{DevTool, DevToolsTools, PerformToolAction};

mod settings;
pub use self::settings::{DevToolsSetting, DevToolsSettings, SettingValue};

mod resources;
pub(crate) use self::resources::DevToolsResources;

mod diagnostics;
pub use self::diagnostics::{DevToolsDiagnostics, DiagnosticDisplay, DiagnosticGroup};
