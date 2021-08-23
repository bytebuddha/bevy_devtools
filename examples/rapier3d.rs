use bevy::prelude::*;
use bevy_devtools::{DevToolsExt, DevToolsPlugin};
use bevy_rapier3d::{
    physics::{
        ColliderBundle, ColliderPositionSync, NoUserData, RapierPhysicsPlugin, RigidBodyBundle,
    },
    prelude::{ColliderShape, RigidBodyType},
};

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(DevToolsPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_startup_system(setup.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let cube_size = 1.0;
    let floor_size = 6.0;

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: floor_size })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..Default::default()
        })
        .insert(Name::new("Floor"))
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static,
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(floor_size / 2.0, 0.1, floor_size / 2.0),
            ..Default::default()
        });

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: cube_size })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            ..Default::default()
        })
        .insert(Name::new("Cube"))
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(cube_size / 2.0, cube_size / 2.0, cube_size / 2.0),
            ..Default::default()
        })
        .insert_bundle(RigidBodyBundle {
            position: Vec3::new(0.0, 2.0, 0.0).into(),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete);

    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::default(), Vec3::Y),
        ..Default::default()
    });
}
