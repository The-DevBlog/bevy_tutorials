use bevy::prelude::*;

use bevy_color::palettes::css::DARK_GREEN;
const COLOR_DARK_GREEN: Color = Color::Srgba(DARK_GREEN);

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_floor, spawn_light));
    }
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

fn spawn_light(mut commands: Commands) {
    let light = (
        (
            PointLight {
                color: Color::srgba(0.98, 0.59, 0.0, 1.0),
                intensity: 100.0 * 1000.0,
                ..default()
            },
            Transform::from_xyz(0.0, 5.0, 0.0),
        ),
        Name::new("PointLight"),
    );

    commands.spawn(light);
}
