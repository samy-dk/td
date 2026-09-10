//! Structural and unlock validation for tower designs.

use crate::design::TowerDesign;
use crate::error::ValidateError;
use crate::ids::{DeliveryId, OnHitId, PathId, UnlockKind};
use crate::modules::ModuleCatalog;
use crate::research::ResearchState;

/// Structural rules only (no research unlocks).
pub fn validate_structure(
    design: &TowerDesign,
    catalog: &ModuleCatalog,
) -> Result<(), ValidateError> {
    if design.on_hit.len() > 2 {
        return Err(ValidateError::TooManyOnHit {
            len: design.on_hit.len(),
        });
    }

    // Duplicate on-hit check (order-preserving uniqueness).
    for (i, a) in design.on_hit.iter().enumerate() {
        for b in design.on_hit.iter().skip(i + 1) {
            if a == b {
                return Err(ValidateError::DuplicateOnHit(*a));
            }
        }
    }

    // Path != Straight only for Travel | Explode; else must be Straight.
    if design.path != PathId::Straight && !catalog.supports_path(design.delivery) {
        return Err(ValidateError::PathNotSupported {
            path: design.path,
            delivery: design.delivery,
        });
    }

    // Pierce illegal on Chain | Beam.
    if design.on_hit.contains(&OnHitId::Pierce)
        && matches!(design.delivery, DeliveryId::Chain | DeliveryId::Beam)
    {
        return Err(ValidateError::PierceIllegalOn(design.delivery));
    }

    // Splash illegal on Beam.
    if design.on_hit.contains(&OnHitId::Splash) && design.delivery == DeliveryId::Beam {
        return Err(ValidateError::SplashIllegalOn(design.delivery));
    }

    // Pulse requires Aim::Area.
    if design.delivery == DeliveryId::Pulse && design.aim != crate::ids::AimId::Area {
        return Err(ValidateError::PulseRequiresArea { aim: design.aim });
    }

    let _ = catalog.chain_base_jumps(); // document lock: Chain jumps = 1
    Ok(())
}

/// Structural + research unlock checks (used when assembling).
pub fn validate_for_assemble(
    design: &TowerDesign,
    catalog: &ModuleCatalog,
    research: &ResearchState,
) -> Result<(), ValidateError> {
    validate_structure(design, catalog)?;
    check_unlocks(design, research)
}

fn check_unlocks(design: &TowerDesign, research: &ResearchState) -> Result<(), ValidateError> {
    if !research.is_base_unlocked(design.base) {
        return Err(ValidateError::ModuleLocked(UnlockKind::Base(design.base)));
    }
    if !research.is_aim_unlocked(design.aim) {
        return Err(ValidateError::ModuleLocked(UnlockKind::Aim(design.aim)));
    }
    if !research.is_path_unlocked(design.path) {
        return Err(ValidateError::ModuleLocked(UnlockKind::Path(design.path)));
    }
    if !research.is_projectile_unlocked(design.projectile) {
        return Err(ValidateError::ModuleLocked(UnlockKind::Projectile(
            design.projectile,
        )));
    }
    if !research.is_delivery_unlocked(design.delivery) {
        return Err(ValidateError::ModuleLocked(UnlockKind::Delivery(
            design.delivery,
        )));
    }
    for oh in &design.on_hit {
        if !research.is_on_hit_unlocked(*oh) {
            return Err(ValidateError::ModuleLocked(UnlockKind::OnHit(*oh)));
        }
    }
    Ok(())
}
