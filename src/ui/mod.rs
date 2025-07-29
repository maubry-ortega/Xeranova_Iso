mod clock;
pub mod menu;

use bevy::prelude::*;
use clock::{setup_clock, update_clock};
use crate::GameState;
use menu::{spawn_menu, handle_menu_input};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), spawn_menu)
           .add_systems(Update, handle_menu_input.run_if(in_state(GameState::Menu)))
           .add_systems(Startup, setup_clock)
           .add_systems(Update, update_clock);
    }
}
