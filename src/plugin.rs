use bevy::asset::diagnostic::AssetCountDiagnosticsPlugin;
use bevy::diagnostic::{
    DiagnosticsPlugin, EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin,
};
use bevy::prelude::*;
use bevy_inspector_egui::{
    bevy_egui::{EguiContext, EguiStage},
    WorldInspectorParams, WorldInspectorPlugin,
};

use super::DevToolsEvent;

pub struct DevToolsPlugin;

impl Plugin for DevToolsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<super::DevToolsResources>()
            .init_resource::<super::SaveWorldRequest>()
            .add_event::<DevToolsEvent>()
            .add_plugin(DiagnosticsPlugin)
            .add_plugin(FrameTimeDiagnosticsPlugin)
            .add_plugin(EntityCountDiagnosticsPlugin)
            .insert_resource(WorldInspectorParams {
                enabled: false,
                ..Default::default()
            })
            .add_plugin(WorldInspectorPlugin::new())
//            .add_plugin(AssetCountDiagnosticsPlugin::<Mesh>::default())
            .add_plugin(AssetCountDiagnosticsPlugin::<StandardMaterial>::default())
            .add_plugin(AssetCountDiagnosticsPlugin::<ColorMaterial>::default())
            .add_plugin(AssetCountDiagnosticsPlugin::<Texture>::default())
            .add_system(handle_devtools_event.system())
            .add_system(super::draw::draw_debug_ui.system())
            .add_system(save_scene.exclusive_system())
            .add_system(toggle_debug.system())
            .add_system_to_stage(EguiStage::UiFrameEnd, apply_ui_settings.system());
    }
}

pub fn toggle_debug(
    inspector: Res<WorldInspectorParams>,
    keys: Res<Input<KeyCode>>,
    mut events: EventWriter<DevToolsEvent>
) {
    if keys.just_pressed(KeyCode::F11) {
        if inspector.enabled {
            events.send(DevToolsEvent::Stop);
        } else {
            events.send(DevToolsEvent::Start);
        }
    }
}

fn handle_devtools_event(
    context: Res<EguiContext>,
    resources: Res<super::DevToolsResources>,
    mut inspector: ResMut<WorldInspectorParams>,
    mut events: EventReader<super::DevToolsEvent>
) {
    for event in events.iter() {
        match event {
            DevToolsEvent::Start => {
                context.ctx().set_style(super::DraculaEgui::style());
                if !resources.always_visible {
                    inspector.enabled = true;
                }
            },
            DevToolsEvent::Stop => {
                if !resources.always_visible {
                    inspector.enabled = false;
                }
            }
        }
    }
}

use std::fs::File;
use std::io::Write;

fn save_scene(world: &mut World) {
    let scene_name = {
        let mut item = world.get_resource_mut::<super::SaveWorldRequest>().unwrap();
        let value = item.0.clone();
        item.0 = None;
        value
    };
    if let Some(name) = scene_name {
        let mut file = File::create(&name).unwrap();
        let type_registry = world.get_resource::<bevy::reflect::TypeRegistry>().unwrap();
        let scene = DynamicScene::from_world(&world, &type_registry);
        let scene_data = scene.serialize_ron(&type_registry).unwrap();
        file.write_all(scene_data.as_bytes()).unwrap();
    }
}

fn apply_ui_settings(resources: Res<super::DevToolsResources>, context: ResMut<EguiContext>) {
    let ctx = context.ctx();
    let mut style = (*ctx.style()).clone();
    style.debug.show_widgets = resources.egui.show_widgets_on_hover;
    style.debug.show_expand_width = resources.egui.expand_width;
    style.debug.show_expand_height = resources.egui.expand_height;
    style.debug.show_resize = resources.egui.show_resize;
    ctx.set_style(style);
}
