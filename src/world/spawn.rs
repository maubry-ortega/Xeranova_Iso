use bevy::prelude::*;
use bevy::math::primitives::Cuboid;
use crate::region::{RegionList, RegionWithOffset};
use crate::world::voxel::generate_voxel_region;

#[derive(Component)]
pub struct SolidBlock; // 👈 componente de colisión para el jugador

pub fn spawn_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    regions: Res<RegionList>,
) {
    for region_with in &regions.0 {
        let RegionWithOffset { region, offset_x, offset_y } = region_with;
        let map = generate_voxel_region(region);

        for z in 0..region.altura_max {
            for y in 0..region.alto {
                for x in 0..region.ancho {
                    let block = &map[y][x][z];
                    if !block.visible { continue; }

                    let tx = x as f32 + *offset_x as f32;
                    let ty = z as f32;
                    let tz = y as f32 + *offset_y as f32;

                    commands.spawn((
                        SolidBlock,
                        Mesh3d::from(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
                        MeshMaterial3d::from(materials.add(StandardMaterial {
                            base_color: block.color,
                            perceptual_roughness: 0.8,
                            ..default()
                        })),
                        Transform::from_xyz(tx, ty, tz),
                    ));
                }
            }
        }
    }
}
