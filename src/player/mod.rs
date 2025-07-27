pub mod spawn;
pub mod movement;

pub use crate::physics::Velocity;
pub use spawn::{Player, CameraFollow};

use bevy::prelude::*;
use spawn::spawn_player;
use movement::{player_movement, jump_system, camera_follow_player};
use crate::physics::simple_ground_collision;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, (
            player_movement,
            jump_system,
            simple_ground_collision,
            camera_follow_player,
        ));
    }
}
