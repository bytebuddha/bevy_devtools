pub extern crate bevy_inspector_egui;

pub mod draw;

mod style;
pub use self::style::DraculaEgui;

mod history;
pub use self::history::History;

mod plugin;
pub use self::plugin::DevToolsPlugin;

mod events;
pub use self::events::DevToolsEvent;

mod resources;
pub use self::resources::{DevToolsResources, SaveWorldRequest};

pub const HISTORY_LENGTH: usize = 100;
