use bevy::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Region {
    pub nombre: String,
    pub bioma: String,
    pub color: [f32; 3],
    pub ancho: usize,
    pub alto: usize,
    pub altura_base: i32,
    pub altura_max: usize,
    pub fondo_color: [f32; 3],
    pub textura: String,
}

pub fn load_region(path: &str) -> Region {
    std::fs::read_to_string(path)
        .and_then(|s| serde_json::from_str(&s).map_err(Into::into))
        .expect(&format!("No se pudo cargar región '{}'", path))
}
