use bevy::{prelude::*, DefaultPlugins};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use bevy_third_person_camera::*;
use camera::CameraPlugin;
use player::PlayerPlugin;
use world::WorldPlugin;

mod camera;
mod player;
mod world;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PlayerPlugin,
            WorldPlugin,
            CameraPlugin,
            WorldInspectorPlugin::new(),
            ThirdPersonCameraPlugin,
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin {
                enabled: true,
                ..default()
            },
        ))
        .run();
}
