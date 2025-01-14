use bevy::prelude::*;
use bevy_third_person_camera::{ThirdPersonCamera, Zoom};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    let camera = (
        (
            Camera3d::default(),
            Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ),
        ThirdPersonCamera {
            zoom: Zoom::new(3.0, 8.0),
            ..default()
        },
    );
    commands.spawn(camera);
}
