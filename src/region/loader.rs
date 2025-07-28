use std::fs;

use crate::region::types::{Region};

pub fn load_region(path: &str) -> Region {
    let content = fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("No se pudo cargar región '{}'", path));
    serde_json::from_str(&content)
        .unwrap_or_else(|e| panic!("Error al parsear JSON '{}': {}", path, e))
}
