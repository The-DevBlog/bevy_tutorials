use bevy::{prelude::*, DefaultPlugins};
use player::PlayerPlugin;
use world::WorldPlugin;

mod player;
mod world;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PlayerPlugin, WorldPlugin))
        .run();
}
