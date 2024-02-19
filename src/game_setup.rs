use bevy::prelude::*;

// Temporary import
use crate::spawner;

// Add the game's entities to our world
pub fn setup(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    spawner::spawn_initial_entities(commands, meshes, materials, asset_server);
}
