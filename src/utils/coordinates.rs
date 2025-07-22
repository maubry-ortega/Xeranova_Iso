pub const TILE_SIZE: f32 = 32.0;

/// Proyección isométrica (tipo TutsPlus)
pub fn to_screen_x(x: i32, y: i32) -> f32 {
    (x - y) as f32 * TILE_SIZE / 2.0
}

pub fn to_screen_y(x: i32, y: i32) -> f32 {
    (x + y) as f32 * TILE_SIZE / 4.0
}
