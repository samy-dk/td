//! Closed module-ID enums (34 total; no stretch modules).
//! Serde derives for design / loadout persistence.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BaseId {
    Short,
    Mid,
    Long,
    Rapid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AimId {
    Single,
    Spray,
    Omni,
    Area,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PathId {
    Straight,
    Lob,
    Return,
    Seek,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProjectileId {
    Dart,
    Tack,
    Bomb,
    Glue,
    Ice,
    Ember,
    Spark,
    Bolt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DeliveryId {
    Travel,
    Hitscan,
    Explode,
    Pulse,
    Chain,
    Beam,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OnHitId {
    Pierce,
    Splash,
    Burn,
    Slow,
    Freeze,
    Stun,
    Electrify,
    Knockback,
}

/// Opaque design identity assigned by `Sim` on successful assemble.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DesignId(pub u64);

/// Opaque placed-tower instance identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TowerInstanceId(pub u64);

/// How a tower placement was sourced (loadout slot vs free assemble, etc.).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PlaceSource {
    LoadoutSlot(usize),
    Design(DesignId),
}

/// What kind of research unlock is being granted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UnlockKind {
    Base(BaseId),
    Aim(AimId),
    Path(PathId),
    Projectile(ProjectileId),
    Delivery(DeliveryId),
    OnHit(OnHitId),
    LoadoutSlot,
}
