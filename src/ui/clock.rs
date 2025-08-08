use bevy::prelude::*;

#[derive(Resource)]
pub struct SolarTime(pub f32);

#[derive(Component)]
pub struct ClockText;

pub fn setup_clock(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(SolarTime(0.0));

    let font = asset_server.load("fonts/FiraSans-ExtraBoldItalic.ttf");

    commands.spawn((
        Text::new("Hora: 06:00"),
        TextFont {
            font,
            font_size: 24.0,
            ..default()
        },
        TextLayout::default(),
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        ClockText,
    ));
}

pub fn update_clock(
    time: Res<Time>,
    mut clock: ResMut<SolarTime>,
    mut query: Query<&mut Text, With<ClockText>>,
) {
    use std::f32::consts::PI;

    clock.0 += time.delta().as_secs_f32() * 0.05;
    let angle = clock.0 % (2.0 * PI);
    let hour = ((angle / (2.0 * PI)) * 24.0).round() as u32;

    let formatted = format!("Hora: {:02}:00", hour % 24);

    for mut text in &mut query {
        **text = formatted.clone();
    }
}
