//! A simplified implementation of the classic game "Breakout".

mod colider;
mod colision_event;
mod collision;
mod collision_sound;
mod component;
mod constants;
mod game_setup;
mod paddle;
mod physics;
mod score_board;
mod spawner;
mod velocity;
mod wall_bundle;
mod wall_location;

use bevy::prelude::*;
use colider::Collider;
use colision_event::CollisionEvent;
use component::ball::Ball;
use constants::*;
use score_board::Scoreboard;
use velocity::Velocity;
use wall_location::WallLocation;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Scoreboard { score: 0 })
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_event::<CollisionEvent>()
        .add_systems(Startup, game_setup::setup)
        // Add our gameplay simulation systems to the fixed timestep schedule
        // which runs at 64 Hz by default
        .add_systems(
            FixedUpdate,
            (
                physics::apply_velocity,
                paddle::move_paddle,
                collision::check_for_collisions,
                collision::play_collision_sound,
            )
                // `chain`ing systems together runs them in order
                .chain(),
        )
        .add_systems(Update, score_board::update_scoreboard)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Update, spawner::respawn_bricks)
        .run();
}
