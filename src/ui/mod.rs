mod clock;
pub mod menu;

use bevy::prelude::*;
use clock::{setup_clock, update_clock};

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_clock);
        app.add_systems(Update, update_clock);
    }
}
