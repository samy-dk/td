//! Map definition: grid of tiles with kind + height (Phase A).

use serde::{Deserialize, Serialize};

/// Phase A elevation clamp (inclusive).
pub const HEIGHT_MIN: i8 = -2;
pub const HEIGHT_MAX: i8 = 4;

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

/// Grid cell on the XZ plane (`y` = depth axis).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Cell {
    pub x: i32,
    pub y: i32,
}

impl Cell {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TileKind {
    Path,
    Placeable,
    Blocked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tile {
    pub kind: TileKind,
    /// Elevation steps; default ground = 0. Clamped to [`HEIGHT_MIN`]..=[`HEIGHT_MAX`].
    pub height: i8,
}

impl Tile {
    pub fn new(kind: TileKind, height: i8) -> Self {
        Self {
            kind,
            height: height.clamp(HEIGHT_MIN, HEIGHT_MAX),
        }
    }

    pub fn path(height: i8) -> Self {
        Self::new(TileKind::Path, height)
    }

    pub fn placeable(height: i8) -> Self {
        Self::new(TileKind::Placeable, height)
    }

    pub fn blocked(height: i8) -> Self {
        Self::new(TileKind::Blocked, height)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PathValidationError {
    EmptyPath,
    OutOfBounds(Cell),
    NotOrthogonal { from: Cell, to: Cell },
    SteepSlope { from: Cell, to: Cell, delta: i8 },
    WrongKind { cell: Cell, kind: TileKind },
    SpawnMismatch,
    LeakMismatch,
}

/// Rectangular grid map: `width` × `depth` tiles with kind + height.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MapDef {
    pub width: u32,
    /// Depth along the second grid axis (was historically named `height`).
    pub depth: u32,
    /// Row-major tiles: index = `y * width + x`.
    pub tiles: Vec<Tile>,
    /// Ordered path waypoints in cell space (orthogonal steps).
    pub path: Vec<Cell>,
    pub spawn: Cell,
    pub leak: Cell,
}

impl MapDef {
    /// Tiny stub map for tests: 10×8 grid, eastbound path on row 4, placeable banks.
    pub fn stub() -> Self {
        let width = 10u32;
        let depth = 8u32;
        let mut tiles = vec![Tile::blocked(0); (width * depth) as usize];

        // Path along y=4 from x=0..=9
        let mut path = Vec::new();
        for x in 0..width as i32 {
            let cell = Cell::new(x, 4);
            path.push(cell);
            let idx = (4 * width as i32 + x) as usize;
            tiles[idx] = Tile::path(0);
        }

        // Placeable banks above/below the path
        for &(x, y) in &[
            (3, 2),
            (4, 2),
            (5, 2),
            (3, 5),
            (4, 5),
            (5, 5),
        ] {
            let idx = (y * width as i32 + x) as usize;
            tiles[idx] = Tile::placeable(0);
        }

        Self {
            width,
            depth,
            tiles,
            spawn: Cell::new(0, 4),
            leak: Cell::new(9, 4),
            path,
        }
    }

    pub fn in_bounds(&self, cell: Cell) -> bool {
        cell.x >= 0
            && cell.y >= 0
            && (cell.x as u32) < self.width
            && (cell.y as u32) < self.depth
    }

    fn index(&self, cell: Cell) -> Option<usize> {
        if !self.in_bounds(cell) {
            return None;
        }
        Some((cell.y as u32 * self.width + cell.x as u32) as usize)
    }

    pub fn get_tile(&self, cell: Cell) -> Option<Tile> {
        self.index(cell).map(|i| self.tiles[i])
    }

    pub fn set_tile(&mut self, cell: Cell, tile: Tile) -> bool {
        if let Some(i) = self.index(cell) {
            self.tiles[i] = Tile::new(tile.kind, tile.height);
            true
        } else {
            false
        }
    }

    /// Raise tile height by 1, clamped to [`HEIGHT_MAX`]. Returns new height, or `None` if OOB.
    pub fn raise(&mut self, cell: Cell) -> Option<i8> {
        let i = self.index(cell)?;
        let h = (self.tiles[i].height + 1).min(HEIGHT_MAX);
        self.tiles[i].height = h;
        Some(h)
    }

    /// Lower tile height by 1, clamped to [`HEIGHT_MIN`]. Returns new height, or `None` if OOB.
    pub fn lower(&mut self, cell: Cell) -> Option<i8> {
        let i = self.index(cell)?;
        let h = (self.tiles[i].height - 1).max(HEIGHT_MIN);
        self.tiles[i].height = h;
        Some(h)
    }

    pub fn is_placeable(&self, cell: Cell) -> bool {
        matches!(
            self.get_tile(cell),
            Some(Tile {
                kind: TileKind::Placeable,
                ..
            })
        )
    }

    fn is_orthogonal_step(a: Cell, b: Cell) -> bool {
        let dx = (a.x - b.x).abs();
        let dy = (a.y - b.y).abs();
        (dx == 1 && dy == 0) || (dx == 0 && dy == 1)
    }

    /// Connectivity + slope: consecutive path cells must be orthogonal neighbors with `|Δheight| <= 1`.
    /// Path cells must be `TileKind::Path`. Spawn/leak must match path ends.
    pub fn validate_path(&self) -> Result<(), PathValidationError> {
        if self.path.is_empty() {
            return Err(PathValidationError::EmptyPath);
        }

        for &cell in &self.path {
            if !self.in_bounds(cell) {
                return Err(PathValidationError::OutOfBounds(cell));
            }
            let tile = self.get_tile(cell).unwrap();
            if tile.kind != TileKind::Path {
                return Err(PathValidationError::WrongKind {
                    cell,
                    kind: tile.kind,
                });
            }
        }

        for w in self.path.windows(2) {
            let from = w[0];
            let to = w[1];
            if !Self::is_orthogonal_step(from, to) {
                return Err(PathValidationError::NotOrthogonal { from, to });
            }
            let h0 = self.get_tile(from).unwrap().height;
            let h1 = self.get_tile(to).unwrap().height;
            let delta = (h0 - h1).abs();
            if delta > 1 {
                return Err(PathValidationError::SteepSlope { from, to, delta });
            }
        }

        if self.spawn != *self.path.first().unwrap() {
            return Err(PathValidationError::SpawnMismatch);
        }
        if self.leak != *self.path.last().unwrap() {
            return Err(PathValidationError::LeakMismatch);
        }

        Ok(())
    }

    /// World-space center of a cell (XZ; Y elevation separate).
    pub fn cell_center(&self, cell: Cell) -> Option<Vec2> {
        if !self.in_bounds(cell) {
            return None;
        }
        Some(Vec2::new(cell.x as f32 + 0.5, cell.y as f32 + 0.5))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raise_lower_clamp() {
        let mut map = MapDef::stub();
        let c = Cell::new(3, 2);
        assert_eq!(map.get_tile(c).unwrap().height, 0);

        for _ in 0..10 {
            map.raise(c);
        }
        assert_eq!(map.get_tile(c).unwrap().height, HEIGHT_MAX);

        for _ in 0..20 {
            map.lower(c);
        }
        assert_eq!(map.get_tile(c).unwrap().height, HEIGHT_MIN);

        map.raise(c);
        assert_eq!(map.get_tile(c).unwrap().height, HEIGHT_MIN + 1);
    }

    #[test]
    fn illegal_steep_path_step() {
        let mut map = MapDef::stub();
        // Flat path then a cliff: last step 0 → 2.
        let a = Cell::new(8, 4);
        let b = Cell::new(9, 4);
        map.raise(b);
        map.raise(b); // height 2; neighbor a stays 0
        assert_eq!(map.get_tile(a).unwrap().height, 0);
        assert_eq!(map.get_tile(b).unwrap().height, 2);
        let err = map.validate_path().unwrap_err();
        match err {
            PathValidationError::SteepSlope { from, to, delta } => {
                assert_eq!(from, a);
                assert_eq!(to, b);
                assert_eq!(delta, 2);
            }
            other => panic!("expected SteepSlope, got {:?}", other),
        }
    }

    #[test]
    fn legal_ramp_path() {
        let mut map = MapDef::stub();
        // Gentle ramp: 0 → 1 → 2 → 1 → 0 along path
        let heights = [0i8, 0, 1, 2, 1, 0, 0, 0, 0, 0];
        for (x, h) in heights.iter().enumerate() {
            let cell = Cell::new(x as i32, 4);
            let mut t = map.get_tile(cell).unwrap();
            t.height = *h;
            map.set_tile(cell, t);
        }
        assert!(map.validate_path().is_ok());
    }

    #[test]
    fn stub_path_validates() {
        assert!(MapDef::stub().validate_path().is_ok());
    }

    #[test]
    fn serde_roundtrip_map() {
        let map = MapDef::stub();
        let json = serde_json::to_string(&map).expect("serialize");
        let back: MapDef = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(map, back);
    }
}
