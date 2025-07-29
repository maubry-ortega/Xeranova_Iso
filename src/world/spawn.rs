use bevy::prelude::*;
use bevy::math::primitives::Cuboid;
use crate::region::RegionList;
use crate::world::voxel::generate_voxel_region;
use crate::region::types::Region;
use crate::GameState;

#[derive(Component)]
pub struct SolidBlock;

#[derive(Resource, Debug, Clone, Copy)]
pub struct SpawnPosition(pub Vec3);

#[derive(Debug, Clone)]
pub struct RegionWithOffset {
    pub region: Region,
    pub offset_x: usize,
    pub offset_y: usize,
}

pub fn spawn_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    regions: Res<RegionList>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    println!("🧱 spawn_world ejecutado");

    let mut highest_block_pos = Vec3::ZERO;
    let mut max_y = f32::MIN;

    for region_data in &regions.0 {
        let region = &region_data.region;
        let offset_x = region_data.offset_x;
        let offset_y = region_data.offset_y;
        let map = generate_voxel_region(region);

        for z in 0..region.elevation_max {
            for y in 0..region.height {
                for x in 0..region.width {
                    let block = &map[y][x][z];
                    if !block.visible {
                        continue;
                    }

                    let position = Vec3::new(
                        x as f32 + offset_x as f32,
                        z as f32,
                        y as f32 + offset_y as f32,
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

                    if position.y > max_y {
                        max_y = position.y;
                        highest_block_pos = position;
                    }
                }
            }
        }
    }

    commands.insert_resource(SpawnPosition(highest_block_pos + Vec3::Y));
    next_state.set(GameState::Playing);
}
