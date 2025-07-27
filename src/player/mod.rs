pub mod spawn;
pub mod movement;

// 🔓 reexporta los componentes aquí
pub use spawn::{Player, Velocity, CameraFollow};

use bevy::prelude::*;
use spawn::spawn_player;
use movement::{player_movement, jump_system, apply_velocity, camera_follow_player};

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, (
            player_movement,
            jump_system,
            apply_velocity,
            camera_follow_player,
        ));
    }
}
