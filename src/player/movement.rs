use bevy::prelude::*;
use crate::player::{Player, CameraFollow};
use crate::physics::Velocity;

pub fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    const SPEED: f32 = 5.0;
    let mut direction = Vec3::ZERO;

    if keyboard.pressed(KeyCode::KeyW) { direction.z -= 1.0; }
    if keyboard.pressed(KeyCode::KeyS) { direction.z += 1.0; }
    if keyboard.pressed(KeyCode::KeyA) { direction.x -= 1.0; }
    if keyboard.pressed(KeyCode::KeyD) { direction.x += 1.0; }

    for mut velocity in &mut query {
        velocity.0.x = direction.x * SPEED;
        velocity.0.z = direction.z * SPEED;
    }
}

/// Salto solo cuando está en el suelo
pub fn jump_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Transform, &mut Velocity), With<Player>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        let Ok((transform, mut velocity)) = query.single_mut() else { return };
        if velocity.0.y.abs() < 0.01 && (transform.translation.y % 1.0 <= 0.05) {
            velocity.0.y = 6.5;
        }
    }
}

/// Cámara que sigue al jugador
pub fn camera_follow_player(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<CameraFollow>, Without<Player>)>,
) {
    let Ok(player) = player_query.single() else { return };
    let Ok(mut cam) = camera_query.single_mut() else { return };

    const OFFSET: Vec3 = Vec3::new(-6.0, 6.0, 6.0);
    let target = player.translation + OFFSET;
    cam.translation = cam.translation.lerp(target, 0.1);
    cam.look_at(player.translation, Vec3::Y);
}