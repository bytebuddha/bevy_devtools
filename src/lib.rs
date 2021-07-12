pub extern crate bevy_inspector_egui;

pub mod draw;
pub mod consts;
pub mod systems;
pub mod helpers;

mod plugin;
pub use self::plugin::DevToolsPlugin;

mod tools;
pub use self::tools::{DevToolsTools, DevTool, PerformToolAction};

mod settings;
pub use self::settings::{DevToolsSettings, DevToolsSetting};

mod resources;
pub use self::resources::{DevToolsResources, SaveWorldRequest};

mod diagnostics;
pub use self::diagnostics::{DevToolsDiagnostics, DiagnosticGroup, DiagnosticDisplay};

pub const HISTORY_LENGTH: usize = 100;
