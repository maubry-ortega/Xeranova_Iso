use bevy::prelude::*;
use crate::region::types::Region;
use noise::{NoiseFn, Perlin};

#[derive(Clone)]
pub struct Block {
    pub visible: bool,
    pub color: Color,
    pub solid: bool,
}

pub type VoxelMap = Vec<Vec<Vec<Block>>>;

pub fn generate_voxel_region(region: &Region) -> VoxelMap {
    let perlin = Perlin::new(42);

    let width = region.width;
    let height = region.height;
    let elevation_min = region.elevation_min;
    let elevation_max = region.elevation_max;

    // Inicializar mapa vacío
    let mut map = vec![vec![vec![Block {
        visible: false,
        color: Color::BLACK,
        solid: false,
    }; elevation_max]; width]; height];

    for y in 0..height {
        for x in 0..width {
            // Ruido Perlin para la altura
            let noise_val = perlin.get([x as f64 * 0.1, y as f64 * 0.1]) as f32;
            let height_f32 = elevation_min as f32 + noise_val * elevation_max as f32;
            let column_height = height_f32.round().clamp(1.0, elevation_max as f32) as usize;

            for z in 0..column_height {
                let is_surface = z == column_height - 1;
                let color = if is_surface {
                    Color::srgb(
                        region.base_color[0],
                        region.base_color[1],
                        region.base_color[2],
                    )
                } else {
                    Color::srgb(
                        region.background_color[0],
                        region.background_color[1],
                        region.background_color[2],
                    )
                };

                map[y][x][z] = Block {
                    visible: true,
                    solid: true,
                    color,
                };
            }
        }
    }

    map
}
