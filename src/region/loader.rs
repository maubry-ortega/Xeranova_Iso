use bevy::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Resource)]
pub struct Region {
    pub nombre: String,
    pub bioma: String,
    pub color: [f32; 3],
    pub ancho: usize,
    pub alto: usize,
}

pub fn load_region(path: &str) -> Region {
    std::fs::read_to_string(path)
        .and_then(|s| Ok(serde_json::from_str(&s).expect("JSON inválido")))
        .expect("No se pudo leer la región")
}
