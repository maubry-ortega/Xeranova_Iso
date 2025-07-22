use bevy::prelude::*;
use crate::{
    region::{RegionList, RegionWithOffset},
    world::generator,
    utils::coordinates as iso,
};

pub fn spawn_world(mut commands: Commands, regions: Res<RegionList>) {
    let tile_size = Vec2::new(28.0, 14.0);
    let height_step = 7.0;

    for RegionWithOffset { region, offset_x, offset_y } in &regions.0 {
        let map = generator::generate(region);
        let [r, g, b] = region.color;

        for (y, row) in map.iter().enumerate() {
            for (x, &h) in row.iter().enumerate() {
                let tile_x = x as i32 + *offset_x;
                let tile_y = y as i32 + *offset_y;

                let screen_x = iso::to_screen_x(tile_x, tile_y);
                let screen_y = iso::to_screen_y(tile_x, tile_y);

                let layers = (h * 3.0).round() as i32;

                for z in 0..layers.max(1) {
                    let factor = ((z + 1) as f32 / layers.max(1) as f32).clamp(0.0, 1.0);
                    let color = Color::srgb(
                        (r * factor).clamp(0.0, 1.0),
                        (g * factor).clamp(0.0, 1.0),
                        (b * factor).clamp(0.0, 1.0),
                    );

                    commands.spawn((
                        Sprite::from_color(color, tile_size),
                        Transform::from_xyz(
                            screen_x,
                            screen_y - z as f32 * height_step,
                            z as f32,
                        ),
                    ));
                }
            }
        }
    }
}
