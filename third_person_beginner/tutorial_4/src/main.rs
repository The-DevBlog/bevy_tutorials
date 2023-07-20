use bevy::{prelude::*, DefaultPlugins};
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
            ThirdPersonCameraPlugin,
        ))
        .run();
}
