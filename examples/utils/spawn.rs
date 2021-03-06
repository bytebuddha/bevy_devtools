use bevy::prelude::*;

pub fn spawn_world(
    mut commands: Commands,
    assets: Res<AssetServer>,
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
        .insert(Name::new("Camera"))
        .insert(super::rotates::Rotates);
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0 })),
            material: materials.add(
                assets
                    .load("textures/prototype/PNG/Dark/texture_05.png")
                    .into(),
            ),
            ..Default::default()
        })
        .insert(Name::new("Floor"));
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            material: materials.add(
                assets
                    .load("textures/prototype/PNG/Red/texture_01.png")
                    .into(),
            ),
            ..Default::default()
        })
        .insert(Name::new("Cube"))
        .with_children(|commands| {
            commands
                .spawn_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
                    transform: Transform::from_xyz(0.0, 0.8, 0.0),
                    material: materials.add(
                        assets
                            .load("textures/prototype/PNG/Green/texture_01.png")
                            .into(),
                    ),
                    ..Default::default()
                })
                .insert(Name::new("Child"))
                .with_children(|commands| {
                    commands
                        .spawn_bundle(PbrBundle {
                            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.2 })),
                            transform: Transform::from_xyz(0.0, 0.4, 0.0),
                            material: materials.add(
                                assets
                                    .load("textures/prototype/PNG/Purple/texture_01.png")
                                    .into(),
                            ),
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
        .spawn_bundle(PointLightBundle {
            transform: Transform::from_xyz(10.3, 8.0, -2.3),
            point_light: PointLight {
                range: 20.0,
                intensity: 1237.0,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("Light"));
    commands
        .spawn_bundle(PointLightBundle {
            transform: Transform::from_xyz(-6.2, 8.0, 4.3),
            point_light: PointLight {
                range: 20.0,
                intensity: 245.0,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("Second Light"));
}
