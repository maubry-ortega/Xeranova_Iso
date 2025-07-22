use crate::region::loader::Region;
use noise::{NoiseFn, Perlin};

pub fn generate(region: &Region) -> Vec<Vec<f32>> {
    let perlin = Perlin::new(42); // semilla fija
    let mut map = vec![vec![0.0; region.ancho]; region.alto];

    for y in 0..region.alto {
        for x in 0..region.ancho {
            let val = perlin.get([x as f64 * 0.1, y as f64 * 0.1]) as f32;
            map[y][x] = val;
        }
    }

    map
}
