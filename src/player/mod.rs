mod spawn;
mod movement;

use bevy::prelude::*;
use spawn::*;
use movement::player_movement;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, player_movement);
    }
}
