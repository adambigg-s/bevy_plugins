use bevy::prelude::*;
use bevy_plugins::camera::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraPlugin)
        .add_systems(Startup, scene_setup)
        .run();
}

fn scene_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(100.)))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.2, 0.25),
            ..Default::default()
        })),
        Transform::from_xyz(0., -3., 0.),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(5., 5., 5.))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(1., 1., 1.),
            ..Default::default()
        })),
        Transform::default(),
    ));

    commands.spawn((
        DirectionalLight { ..Default::default() },
        Transform::from_xyz(0., 10., 15.).looking_at(Vec3::new(0., -1., -1.), Vec3::Y),
    ));
}
