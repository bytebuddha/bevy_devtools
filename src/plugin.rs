use bevy::diagnostic::{DiagnosticsPlugin, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiStage;

use super::{
    DevToolsDiagnostics, DevToolsState, DevToolsSettings, DevToolsTools,
    PerformToolAction, DevTool, DevToolsLocation
};

pub struct DevToolsPlugin {
    location: DevToolsLocation,
    toggle_key: KeyCode,
    active_tab: crate::helpers::Tab,
    settings: DevToolsSettings,
    tools: DevToolsTools
}

impl Default for DevToolsPlugin {
    fn default() -> DevToolsPlugin {
        DevToolsPlugin {
            location: DevToolsLocation::Window,
            toggle_key: KeyCode::F11,
            active_tab: crate::helpers::Tab::default(),
            settings: Default::default(),
            tools: Default::default()
        }
    }
}

impl DevToolsPlugin {

    pub fn add_tool(mut self, tool: DevTool) -> DevToolsPlugin {
        self.tools.0.push(tool);
        self
    }

    pub fn location(mut self, location: DevToolsLocation) -> DevToolsPlugin {
        self.location = location;
        self
    }

    pub fn remove_tool(mut self, name: &str) -> DevToolsPlugin {
        let mut index = None;
        for (dex, tool) in self.tools.0.iter().enumerate() {
            if &tool.name == name {
                index = Some(dex);
            }
        }
        if let Some(index) = index {
            self.tools.0.remove(index);
        }
        self
    }
}

impl Plugin for DevToolsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let diagnostics = DevToolsDiagnostics::default();
        for group in diagnostics.0.iter() {
            for group in group.data.iter() {
                for data in group {
                    (data.build)(app);
                }
            }
        }
        app.insert_resource(self.tools.clone())
            .insert_resource(self.settings.clone())
            .insert_resource(diagnostics)
            .insert_resource(DevToolsState {
                location: self.location,
                toggle_key: self.toggle_key,
                active_tab: self.active_tab,
                history: Default::default()
            })
            .insert_resource(crate::world::WorldInspectorParams::default())
            .init_resource::<crate::world::InspectableRegistry>()
            .add_event::<PerformToolAction>()
            .add_plugin(DiagnosticsPlugin)
            .add_plugin(FrameTimeDiagnosticsPlugin)
            .add_plugin(bevy_inspector_egui::bevy_egui::EguiPlugin)
            .add_system(crate::draw::draw_debug_ui.exclusive_system())
            .add_system(crate::systems::perform_tool_action.exclusive_system())
            .add_system(crate::systems::toggle_devtools.system())
            .add_system_to_stage(
                EguiStage::UiFrameEnd,
                crate::systems::apply_ui_settings.system(),
            );

        #[cfg(feature = "rapier")]
        {
            app.add_system(crate::systems::rapier::rapier_settings.system())
                .add_startup_system(crate::systems::rapier::initial_rapier_settings.system());
        }
    }
}
