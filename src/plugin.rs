use bevy::asset::diagnostic::AssetCountDiagnosticsPlugin;
use bevy::diagnostic::{
    DiagnosticsPlugin, EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin,
};
use bevy::prelude::*;
use bevy::wgpu::diagnostic::WgpuResourceDiagnosticsPlugin;
use bevy_inspector_egui::bevy_egui::EguiStage;
use bevy_inspector_egui::world_inspector::{InspectableRegistry, WorldInspectorParams};

use super::{
    DevToolsDiagnostics, DevToolsSettings, DevToolsPanels, DevToolsTools, PerformToolAction,
};

pub struct DevToolsPlugin;

impl Plugin for DevToolsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<DevToolsTools>()
            .init_resource::<DevToolsSettings>()
            .init_resource::<DevToolsDiagnostics>()
            .init_resource::<DevToolsPanels>()
            .init_resource::<WorldInspectorParams>()
            .init_resource::<InspectableRegistry>()
            .add_event::<PerformToolAction>()
            .add_plugin(DiagnosticsPlugin)
            .add_plugin(FrameTimeDiagnosticsPlugin)
            .add_plugin(bevy_inspector_egui::bevy_egui::EguiPlugin)
            .add_plugin(AssetCountDiagnosticsPlugin::<StandardMaterial>::default())
            .add_plugin(AssetCountDiagnosticsPlugin::<ColorMaterial>::default())
            .add_plugin(AssetCountDiagnosticsPlugin::<Texture>::default())
            .add_plugin(EntityCountDiagnosticsPlugin)
            .add_plugin(WgpuResourceDiagnosticsPlugin)
            .add_system(crate::draw::draw_debug_ui.exclusive_system())
            .add_system(crate::systems::perform_tool_action.exclusive_system())
            .add_system_to_stage(
                EguiStage::UiFrameEnd,
                crate::systems::apply_ui_settings.system(),
            );
    }
}
