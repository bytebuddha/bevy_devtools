use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::{
    egui,
    egui::Ui,
    egui::widgets::plot::{Curve, Plot},
    EguiContext,
};

use crate::{
    helpers::DraculaEgui,
    DevToolsTools, DevToolsSettings,
    DevToolsSetting, DevToolsResources, PerformToolAction
};

pub fn draw_debug_ui(
    context: Res<EguiContext>,
    diagnostics: Res<Diagnostics>,
    mut resources: ResMut<DevToolsResources>,
    mut settings: ResMut<DevToolsSettings>,
    devtools_diagnostics: Res<crate::DevToolsDiagnostics>,
    tools: Res<DevToolsTools>,
    mut tool_actions: EventWriter<PerformToolAction>
) {
    if resources.always_visible || resources.enabled {
        let mut pos = context.ctx().available_rect().right_top();
        pos.y += 15.0;
        pos.x -= 370.0;
        egui::Window::new("DevTools")
            .default_pos(pos)
            .enabled(resources.enabled || !resources.always_visible)
            .collapsible(true)
            .show(context.ctx(), |ui| {
                ui.columns(1, |ui| {
                    ui[0].checkbox(&mut resources.always_visible, "Always Visible");
                });
                ui.group(|ui| {
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
                    ui.columns(4, |ui| {
                        ui[0].label(format!("FPS: {:.0}", fps.abs()));
                        ui[1].label(format!("TIME: {:.4}", avg.abs()));
                        ui[3].label(format!("Count: {}", count.abs()));
                    });
                    ui.end_row();
                    ui.add(
                        Plot::default()
                            .include_x(crate::consts::HISTORY_LENGTH as f32)
                            .height(50.0)
                            .show_x(false)
                            .curve(
                                Curve::from_ys_f32(&resources.history.fps)
                                    .color(DraculaEgui::PINK),
                            ),
                    );
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
                        for group in devtools_diagnostics.0.iter() {
                            ui.group(|ui| {
                                ui.heading(group.label.as_ref().unwrap_or(&group.name));
                                ui.end_row();
                                for group in group.data.iter() {
                                    ui.columns(group.len(), |ui| {
                                        for (dex, data) in group.iter().enumerate() {
                                            (data.render)(&mut ui[dex], &diagnostics);
                                        }
                                    });
                                }
                            });
                        }
//                        let entities = diagnostics
//                            .get(EntityCountDiagnosticsPlugin::ENTITY_COUNT)
//                            .unwrap()
//                            .value()
//                            .unwrap_or(0.0);
//
//                        //let meshes = diagnostics.get(AssetCountDiagnosticsPlugin::<Mesh>::diagnostic_id()).unwrap().value().unwrap();
//                        let textures = diagnostics.get(AssetCountDiagnosticsPlugin::<Texture>::diagnostic_id()).unwrap().value().unwrap_or(0.0);
//                        let colors = diagnostics.get(AssetCountDiagnosticsPlugin::<ColorMaterial>::diagnostic_id()).unwrap().value().unwrap_or(0.0);
//                        let materials = diagnostics
//                            .get(AssetCountDiagnosticsPlugin::<StandardMaterial>::diagnostic_id())
//                            .unwrap()
//                            .value()
//                            .unwrap_or(0.0);
//                        ui.heading("ECS");
//                        ui.group(|ui| ui.label(format!("Entities: {:.0}", entities)));
//                        ui.heading("Assets");
//                        ui.group(|ui| {
//                            ui.columns(4, |ui| {
//        //                        ui[0].label(format!("Meshes: {:.0}", meshes));
//                                ui[1].label(format!("Colors: {:.0}", colors));
//                                ui[2].label(format!("Materials: {:.0}", materials));
//                                ui[3].label(format!("Textures: {:.0}", textures));
//                            });
//                        });
                    }
                    1 => {
                        for tool in tools.0.iter() {
                            ui.group(|ui| {
                                ui.columns(2, |ui| {
                                    ui[0].heading(tool.label.as_ref().unwrap_or(&tool.name));
                                    if ui[1].button("Perform").clicked() {
                                        tool_actions.send(PerformToolAction(tool.clone()));
                                    }
                                });
                                (tool.render)(ui, &mut settings);
                            });
                        }
                    }
                    2 => {
                        for setting in settings.0.iter_mut() {
                             display_setting(ui, setting);
                        }
                    }
                    _ => unreachable!(),
                }
            });
    }
}

fn display_setting(ui: &mut Ui, setting: &mut DevToolsSetting) {
    match setting {
        DevToolsSetting::Group { name, label, children } => {
            ui.collapsing(label.as_ref().unwrap_or(name), |ui| {
                for child in children {
                    display_setting(ui, child);
                }
            });
        },
        DevToolsSetting::Bool { name, label, value } => {
            ui.checkbox(value, label.as_ref().unwrap_or(name),);
        },
        DevToolsSetting::String { name, label, value } => {
            ui.label(label.as_ref().unwrap_or(name));
            ui.end_row();
            ui.text_edit_singleline(value);
        }
    }
}
