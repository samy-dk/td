//! Minimal wave types (skeleton).

use serde::{Deserialize, Serialize};

use crate::enemy::EnemyKind;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WaveSpawn {
    pub kind: EnemyKind,
    pub count: u32,
    /// Delay between spawns in seconds. // STUB
    pub interval: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WaveDef {
    pub index: u32,
    pub spawns: Vec<WaveSpawn>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WaveState {
    pub current: Option<u32>,
    pub remaining_to_spawn: u32,
    pub elapsed: f32,
}
