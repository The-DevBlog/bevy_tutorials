use bevy::{prelude::*, DefaultPlugins};

use bevy_color::palettes::css::{BLUE, DARK_GREEN};
const COLOR_DARK_GREEN: Color = Color::Srgba(DARK_GREEN);
const COLOR_BLUE: Color = Color::Srgba(BLUE);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(
            Startup,
            (spawn_player, spawn_camera, spawn_floor, spawn_light),
        )
        .run();
}

fn spawn_camera(mut commands: Commands) {
    let camera = (
        Camera3d::default(),
        Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        );

    commands.spawn(camera);
}

fn spawn_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor = (
        Mesh3d(meshes.add(Mesh::from(Plane3d::default().mesh().size(15.0, 15.0)))),
        MeshMaterial3d(materials.add(COLOR_DARK_GREEN)),
    );

    commands.spawn(floor);
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player = (
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(COLOR_BLUE)),
        Transform::from_xyz(0.0, 0.5, 0.0),
    );

    commands.spawn(player);
}

fn spawn_light(mut commands: Commands) {
    let light = (
        PointLight {
            intensity: 2000.0 * 1000.0,
            ..default()
        },
        Transform::from_xyz(0.0, 5.0, 0.0),
    );

    commands.spawn(light);
}
