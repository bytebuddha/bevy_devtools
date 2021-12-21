use bevy::diagnostic::{
    DiagnosticsPlugin, EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin,
};
use bevy::prelude::*;
use bevy::ecs::component::Component;
use bevy_inspector_egui::world_inspector::{InspectableRegistry, WorldInspectorParams};

use std::fmt::Debug;
use std::hash::Hash;

use super::{
    DiagnosticPanel, Settings, Panels, Tools, PerformToolAction,
};

#[derive(Debug, Default, PartialEq, Clone, Eq, Hash, Component)]
pub struct NoState;

#[derive(Default)]
pub struct DevToolsPlugin<T: Debug + Clone + Eq + Hash + Component = NoState>(std::marker::PhantomData<T>);

impl <T: Debug + Clone + Eq + Hash + Component>Plugin for DevToolsPlugin<T> {
    fn build(&self, app: &mut App) {
        app.init_resource::<Tools>()
            .init_resource::<Settings>()
            .init_resource::<DiagnosticPanel>()
            .init_resource::<Panels>()
            .init_resource::<WorldInspectorParams>()
            .init_resource::<InspectableRegistry>()
            .add_event::<PerformToolAction>()
            .add_plugin(DiagnosticsPlugin)
            .add_plugin(FrameTimeDiagnosticsPlugin)
            .add_plugin(bevy_inspector_egui::bevy_egui::EguiPlugin)
            .add_plugin(EntityCountDiagnosticsPlugin)
            .add_system(crate::draw::draw_debug_ui::<T>.exclusive_system())
            .add_system(crate::systems::perform_tool_action.exclusive_system())
            .add_system(crate::systems::apply_ui_settings);
    }
}
