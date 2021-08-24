use bevy::diagnostic::{DiagnosticsPlugin, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiStage;

use super::{
    DevToolsDiagnostics, DevToolsLocation, DevToolsSettings, DevToolsState, DevToolsTabs,
    DevToolsTools, PerformToolAction,
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
            .insert_resource(DevToolsState {
                location: DevToolsLocation::Window,
                toggle_key: KeyCode::F11,
                #[cfg(feature = "puffin")]
                profiler_key: KeyCode::F12,
                active_tab: Default::default(),
                history: Default::default(),
            })
            .insert_resource(bevy_inspector_egui::world_inspector::WorldInspectorParams::default())
            .init_resource::<bevy_inspector_egui::world_inspector::InspectableRegistry>()
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

        #[cfg(feature = "rapier3d")]
        {
            app.add_system(crate::systems::rapier::rapier_settings.system())
                .add_startup_system(crate::systems::rapier::initial_rapier_settings.system());
        }
        #[cfg(feature = "puffin")]
        {
            app.add_system_to_stage(
                bevy::app::CoreStage::First,
                (|| {
                    puffin_profiler::GlobalProfiler::lock().new_frame();
                })
                .system(),
            )
            .add_startup_system(crate::systems::puffin::initialize_puffin.system());
        }
    }
}
