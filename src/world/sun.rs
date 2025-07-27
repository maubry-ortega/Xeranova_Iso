use bevy::prelude::*;

#[derive(Component)]
pub struct Sun;

pub fn spawn_sun(mut commands: Commands) {
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            illuminance: 30_000.0,
            color: Color::srgb(1.0, 0.95, 0.85),
            shadow_depth_bias: 0.02,
            shadow_normal_bias: 0.6,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.5, 0.5, 0.0)),
        Sun,
    ));
}

pub fn update_sun(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Sun>>,
) {
    for mut transform in &mut query {
        transform.rotate_y(0.01 * time.delta().as_secs_f32());
    }
}
