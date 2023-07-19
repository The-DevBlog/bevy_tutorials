use bevy::{prelude::*, DefaultPlugins};
use camera::CameraPlugin;
use player::PlayerPlugin;
use world::WorldPlugin;

mod camera;
mod player;
mod world;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PlayerPlugin, WorldPlugin, CameraPlugin))
        .run();
}
