use bevy::asset::diagnostic::AssetCountDiagnosticsPlugin;
use bevy::diagnostic::{Diagnostics, EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::{
    egui,
    egui::widgets::plot::{Curve, Plot},
    EguiContext,
};

use super::DraculaEgui;

pub fn draw_debug_ui(
    context: Res<EguiContext>,
    diagnostics: Res<Diagnostics>,
    mut save_world: ResMut<super::SaveWorldRequest>,
    mut resources: ResMut<super::DevToolsResources>,
//    mut rapier_enable_events: EventWriter<rapier_3debug::RapierRenderEnable>,
//    mut rapier_disable_events: EventWriter<rapier_3debug::RapierRenderDisable>,
    mut inspector: ResMut<bevy_inspector_egui::WorldInspectorParams>,
) {
    if resources.always_visible || inspector.enabled {
        let mut pos = context.ctx().available_rect().right_top();
        pos.y += 15.0;
        pos.x -= 370.0;
        egui::Window::new("DevTools")
            .default_pos(pos)
            .enabled(inspector.enabled)
            .collapsible(true)
            .show(context.ctx(), |ui| {
                ui.columns(1, |ui| {
                    ui[0].checkbox(&mut resources.always_visible, "Always Visible");
                });
                ui.columns(3, |ui| {
                    if ui[0]
                        .selectable_label(resources.active_tab == 0, "🔍 Diagnostics")
                        .clicked()
                    {
                        resources.active_tab = 0;
                    }
                    if ui[1]
                        .selectable_label(resources.active_tab == 1, "🛠 Tools")
                        .clicked()
                    {
                        resources.active_tab = 1;
                    }
                    if ui[2]
                        .selectable_label(resources.active_tab == 2, "⚙ Settings")
                        .clicked()
                    {
                        resources.active_tab = 2;
                    }
                });
                ui.end_row();
                match resources.active_tab {
                    0 => {
                        let fps = diagnostics
                            .get(FrameTimeDiagnosticsPlugin::FPS)
                            .unwrap()
                            .value()
                            .unwrap_or(0.0);
                        resources.history.push_fps(fps);
                        let avg = diagnostics
                            .get(FrameTimeDiagnosticsPlugin::FRAME_TIME)
                            .unwrap()
                            .value()
                            .unwrap_or(0.0);
                        let count = diagnostics
                            .get(FrameTimeDiagnosticsPlugin::FRAME_COUNT)
                            .unwrap()
                            .value()
                            .unwrap_or(0.0);
                        let entities = diagnostics
                            .get(EntityCountDiagnosticsPlugin::ENTITY_COUNT)
                            .unwrap()
                            .value()
                            .unwrap_or(0.0);

                        //let meshes = diagnostics.get(AssetCountDiagnosticsPlugin::<Mesh>::diagnostic_id()).unwrap().value().unwrap();
                        let textures = diagnostics.get(AssetCountDiagnosticsPlugin::<Texture>::diagnostic_id()).unwrap().value().unwrap_or(0.0);
                        let colors = diagnostics.get(AssetCountDiagnosticsPlugin::<ColorMaterial>::diagnostic_id()).unwrap().value().unwrap_or(0.0);
                        let materials = diagnostics
                            .get(AssetCountDiagnosticsPlugin::<StandardMaterial>::diagnostic_id())
                            .unwrap()
                            .value()
                            .unwrap_or(0.0);
                        ui.heading("FPS");
                        ui.group(|ui| {
                            ui.columns(4, |ui| {
                                ui[0].label(format!("FPS: {:.0}", fps.abs()));
                                ui[1].label(format!("TIME: {:.4}", avg.abs()));
                                ui[3].label(format!("Count: {}", count.abs()));
                            });
                            ui.end_row();
                            ui.add(
                                Plot::default()
                                    .include_x(super::HISTORY_LENGTH as f32)
                                    .height(50.0)
                                    .show_x(false)
                                    .curve(
                                        Curve::from_ys_f32(&resources.history.fps)
                                            .color(DraculaEgui::PINK),
                                    ),
                            );
                        });
                        ui.end_row();
                        ui.heading("ECS");
                        ui.group(|ui| ui.label(format!("Entities: {:.0}", entities)));
                        ui.heading("Assets");
                        ui.group(|ui| {
                            ui.columns(4, |ui| {
        //                        ui[0].label(format!("Meshes: {:.0}", meshes));
                                ui[1].label(format!("Colors: {:.0}", colors));
                                ui[2].label(format!("Materials: {:.0}", materials));
                                ui[3].label(format!("Textures: {:.0}", textures));
                            });
                        });
                    }
                    1 => {
                        ui.group(|ui| {
                            ui.heading("Save World");
                            ui.columns(2, |ui| {
                                ui[0].text_edit_singleline(&mut resources.save_scene_name);
                                if ui[1].button("Save").clicked() {
                                    save_world.0 = Some(resources.save_scene_name.clone());
                                }
                            });
                        });
                    }
                    2 => {
                        ui.collapsing("DevTools", |ui| {
                            ui.checkbox(
                                &mut resources.egui.show_widgets_on_hover,
                                "Show widgets on Hover",
                            );
                            ui.checkbox(&mut resources.egui.show_resize, "Show Resize");
                            ui.checkbox(
                                &mut resources.egui.expand_width,
                                "Show widgets that make their parent wider",
                            );
                            ui.checkbox(
                                &mut resources.egui.expand_height,
                                "Show widgets that make their parent taller",
                            );
                        });
                        ui.collapsing("World Inspector", |ui| {
                            ui.checkbox(&mut inspector.enabled, "Enabled");
                            ui.checkbox(
                                &mut inspector.despawnable_entities,
                                "Despawnable Entities",
                            );
                            ui.checkbox(&mut inspector.sort_components, "Sort Components");
                        });
                    }
                    _ => unreachable!(),
                }
            });
    }
}
