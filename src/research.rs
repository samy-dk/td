//! Research / unlock state.

use crate::ids::{
    AimId, BaseId, DeliveryId, OnHitId, PathId, ProjectileId, UnlockKind,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResearchState {
    pub bases: Vec<BaseId>,
    pub aims: Vec<AimId>,
    pub paths: Vec<PathId>,
    pub projectiles: Vec<ProjectileId>,
    pub deliveries: Vec<DeliveryId>,
    pub on_hits: Vec<OnHitId>,
    /// Number of loadout slots unlocked (starter = 1).
    pub loadout_slots: usize,
}

impl ResearchState {
    /// Starter unlocks:
    /// Base::Short, Aim::Single, ALL Path variants, Projectile::Dart,
    /// Delivery::Travel, no OnHit, loadout_slots = 1.
    pub fn starter() -> Self {
        Self {
            bases: vec![BaseId::Short],
            aims: vec![AimId::Single],
            paths: vec![
                PathId::Straight,
                PathId::Lob,
                PathId::Return,
                PathId::Seek,
            ],
            projectiles: vec![ProjectileId::Dart],
            deliveries: vec![DeliveryId::Travel],
            on_hits: vec![],
            loadout_slots: 1,
        }
    }

    pub fn unlock(&mut self, kind: UnlockKind) {
        match kind {
            UnlockKind::Base(id) => {
                if !self.bases.contains(&id) {
                    self.bases.push(id);
                }
            }
            UnlockKind::Aim(id) => {
                if !self.aims.contains(&id) {
                    self.aims.push(id);
                }
            }
            UnlockKind::Path(id) => {
                if !self.paths.contains(&id) {
                    self.paths.push(id);
                }
            }
            UnlockKind::Projectile(id) => {
                if !self.projectiles.contains(&id) {
                    self.projectiles.push(id);
                }
            }
            UnlockKind::Delivery(id) => {
                if !self.deliveries.contains(&id) {
                    self.deliveries.push(id);
                }
            }
            UnlockKind::OnHit(id) => {
                if !self.on_hits.contains(&id) {
                    self.on_hits.push(id);
                }
            }
            UnlockKind::LoadoutSlot => {
                self.loadout_slots = self.loadout_slots.saturating_add(1);
            }
        }
    }

    pub fn is_base_unlocked(&self, id: BaseId) -> bool {
        self.bases.contains(&id)
    }
    pub fn is_aim_unlocked(&self, id: AimId) -> bool {
        self.aims.contains(&id)
    }
    pub fn is_path_unlocked(&self, id: PathId) -> bool {
        self.paths.contains(&id)
    }
    pub fn is_projectile_unlocked(&self, id: ProjectileId) -> bool {
        self.projectiles.contains(&id)
    }
    pub fn is_delivery_unlocked(&self, id: DeliveryId) -> bool {
        self.deliveries.contains(&id)
    }
    pub fn is_on_hit_unlocked(&self, id: OnHitId) -> bool {
        self.on_hits.contains(&id)
    }
}
