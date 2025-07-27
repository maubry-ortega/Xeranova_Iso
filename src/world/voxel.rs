use bevy::prelude::*;
use crate::region::loader::Region;
use noise::{NoiseFn, Perlin};

#[derive(Clone)]
pub struct Block {
    pub visible: bool,
    pub color: Color,
    pub solid: bool,
}

/// VoxelMap[y][x][z]
pub type VoxelMap = Vec<Vec<Vec<Block>>>;

pub fn generate_voxel_region(region: &Region) -> VoxelMap {
    let mut map = vec![
        vec![
            vec![
                Block {
                    visible: false,
                    color: Color::BLACK,
                    solid: false,
                };
                region.altura_max
            ];
            region.ancho
        ];
        region.alto
    ];

    let perlin = Perlin::new(42);

    for y in 0..region.alto {
        for x in 0..region.ancho {
            let val = perlin.get([x as f64 * 0.1, y as f64 * 0.1]) as f32;

            // Calculamos h como f32 para más precisión y control
            let h_f32 = region.altura_base as f32 + val * region.altura_max as f32;

            // Clamp a rango seguro, luego lo convertimos a usize
            let h = h_f32.round().clamp(1.0, region.altura_max as f32) as usize;

            for z in 0..h {
                map[y][x][z] = Block {
                    visible: true,
                    color: if z == h - 1 {
                        Color::srgb(
                            region.color[0],
                            region.color[1],
                            region.color[2],
                        )
                    } else {
                        Color::srgb(
                            region.fondo_color[0],
                            region.fondo_color[1],
                            region.fondo_color[2],
                        )
                    },
                    solid: true,
                };
            }
        }
    }

    map
}
