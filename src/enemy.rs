//! Minimal enemy types (skeleton).

use serde::{Deserialize, Serialize};

use crate::map::Vec2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EnemyId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EnemyKind {
    Basic,
    Fast,
    Tank,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Enemy {
    pub id: EnemyId,
    pub kind: EnemyKind,
    pub hp: f32,
    pub max_hp: f32,
    pub speed: f32,
    pub path_t: f32,
    pub pos: Vec2,
}
