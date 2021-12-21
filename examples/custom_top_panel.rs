use bevy::prelude::*;
use bevy_devtools::{DevToolsExt, egui::Ui};
use bevy_devtools::egui::plot::{Plot, Line, Values, Value};
use bevy::diagnostic::{ FrameTimeDiagnosticsPlugin, Diagnostics};

mod utils;

fn render_top_panel(ui: &mut Ui, world: &mut World) {
    let world_ptr = world as *mut _;
    let mut fps_history = world.get_resource_mut::<utils::FPSHistory>().unwrap();
    let world: &mut World = unsafe { &mut *world_ptr };
    let diagnostics = world.get_resource::<Diagnostics>().unwrap();
    let fps = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS).unwrap();
    let frame_time = diagnostics.get(FrameTimeDiagnosticsPlugin::FRAME_TIME).unwrap();
    let frame_count = diagnostics.get(FrameTimeDiagnosticsPlugin::FRAME_COUNT).unwrap();
    ui.group(|ui| {
        ui.columns(3, |ui| {
            if let Some(value) = fps.value() {
                fps_history.push_fps(value, fps.history_len());
                ui[0].label(format!("FPS:{:.0}", value));
            }
            if let Some(value) = frame_time.value() {
                ui[1].label(format!("AVG:{:.4}", value));
            }
            if let Some(value) = frame_count.value() {
                ui[2].label(format!("Count:{}", value));
            }
        });
        ui.end_row();
        ui.separator();
        ui.end_row();
        ui.add(Plot::new("fps-plot")
            .include_x(fps.history_len() as f64)
            .height(50.0)
            .show_x(false)
            .line(Line::new(Values::from_values(
                fps_history
                    .0
                    .iter()
                    .enumerate()
                    .map(|(x, y)| Value {
                        x: x as f64,
                        y: *y as f64,
                    })
                    .collect::<Vec<Value>>(),
            ))));
    });
}

fn main() {
    App::new()
        .init_resource::<utils::FPSHistory>()
        .add_plugin(utils::ExamplePlugin)
        .add_plugin(bevy_devtools::DevToolsPlugin::<utils::ExampleState>::default())
        .devtools_enabled()
        .devtools_top_panel(render_top_panel)
        .run()
}
