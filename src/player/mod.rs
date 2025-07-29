pub mod spawn;
pub mod movement;

pub use spawn::{Player, CameraFollow};
pub use crate::physics::Velocity;
use crate::GameState;

use bevy::prelude::*;
use spawn::spawn_player;
use movement::{player_movement, jump_system, camera_follow_player};
use crate::physics::apply_velocity_and_collisions;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Playing),
            spawn_player.in_set(PlayerSystems::Spawn),
        )
        .add_systems(
            Update,
            (
                player_movement,
                jump_system,
                apply_velocity_and_collisions,
                camera_follow_player,
            )
                .chain()
                .in_set(PlayerSystems::Control),
        );
    }
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum PlayerSystems {
    Spawn,
    Control,
}