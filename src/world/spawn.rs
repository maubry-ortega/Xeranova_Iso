use bevy::prelude::*;
use bevy::math::primitives::Cuboid;
use crate::region::{RegionList, RegionWithOffset};
use crate::world::voxel::generate_voxel_region;

#[derive(Component)]
pub struct SolidBlock;

/// Posición recomendada para el spawn del jugador (bloque más alto del mundo)
#[derive(Resource, Debug, Clone, Copy)]
pub struct SpawnPosition(pub Vec3);

pub fn spawn_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    regions: Res<RegionList>,
) {
    let mut highest_block_pos = Vec3::ZERO;
    let mut max_y = f32::MIN;

    for RegionWithOffset { region, offset_x, offset_y } in &regions.0 {
        let voxel_map = generate_voxel_region(region);

        let width = region.width;
        let height = region.height;
        let elevation = region.elevation_max;

        for z in 0..elevation {
            for y in 0..height {
                for x in 0..width {
                    let block = &voxel_map[y][x][z];
                    if !block.visible {
                        continue;
                    }

                    let position = Vec3::new(
                        x as f32 + *offset_x as f32,
                        z as f32,
                        y as f32 + *offset_y as f32,
                    );

                    commands.spawn((
                        SolidBlock,
                        Mesh3d::from(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
                        MeshMaterial3d::from(materials.add(StandardMaterial {
                            base_color: block.color,
                            perceptual_roughness: 0.8,
                            ..default()
                        })),
                        Transform::from_translation(position),
                    ));

                    // Determinar el punto de spawn del jugador (bloque más alto)
                    if position.y > max_y {
                        max_y = position.y;
                        highest_block_pos = position;
                    }
                }
            }
        }
    }

    // +Y para que el jugador no spawnee dentro del bloque
    commands.insert_resource(SpawnPosition(highest_block_pos + Vec3::Y));
}
