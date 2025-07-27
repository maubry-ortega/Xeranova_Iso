use bevy::prelude::*;
use crate::player::{Player, CameraFollow};
use crate::physics::Velocity;

pub fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    let mut direction = Vec3::ZERO;
    let speed = 5.0;

    if keyboard.pressed(KeyCode::KeyW) {
        direction.z -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        direction.z += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    for mut velocity in &mut query {
        velocity.0.x = direction.x * speed;
        velocity.0.z = direction.z * speed;
    }
}

pub fn jump_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    query: Query<&Transform, With<Player>>,
    mut velocity_query: Query<&mut Velocity, With<Player>>,
) {
    for (transform, mut velocity) in query.iter().zip(velocity_query.iter_mut()) {
        if keyboard.just_pressed(KeyCode::Space) && transform.translation.y <= 1.05 {
            velocity.0.y = 6.0;
        }
    }
}

pub fn camera_follow_player(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<CameraFollow>, Without<Player>)>,
) {
    if let (Ok(player), Ok(mut cam)) = (player_query.single(), camera_query.single_mut()) {
        let offset = Vec3::new(-6.0, 6.0, 6.0);
        let target = player.translation + offset;
        cam.translation = cam.translation.lerp(target, 0.1);
        cam.look_at(player.translation, Vec3::Y);
    }
}
