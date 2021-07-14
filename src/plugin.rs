use bevy::diagnostic::{DiagnosticsPlugin, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiStage, WorldInspectorParams, WorldInspectorPlugin};

use super::{
    DevToolsDiagnostics, DevToolsResources, DevToolsSettings, DevToolsTools, PerformToolAction,
};

pub struct DevToolsPlugin;

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
        app.init_resource::<DevToolsResources>()
            .init_resource::<DevToolsSettings>()
            .init_resource::<DevToolsTools>()
            .insert_resource(diagnostics)
            .insert_resource(WorldInspectorParams {
                enabled: false,
                ..Default::default()
            })
            .add_event::<PerformToolAction>()
            .add_plugin(DiagnosticsPlugin)
            .add_plugin(FrameTimeDiagnosticsPlugin)
            .add_plugin(WorldInspectorPlugin::new())
            .add_system(crate::draw::draw_debug_ui.system())
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
