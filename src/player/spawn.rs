use bevy::prelude::*;
use bevy::math::primitives::Capsule3d;
use crate::physics::Velocity;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct CameraFollow;

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let start_pos = Vec3::new(10.0, 10.0, 10.0);

    commands.spawn((
        Player,
        Velocity(Vec3::ZERO),
        Mesh3d::from(meshes.add(Capsule3d::default())),
        MeshMaterial3d::from(materials.add(StandardMaterial {
            base_color: Color::srgb(0.9, 0.2, 0.2),
            ..default()
        })),
        Transform::from_translation(start_pos),
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(4.0, 8.0, 12.0).looking_at(start_pos, Vec3::Y),
        CameraFollow,
    ));
}
