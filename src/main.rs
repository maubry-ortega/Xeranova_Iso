use bevy::prelude::*;
use xeranova_game::GamePlugin;
use xeranova_game::world::sun::{Sun, update_sun};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GamePlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, update_sun)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            illuminance: 30_000.0,
            color: Color::srgb(1.0, 0.95, 0.85),
            shadow_depth_bias: 0.02,
            shadow_normal_bias: 0.6,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.5, 0.5, 0.0)),
        Sun,
    ));
}
