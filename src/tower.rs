//! Placed tower instances.

use crate::ids::{DesignId, TowerInstanceId};
use crate::map::Cell;

#[derive(Debug, Clone, PartialEq)]
pub struct PlacedTower {
    pub id: TowerInstanceId,
    pub design_id: DesignId,
    pub cell: Cell,
    /// Sell refund fraction applied later; stub. // STUB
    pub sell_value: u32,
}
