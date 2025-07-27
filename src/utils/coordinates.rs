pub const TILE_SIZE: f32 = 32.0;

/// Para convertir coordenadas voxel (x,y,z) a posición 3D isométrica
pub fn to_iso_coords(x: i32, y: i32, z: i32) -> (f32, f32) {
    let screen_x = (x - y) as f32 * TILE_SIZE / 2.0;
    let screen_y = (x + y) as f32 * TILE_SIZE / 4.0 - z as f32 * TILE_SIZE / 8.0;
    (screen_x, screen_y)
}
