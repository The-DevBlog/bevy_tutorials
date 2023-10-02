use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

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
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane::from_size(15.0))),
            material: materials.add(Color::DARK_GREEN.into()),
            ..default()
        },
        Collider::cuboid(7.5, 0.0, 7.5),
    );

    commands.spawn(floor);
}

fn spawn_light(mut commands: Commands) {
    let light = (
        PointLightBundle {
            point_light: PointLight {
                color: Color::rgba(0.98, 0.59, 0.0, 1.0),
                intensity: 100.0,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 5.0, 0.0),
            ..default()
        },
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
     -> (PbrBundle, Name, Collider, RigidBody) {
        (
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube::new(size))),
                material: materials.add(color.into()),
                transform: Transform::from_xyz(xyz.0, xyz.1, xyz.2),
                ..default()
            },
            Name::new(name),
            Collider::cuboid(size / 2.0, size / 2.0, size / 2.0),
            RigidBody::Dynamic,
        )
    };

    commands.spawn(create_obj(
        3.0,
        Color::RED,
        "Red Cube".to_string(),
        (-4.5, 1.5, -4.5),
    ));
    commands.spawn(create_obj(
        2.0,
        Color::MIDNIGHT_BLUE,
        "Blue Cube".to_string(),
        (5.3, 1.0, 5.7),
    ));
}
