//! A simplified implementation of the classic game "Breakout".

mod colider;
mod colision_event;
mod collision;
mod collision_sound;
mod component;
mod constants;
mod game_setup;
mod menu;
mod physics;
mod score_board;
mod spawner;
mod state;
mod velocity;
mod wall_bundle;
mod wall_location;

use bevy::prelude::*;
use colider::Collider;
use colision_event::CollisionEvent;
use component::{ball::Ball, paddle};
use constants::*;
use menu::{cleanup_menu, menu, setup_menu};
use score_board::Scoreboard;
use state::AppState;
use velocity::Velocity;
use wall_location::WallLocation;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Scoreboard { score: 0 })
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .init_state::<AppState>()
        .add_event::<CollisionEvent>()
        .add_systems(OnEnter(AppState::InGame), game_setup::setup)
        .add_systems(OnEnter(AppState::Menu), setup_menu)
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
                .chain()
                .run_if(in_state(AppState::InGame)),
        )
        .add_systems(OnExit(AppState::Menu), cleanup_menu)
        .add_systems(
            Update,
            (score_board::update_scoreboard).run_if(in_state(AppState::InGame)),
        )
        .add_systems(
            Update,
            (bevy::window::close_on_esc).run_if(in_state(AppState::InGame)),
        )
        .add_systems(
            Update,
            (spawner::respawn_bricks).run_if(in_state(AppState::InGame)),
        )
        .add_systems(Update, menu.run_if(in_state(AppState::Menu)))
        .run();
}
