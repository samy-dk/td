//! Tower design composition and cost.

use serde::{Deserialize, Serialize};

use crate::ids::{AimId, BaseId, DeliveryId, DesignId, OnHitId, PathId, ProjectileId};
use crate::modules::ModuleCatalog;

/// A complete tower design recipe.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TowerDesign {
    pub name: Option<String>,
    pub base: BaseId,
    pub aim: AimId,
    pub path: PathId,
    pub projectile: ProjectileId,
    pub delivery: DeliveryId,
    /// Ordered, unique; length 0..=2.
    pub on_hit: Vec<OnHitId>,
}

impl TowerDesign {
    pub fn cost(&self, catalog: &ModuleCatalog) -> u32 {
        let mut total = catalog.base_stats(self.base).cost;
        total += catalog.aim_cost(self.aim);
        total += catalog.path_cost(self.path);
        total += catalog.projectile_cost(self.projectile);
        total += catalog.delivery_cost(self.delivery);
        for oh in &self.on_hit {
            total += catalog.on_hit_cost(*oh);
        }
        total
    }
}

/// An assembled design registered in a sim run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssembledDesign {
    pub id: DesignId,
    pub design: TowerDesign,
    pub cost: u32,
}
