use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct SelectedMapSize(pub Option<MapSize>);

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapSize {
    Small,
    Medium,
    Large,
}

impl MapSize {
    pub fn dimensions(self) -> (u32, u32) {
        match self {
            MapSize::Small => (2, 2),
            MapSize::Medium => (3, 3),
            MapSize::Large => (4, 4),
        }
    }
}
