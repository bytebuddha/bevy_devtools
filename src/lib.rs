pub extern crate bevy_inspector_egui;
pub use bevy_inspector_egui::bevy_egui;
pub use bevy_inspector_egui::bevy_egui::egui;

#[cfg(feature = "puffin")]
pub extern crate puffin_profiler as puffin;
#[cfg(feature = "puffin")]
pub use puffin::profile_function;
#[cfg(feature = "puffin")]
pub use puffin::profile_scope;

#[macro_use]
pub mod helpers;
pub mod consts;
pub mod draw;
pub mod systems;

mod plugin;
pub use self::plugin::DevToolsPlugin;

mod tools;
pub use self::tools::{DevTool, DevToolsTools, PerformToolAction};

mod settings;
pub use self::settings::{DevToolsSetting, DevToolsSettings, SettingValue};

mod state;
pub use self::state::{DevToolsLocation, DevToolsState};

mod diagnostics;
pub use self::diagnostics::{DevToolsDiagnostics, DiagnosticDisplay, DiagnosticGroup};
