use bevy::prelude::*;
use bevy::math::primitives::Cuboid;
use crate::region::{RegionList, RegionWithOffset};
use crate::world::voxel::{generate_voxel_region};

#[derive(Component)]
pub struct SolidBlock;

/// Posición recomendada para el spawn del jugador (e.g. bloque más alto)
#[derive(Resource, Debug, Clone, Copy)]
pub struct SpawnPosition(pub Vec3);

pub fn spawn_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    regions: Res<RegionList>,
) {
    let mut highest_block = Vec3::new(0.0, 0.0, 0.0);
    let mut highest_y = f32::MIN;

    for RegionWithOffset { region, offset_x, offset_y } in &regions.0 {
        let map = generate_voxel_region(region);

        for z in 0..region.altura_max {
            for y in 0..region.alto {
                for x in 0..region.ancho {
                    let block = &map[y][x][z];
                    if !block.visible { continue; }

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

                    // Detectar el bloque más alto para spawnear al jugador
                    if position.y > highest_y {
                        highest_y = position.y;
                        highest_block = position;
                    }
                }
            }
        }
    }

    // SpawnPosition +1 para que no colisione justo con el bloque
    commands.insert_resource(SpawnPosition(highest_block + Vec3::Y));
}
