use bevy::prelude::*;

use crate::player::Player;

pub fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let speed = 200.0;
    let delta = time.delta().as_secs_f32();

    for mut transform in &mut query {
        if keyboard.pressed(KeyCode::KeyW) {
            transform.translation.y += speed * delta;
        }
        if keyboard.pressed(KeyCode::KeyS) {
            transform.translation.y -= speed * delta;
        }
        if keyboard.pressed(KeyCode::KeyA) {
            transform.translation.x -= speed * delta;
        }
        if keyboard.pressed(KeyCode::KeyD) {
            transform.translation.x += speed * delta;
        }
    }
}
