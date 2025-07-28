use bevy::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct UniqueFractus {
    pub name: String,
    pub effect: String,
    pub evolution_stage: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Region {
    pub name: String,
    pub biome: String,
    pub base_color: [f32; 3],
    pub background_color: [f32; 3],
    pub texture_id: String,
    pub width: usize,
    pub height: usize,
    pub elevation_min: i32,
    pub elevation_max: usize,

    pub fractus_origin: String,
    pub physical_anomalies: Vec<String>,
    pub khaon_virus_level: String,
    pub pulse_visibility: bool,

    pub factions_present: Vec<String>,
    pub unique_fractus: Vec<UniqueFractus>,
    pub environmental_effects: Vec<String>,
    pub dynamic_events: Vec<String>,
    pub tags: Vec<String>,
}
