use bevy::prelude::*;
use crate::{
    region::loader::Region,
    world::generator,
    utils::coordinates as iso,
};

pub fn spawn_world(mut commands: Commands, region: Res<Region>) {
    let map = generator::generate(&region);
    let [r, g, b] = region.color;

    for (y, row) in map.iter().enumerate() {
        for (x, &h) in row.iter().enumerate() {
            let factor = ((h + 1.0) * 0.5).clamp(0.0, 1.0);
            let color = Color::srgb(
                (r * factor).clamp(0.0, 1.0),
                (g * factor).clamp(0.0, 1.0),
                (b * factor).clamp(0.0, 1.0),
            );

            commands.spawn((
                Sprite::from_color(color, Vec2::splat(28.0)),
                Transform::from_xyz(
                    iso::to_screen_x(x, y),
                    iso::to_screen_y(x, y),
                    h * 10.0,
                ),
            ));
        }
    }
}
