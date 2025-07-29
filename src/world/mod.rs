pub mod voxel;
pub mod spawn;
pub mod sun;

use bevy::prelude::*;
use crate::GameState;
use spawn::spawn_world;
use sun::{spawn_sun, update_sun};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::GeneratingDone), spawn_world)
            .add_systems(Startup, spawn_sun)
            .add_systems(Update, update_sun);
    }
}
