use bevy::prelude::*;

use bevy_color::palettes::css::BLUE;
const COLOR_BLUE: Color = Color::Srgba(BLUE);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
    }
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
