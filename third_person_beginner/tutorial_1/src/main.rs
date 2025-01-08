use bevy::{prelude::*, DefaultPlugins};

const COLOR_DARK_GREEN: Color = Color::rgb(0.0, 0.5, 0.0);
const COLOR_BLUE: Color = Color::rgb(0.0, 0.0, 1.0);

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
    let camera = Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    };
    commands.spawn(camera);
}

fn spawn_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor = PbrBundle {
        mesh: Mesh3d(meshes.add(Mesh::from(Plane3d::default().mesh().size(15.0, 15.0)))),
        material: MeshMaterial3d(materials.add(COLOR_DARK_GREEN)),
        ..default()
    };

    commands.spawn(floor);
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player = PbrBundle {
        mesh: Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        material: MeshMaterial3d(materials.add(COLOR_BLUE)),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    };

    commands.spawn(player);
}

fn spawn_light(mut commands: Commands) {
    let light = PointLightBundle {
        point_light: PointLight {
            intensity: 2000.0 * 1000.0,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 5.0, 0.0),
        ..default()
    };

    commands.spawn(light);
}
