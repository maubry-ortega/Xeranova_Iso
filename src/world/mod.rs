pub mod voxel;
pub mod spawn;
pub mod sun; // 👈 añade esto para acceder al sistema del sol

use bevy::prelude::*;
use spawn::spawn_world;
use sun::{spawn_sun, update_sun}; // 👈 importa los sistemas del sol

pub struct WorldPlugin;
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_world, spawn_sun)) // 🌍 + ☀️
           .add_systems(Update, update_sun);               // ☀️ se mueve
    }
}
