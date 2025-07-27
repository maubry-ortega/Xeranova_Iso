use bevy::prelude::*;

/// Velocidad lineal en unidades por segundo
#[derive(Component, Debug, Default)]
pub struct Velocity(pub Vec3);