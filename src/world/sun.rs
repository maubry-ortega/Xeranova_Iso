use bevy::prelude::*;

#[derive(Component)]
pub struct Sun;

pub fn spawn_sun(mut commands: Commands) {
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            illuminance: 100000.0,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -1.0, 0.0, 0.0)),
        Sun,
    ));
}

pub fn update_sun(time: Res<Time>, mut query: Query<&mut Transform, With<Sun>>) {
    for mut transform in &mut query {
        transform.rotate_y(0.1 * time.delta().as_secs_f32());
    }
}
