use bevy::prelude::*;
use crate::utils::coordinates as iso;

/// Componente que identifica al jugador
#[derive(Component)]
pub struct Player;

pub fn spawn_player(mut commands: Commands) {
    let start_pos = Vec3::new(
        iso::to_screen_x(10, 10),
        iso::to_screen_y(10, 10),
        100.0,
    );

    commands.spawn((
        Sprite::from_color(Color::srgb(0.9, 0.2, 0.2), Vec2::splat(24.0)),
        Transform::from_translation(start_pos),
        Player,
    ));
}
