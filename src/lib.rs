pub mod region;
pub mod world;
pub mod player;
pub mod utils;
pub mod ui;
pub mod physics;
pub mod game_state;

use bevy::prelude::*;
use region::RegionPlugin;
use world::WorldPlugin;
use player::PlayerPlugin;
use ui::UiPlugin;
pub use game_state::GameState;
use utils::map_size::SelectedMapSize;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .insert_resource(SelectedMapSize::default())
            .add_plugins((
                RegionPlugin,
                WorldPlugin,
                PlayerPlugin,
                UiPlugin,
            ));
    }
}
