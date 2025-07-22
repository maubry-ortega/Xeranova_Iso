use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

pub fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Sprite::from_color(Color::srgb(0.9, 0.2, 0.2), Vec2::splat(24.0)),
        Transform::from_xyz(0.0, 0.0, 100.0),
        Player,
    ));
}
