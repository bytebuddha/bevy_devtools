use bevy::prelude::*;
use bevy_devtools::{DevTool, DevToolsSettings, DevToolsExt, egui::Ui};

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_devtools::DevToolsPlugin)
        .add_startup_system(setup.system())
        .devtools_tool(DevTool {
            label: Some("This tool is useless".into()),
            name: "blank_tool".into(),
            perform_icon: None,
            perform: None,
            render: render_blank_tool
        })
        .devtools_tool(DevTool {
            label: Some("Hello World".into()),
            name: "hello_tool".into(),
            perform_icon: Some("Hello".into()),
            perform: Some(perform_hello),
            render: render_hello_tool
        })
        .run()
}

fn render_blank_tool(ui: &mut Ui, _: &mut DevToolsSettings) {
    ui.label("Custom Tool");
}

fn render_hello_tool(ui: &mut Ui, _: &mut DevToolsSettings) {
    ui.label("Hello World Tool");
}

fn perform_hello(_: &mut World) {
    println!("Hello World");
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_matrix(Mat4::face_toward(
                Vec3::new(-3.0, 5.0, 8.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
            )),
            ..Default::default()
        })
        .insert(Name::new("Camera"));
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0 })),
            material: materials.add(Color::rgb_u8(80, 233, 54).into()),
            ..Default::default()
        })
        .insert(Name::new("Floor"));
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
            ..Default::default()
        })
        .insert(Name::new("Cube"))
        .with_children(|commands| {
            commands
                .spawn_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
                    transform: Transform::from_xyz(0.0, 0.8, 0.0),
                    material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
                    ..Default::default()
                })
                .insert(Name::new("Child"))
                .with_children(|commands| {
                    commands
                        .spawn_bundle(PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.2 })),
                            transform: Transform::from_xyz(0.0, 0.4, 0.0),
                            material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
                            ..Default::default()
                        })
                        .insert(Name::new("Child"));
                });
        });
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                subdivisions: 20,
                radius: 0.5,
            })),
            transform: Transform::from_xyz(1.5, 1.5, 1.5),
            material: materials.add(Color::RED.into()),
            ..Default::default()
        })
        .insert(Name::new("Sphere"));
    commands
        .spawn_bundle(LightBundle {
            transform: Transform::from_xyz(10.3, 8.0, -2.3),
            light: Light {
                range: 20.0,
                intensity: 1237.0,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("Light"));
    commands
        .spawn_bundle(LightBundle {
            transform: Transform::from_xyz(-6.2, 8.0, 4.3),
            light: Light {
                range: 20.0,
                intensity: 245.0,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("Second Light"));
}
