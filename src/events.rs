//! Simulation events emitted by `Sim::tick`.

use crate::enemy::EnemyId;
use crate::ids::TowerInstanceId;
use crate::map::Cell;

#[derive(Debug, Clone, PartialEq)]
pub enum GameEvent {
    WaveStarted { index: u32 },
    WaveCleared { index: u32 },
    EnemySpawned { id: EnemyId },
    EnemyDied { id: EnemyId },
    EnemyLeaked { id: EnemyId },
    TowerFired { tower: TowerInstanceId },
    TowerPlaced { tower: TowerInstanceId, cell: Cell },
    TowerSold { tower: TowerInstanceId },
    /// Stub / catch-all for future events.
    Other(String),
}
