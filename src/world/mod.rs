mod generator;
mod spawn;
mod sun;

use bevy::prelude::*;
use spawn::spawn_world;
use sun::{spawn_sun, update_sun};

pub struct WorldPlugin;
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_world, spawn_sun));
        app.add_systems(Update, update_sun);
    }
}
