//! Map definition.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Cell {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MapDef {
    pub width: u32,
    pub height: u32,
    /// Enemy path waypoints in world space.
    pub path: Vec<Vec2>,
    /// Cells where towers may be placed.
    pub placeable: Vec<Cell>,
}

impl MapDef {
    /// Tiny stub map for tests. // STUB
    pub fn stub() -> Self {
        Self {
            width: 10,
            height: 8,
            path: vec![
                Vec2::new(0.0, 4.0),
                Vec2::new(10.0, 4.0),
            ],
            placeable: vec![
                Cell { x: 3, y: 2 },
                Cell { x: 4, y: 2 },
                Cell { x: 5, y: 2 },
                Cell { x: 3, y: 5 },
                Cell { x: 4, y: 5 },
                Cell { x: 5, y: 5 },
            ],
        }
    }

    pub fn is_placeable(&self, cell: Cell) -> bool {
        self.placeable.contains(&cell)
    }
}
