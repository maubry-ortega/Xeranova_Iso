pub mod loader;

use bevy::prelude::*;
use loader::{load_region, Region};

/// Lista de regiones con offsets para world spawning
#[derive(Resource)]
pub struct RegionList(pub Vec<RegionWithOffset>);

#[derive(Clone)]
pub struct RegionWithOffset {
    pub region: Region,
    pub offset_x: i32,
    pub offset_y: i32,
}

pub struct RegionPlugin;
impl Plugin for RegionPlugin {
    fn build(&self, app: &mut App) {
        // Carga JSON y combina en RegionList
        let mut list = Vec::new();
        list.push(RegionWithOffset {
            region: load_region("data/regiones/pantano.json"),
            offset_x: 0,
            offset_y: 0,
        });
        list.push(RegionWithOffset {
            region: load_region("data/regiones/desierto.json"),
            offset_x: 20,
            offset_y: 0,
        });
        list.push(RegionWithOffset {
            region: load_region("data/regiones/bosque_helado.json"),
            offset_x: 0,
            offset_y: 40,
        });
        app.insert_resource(RegionList(list));
    }
}
