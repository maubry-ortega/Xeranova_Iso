pub mod loader;
pub mod types;

use bevy::prelude::*;
use rand::prelude::*;
use std::fs;

use crate::utils::map_size::SelectedMapSize;
use crate::region::loader::load_region;
use crate::world::spawn::RegionWithOffset;
use crate::GameState;

#[derive(Resource)]
pub struct RegionList(pub Vec<RegionWithOffset>);

pub struct RegionPlugin;
impl Plugin for RegionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(RegionList(Vec::new()));
        app.add_systems(OnEnter(GameState::Generating), generate_regions);
    }
}

fn generate_regions(
    mut region_list: ResMut<RegionList>,
    selected: Res<SelectedMapSize>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let Some(map_size) = selected.0 else { return };

    let (cols, rows) = map_size.dimensions();
    let region_files: Vec<_> = fs::read_dir("assets/regions")
        .expect("No se encontró la carpeta 'assets/regions'")
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension()? == "json" {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    let mut regions = Vec::new();
    let mut rng = rand::rngs::ThreadRng::default();

    for y in 0..rows {
        for x in 0..cols {
            if let Some(path) = region_files.choose(&mut rng) {
                let region = load_region(path.to_str().unwrap());
                regions.push(RegionWithOffset {
                    region,
                    offset_x: x as usize * 32,
                    offset_y: y as usize * 32,
                });
            }
        }
    }

    region_list.0 = regions;
    next_state.set(GameState::Playing);
}
