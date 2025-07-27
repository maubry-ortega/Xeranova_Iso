use bevy::prelude::*;
use crate::region::loader::Region;
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

    let mut map = vec![vec![vec![Block {
        visible: false,
        color: Color::BLACK,
        solid: false,
    }; region.altura_max]; region.ancho]; region.alto];

    for y in 0..region.alto {
        for x in 0..region.ancho {
            let val = perlin.get([x as f64 * 0.1, y as f64 * 0.1]) as f32;
            let h_f32 = region.altura_base as f32 + val * region.altura_max as f32;
            let h = h_f32.round().clamp(1.0, region.altura_max as f32) as usize;

            for z in 0..h {
                let is_surface = z == h - 1;
                map[y][x][z] = Block {
                    visible: true,
                    solid: true,
                    color: Color::srgb(
                        if is_surface { region.color[0] } else { region.fondo_color[0] },
                        if is_surface { region.color[1] } else { region.fondo_color[1] },
                        if is_surface { region.color[2] } else { region.fondo_color[2] },
                    ),
                };
            }
        }
    }

    map
}
