use bevy::prelude::*;
use crate::player::Player;
use crate::physics::Velocity;
use crate::world::spawn::SolidBlock;

pub fn simple_ground_collision(
    mut query: ParamSet<(
        Query<(&mut Transform, &mut Velocity), With<Player>>,
        Query<&Transform, With<SolidBlock>>,
    )>,
    time: Res<Time>,
) {
    let block_positions: Vec<Vec3> = query.p1().iter().map(|t| t.translation).collect();

    for (mut transform, mut velocity) in &mut query.p0() {
        velocity.0.y -= 9.8 * time.delta_secs();

        let next_pos = transform.translation + velocity.0 * time.delta_secs();
        let player_bottom = next_pos - Vec3::Y * 0.5;

        let on_ground = block_positions.iter().any(|block_pos| {
            (block_pos - player_bottom).length() < 0.6
        });

        if on_ground && next_pos.y < transform.translation.y {
            velocity.0.y = 0.0;
        } else {
            transform.translation = next_pos;
        }
    }
}
