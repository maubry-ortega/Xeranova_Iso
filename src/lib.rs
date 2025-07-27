pub mod region;
pub mod world;
pub mod player;
pub mod utils;
pub mod ui;

use bevy::prelude::*;
use region::RegionPlugin;
use world::WorldPlugin;
use player::PlayerPlugin;
use ui::UiPlugin;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            RegionPlugin,
            WorldPlugin,
            PlayerPlugin,
            UiPlugin,
        ));
    }
}
