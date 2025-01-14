use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use bevy::color::palettes::css;
const COLOR_DARK_GREEN: Color = Color::Srgba(css::DARK_GREEN);
const COLOR_RED: Color = Color::Srgba(css::RED);
const COLOR_MIDNIGHT_BLUE: Color = Color::Srgba(css::MIDNIGHT_BLUE);

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_floor, spawn_objects, spawn_light));
    }
}

fn spawn_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor = (
        (
            Mesh3d(meshes.add(Mesh::from(Plane3d::default().mesh().size(15.0, 15.0)))),
            MeshMaterial3d(materials.add(COLOR_DARK_GREEN)),
        ),
        Collider::cuboid(7.5, 0.0, 7.5),
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

fn spawn_objects(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut create_obj = |size: f32,
                          color: Color,
                          name: String,
                          xyz: (f32, f32, f32)|
     -> (_, Name, Collider, RigidBody) {
        (
            (
                Mesh3d(meshes.add(Cuboid::new(size, size, size))),
                MeshMaterial3d(materials.add(color)),
                Transform::from_xyz(xyz.0, xyz.1, xyz.2),
            ),
            Name::new(name),
            Collider::cuboid(size / 2.0, size / 2.0, size / 2.0),
            RigidBody::Dynamic,
        )
    };

    commands.spawn(create_obj(
        3.0,
        COLOR_RED,
        "Red Cube".to_string(),
        (-4.5, 1.5, -4.5),
    ));
    commands.spawn(create_obj(
        2.0,
        COLOR_MIDNIGHT_BLUE,
        "Blue Cube".to_string(),
        (5.3, 1.0, 5.7),
    ));
}
