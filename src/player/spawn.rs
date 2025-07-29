use bevy::prelude::*;
use bevy::math::primitives::Capsule3d;
use crate::physics::Velocity;
use crate::world::spawn::SpawnPosition;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct CameraFollow;

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    spawn_pos: Res<SpawnPosition>,
) {
    let position = spawn_pos.0;

    commands.spawn((
        Player,
        Velocity(Vec3::ZERO),
        Mesh3d::from(meshes.add(Capsule3d::default())),
        MeshMaterial3d::from(materials.add(StandardMaterial {
            base_color: Color::srgb(0.9, 0.2, 0.2),
            ..default()
        })),
        Transform::from_translation(position),
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(position.x + 4.0, position.y + 6.0, position.z + 8.0)
            .looking_at(position, Vec3::Y),
        CameraFollow,
    ));
}
