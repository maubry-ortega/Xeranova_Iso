use bevy::prelude::*;
use crate::player::{Player, Velocity, CameraFollow};

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
    mut query: Query<(&Transform, &mut Velocity), With<Player>>,
) {
    for (transform, mut velocity) in &mut query {
        if keyboard.just_pressed(KeyCode::Space) && transform.translation.y <= 1.1 {
            velocity.0.y = 6.0;
        }
    }
}

pub fn apply_velocity(
    mut query: Query<(&mut Transform, &mut Velocity), With<Player>>,
    time: Res<Time>,
) {
    for (mut transform, mut velocity) in &mut query {
        velocity.0.y -= 9.8 * time.delta_secs(); // ✅ corregido

        transform.translation += velocity.0 * time.delta_secs();

        // colisión básica con el suelo
        if transform.translation.y < 1.0 {
            transform.translation.y = 1.0;
            velocity.0.y = 0.0;
        }
    }
}

pub fn camera_follow_player(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<CameraFollow>, Without<Player>)>,
) {
    if let (Ok(player), Ok(mut cam)) = (player_query.get_single(), camera_query.get_single_mut()) {
        let offset = Vec3::new(-6.0, 6.0, 6.0);
        let target = player.translation + offset;

        cam.translation = cam.translation.lerp(target, 0.1);
        cam.look_at(player.translation, Vec3::Y);
    }
}
