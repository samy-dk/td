//! Module catalog: costs and delivery capabilities. Balance numbers are stubs.

use crate::ids::{
    AimId, BaseId, DeliveryId, OnHitId, PathId, ProjectileId, UnlockKind,
};

/// Stub stats for a base module. // STUB
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BaseStats {
    pub cost: u32,
    pub range: f32,
    pub fire_interval: f32,
}

/// Catalog entry for a non-base module (cost only for now). // STUB
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModuleCost {
    pub cost: u32,
}

#[derive(Debug, Clone)]
pub struct ModuleCatalog {
    // Intentionally simple maps via match in methods for the skeleton.
}

impl ModuleCatalog {
    /// Version-1 stub catalog. // STUB
    pub fn v1() -> Self {
        Self {}
    }

    pub fn base_stats(&self, id: BaseId) -> BaseStats {
        // STUB
        match id {
            BaseId::Short => BaseStats {
                cost: 25,
                range: 80.0,
                fire_interval: 1.0,
            },
            BaseId::Mid => BaseStats {
                cost: 60,
                range: 120.0,
                fire_interval: 0.9,
            },
            BaseId::Long => BaseStats {
                cost: 90,
                range: 200.0,
                fire_interval: 1.4,
            },
            BaseId::Rapid => BaseStats {
                cost: 40,
                range: 90.0,
                fire_interval: 0.45,
            },
        }
    }

    pub fn aim_cost(&self, id: AimId) -> u32 {
        // STUB
        match id {
            AimId::Single => 5,
            AimId::Spray => 15,
            AimId::Omni => 25,
            AimId::Area => 30,
        }
    }

    pub fn path_cost(&self, id: PathId) -> u32 {
        // STUB
        match id {
            PathId::Straight => 5,
            PathId::Lob => 15,
            PathId::Return => 20,
            PathId::Seek => 25,
        }
    }

    pub fn projectile_cost(&self, id: ProjectileId) -> u32 {
        // STUB
        match id {
            ProjectileId::Dart => 5,
            ProjectileId::Tack => 10,
            ProjectileId::Bomb => 20,
            ProjectileId::Glue => 15,
            ProjectileId::Ice => 20,
            ProjectileId::Ember => 20,
            ProjectileId::Spark => 25,
            ProjectileId::Bolt => 30,
        }
    }

    pub fn delivery_cost(&self, id: DeliveryId) -> u32 {
        // STUB
        match id {
            DeliveryId::Travel => 5,
            DeliveryId::Hitscan => 15,
            DeliveryId::Explode => 25,
            DeliveryId::Pulse => 30,
            DeliveryId::Chain => 35,
            DeliveryId::Beam => 40,
        }
    }

    pub fn on_hit_cost(&self, id: OnHitId) -> u32 {
        // STUB
        match id {
            OnHitId::Pierce => 15,
            OnHitId::Splash => 20,
            OnHitId::Burn => 20,
            OnHitId::Slow => 15,
            OnHitId::Freeze => 25,
            OnHitId::Stun => 30,
            OnHitId::Electrify => 25,
            OnHitId::Knockback => 20,
        }
    }

    /// Chain base jump count. Locked assumption: jumps = 1. // STUB
    pub fn chain_base_jumps(&self) -> u32 {
        1
    }

    /// Deliveries that allow non-Straight paths.
    pub fn supports_path(&self, delivery: DeliveryId) -> bool {
        matches!(delivery, DeliveryId::Travel | DeliveryId::Explode)
    }

    pub fn unlock_kind_cost(&self, kind: UnlockKind) -> u32 {
        // STUB research costs reuse module costs loosely
        match kind {
            UnlockKind::Base(b) => self.base_stats(b).cost,
            UnlockKind::Aim(a) => self.aim_cost(a),
            UnlockKind::Path(p) => self.path_cost(p),
            UnlockKind::Projectile(p) => self.projectile_cost(p),
            UnlockKind::Delivery(d) => self.delivery_cost(d),
            UnlockKind::OnHit(o) => self.on_hit_cost(o),
            UnlockKind::LoadoutSlot => 50, // STUB
        }
    }
}
