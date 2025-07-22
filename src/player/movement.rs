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

pub fn camera_follow_player(
    player_query: Query<&Transform, (With<Player>, Changed<Transform>)>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    if let Ok(player_transform) = player_query.single() {
        if let Ok(mut camera_transform) = camera_query.single_mut() {
            camera_transform.translation.x = player_transform.translation.x;
            camera_transform.translation.y = player_transform.translation.y;
        }
    }
}