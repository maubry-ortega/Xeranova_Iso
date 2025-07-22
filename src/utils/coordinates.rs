pub const TILE_SIZE: f32 = 32.0;

pub fn to_screen_x(x: usize, y: usize) -> f32 {
    ((x as isize - y as isize) as f32) * TILE_SIZE / 2.0
}

pub fn to_screen_y(x: usize, y: usize) -> f32 {
    ((x as isize + y as isize) as f32) * TILE_SIZE / 4.0
}
