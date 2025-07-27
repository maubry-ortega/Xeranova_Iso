use bevy::prelude::*;
use crate::player::Player;
use crate::physics::Velocity;
use crate::world::spawn::SolidBlock;

pub fn apply_velocity_and_collisions(
    time: Res<Time>,
    mut player_q: Query<(&mut Transform, &mut Velocity), With<Player>>,
    block_q: Query<&Transform, (With<SolidBlock>, Without<Player>)>,
) {
    const GRAVITY: f32 = -9.8;
    const PLAYER_HEIGHT: f32 = 2.0;
    const PLAYER_RADIUS: f32 = 0.5;
    const STEP_HEIGHT: f32 = 1.1;

    let delta = time.delta_secs();
    let blocks: Vec<Vec3> = block_q.iter().map(|t| t.translation).collect();

    for (mut transform, mut velocity) in &mut player_q {
        velocity.0.y += GRAVITY * delta;
        let mut next_pos = transform.translation + velocity.0 * delta;
        let mut on_ground = false;

        for &block in &blocks {
            let delta_pos = next_pos - block;

            // vertical
            if delta_pos.y.abs() < 1.0 && delta_pos.xz().abs().max_element() < PLAYER_RADIUS {
                if velocity.0.y < 0.0 && (transform.translation.y - block.y) > 0.5 {
                    next_pos.y = block.y + 1.0;
                    velocity.0.y = 0.0;
                    on_ground = true;
                } else if velocity.0.y > 0.0 {
                    next_pos.y = block.y - PLAYER_HEIGHT;
                    velocity.0.y = 0.0;
                }
            }

            // lateral
            if delta_pos.xz().abs().max_element() < PLAYER_RADIUS && delta_pos.y.abs() < PLAYER_HEIGHT {
                if velocity.0.y <= 0.0 && (block.y - transform.translation.y) <= STEP_HEIGHT {
                    next_pos.y = block.y + 1.0;
                    on_ground = true;
                } else {
                    if delta_pos.x.abs() > delta_pos.z.abs() {
                        next_pos.x = transform.translation.x;
                        velocity.0.x = 0.0;
                    } else {
                        next_pos.z = transform.translation.z;
                        velocity.0.z = 0.0;
                    }
                }
            }
        }

        if on_ground && velocity.0.y < 0.0 {
            velocity.0.y = 0.0;
        }
        transform.translation = next_pos;
    }
}