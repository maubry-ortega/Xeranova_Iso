pub mod loader;

use bevy::prelude::*;
use loader::load_region;

pub struct RegionPlugin;
impl Plugin for RegionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(load_region("data/regiones/pantano.json"));
    }
}
