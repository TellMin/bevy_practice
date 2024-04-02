use bevy::prelude::*;

// Temporary import
use crate::spawner;

#[derive(Resource)]
pub struct InGameData {
    pub ball: Entity,
    pub paddle: Entity,
    pub walls: Vec<Entity>,
    pub bricks: Vec<Entity>,
    pub score_board: Entity,
}

impl InGameData {
    pub fn set_bricks(&mut self, bricks: Vec<Entity>) {
        self.bricks = bricks;
    }
}

// Add the game's entities to our world
pub fn setup(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    spawner::spawn_initial_entities(commands, meshes, materials, asset_server);
}
