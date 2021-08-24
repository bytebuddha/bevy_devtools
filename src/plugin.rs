use bevy::diagnostic::{DiagnosticsPlugin, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiStage;

use super::{
    DevToolsDiagnostics, DevToolsSettings, DevToolsTabs, DevToolsTools, PerformToolAction,
};

pub struct DevToolsPlugin;

impl Plugin for DevToolsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let diagnostics = DevToolsDiagnostics::default();
        for group in diagnostics.0.iter() {
            for group in group.data.iter() {
                for data in &group.data {
                    (data.build)(app);
                }
            }
        }
        app.init_resource::<DevToolsTools>()
            .init_resource::<DevToolsSettings>()
            .init_resource::<DevToolsDiagnostics>()
            .init_resource::<DevToolsTabs>()
            .insert_resource(bevy_inspector_egui::world_inspector::WorldInspectorParams::default())
            .init_resource::<bevy_inspector_egui::world_inspector::InspectableRegistry>()
            .add_event::<PerformToolAction>()
            .add_plugin(DiagnosticsPlugin)
            .add_plugin(FrameTimeDiagnosticsPlugin)
            .add_plugin(bevy_inspector_egui::bevy_egui::EguiPlugin)
            .add_system(crate::draw::draw_debug_ui.exclusive_system())
            .add_system(crate::systems::perform_tool_action.exclusive_system())
            .add_system_to_stage(
                EguiStage::UiFrameEnd,
                crate::systems::apply_ui_settings.system(),
            );
    }
}
