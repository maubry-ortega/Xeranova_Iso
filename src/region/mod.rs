pub mod loader;

use bevy::prelude::*;
pub use loader::{Region, load_region};

#[derive(Resource, Debug, Clone)]
pub struct RegionList(pub Vec<RegionWithOffset>);

#[derive(Debug, Clone)]
pub struct RegionWithOffset {
    pub region: Region,
    pub offset_x: i32,
    pub offset_y: i32,
}

pub struct RegionPlugin;

impl Plugin for RegionPlugin {
    fn build(&self, app: &mut App) {
        let mut regions = Vec::new();

        // Región del pantano
        regions.push(RegionWithOffset {
            region: load_region("data/regiones/pantano.json"),
            offset_x: 0,
            offset_y: 0,
        });

        // Región del desierto
        regions.push(RegionWithOffset {
            region: load_region("data/regiones/desierto.json"),
            offset_x: 20,
            offset_y: 0,
        });

        // Nueva región: Bosque Gélido
        regions.push(RegionWithOffset {
            region: load_region("data/regiones/bosque_helado.json"),
            offset_x: 40,
            offset_y: 0,
        });

        app.insert_resource(RegionList(regions));
    }
}
