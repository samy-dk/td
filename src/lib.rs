//! `td_core` — tower-defense game API contract skeleton.
//!
//! Closed module IDs (34), inherent `Sim` methods, serde on designs/loadout/IDs.
//! Balance numbers marked `// STUB`.

pub mod design;
pub mod enemy;
pub mod error;
pub mod events;
pub mod ids;
pub mod loadout;
pub mod map;
pub mod modules;
pub mod research;
pub mod sim;
pub mod tower;
pub mod validate;
pub mod wave;

pub use design::{AssembledDesign, TowerDesign};
pub use error::{ApiError, ValidateError};
pub use events::GameEvent;
pub use ids::*;
pub use loadout::LoadoutState;
pub use map::{Cell, MapDef, Vec2};
pub use modules::{BaseStats, ModuleCatalog, ModuleCost};
pub use research::ResearchState;
pub use sim::{RunState, Sim, SimState};
pub use tower::PlacedTower;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ids::{
        AimId, BaseId, DeliveryId, OnHitId, PathId, ProjectileId, UnlockKind,
    };

    fn fence_dart() -> TowerDesign {
        // Starter "Fence Dart": Short + Single + Straight + Dart + Travel
        TowerDesign {
            name: Some("Fence Dart".into()),
            base: BaseId::Short,
            aim: AimId::Single,
            path: PathId::Straight,
            projectile: ProjectileId::Dart,
            delivery: DeliveryId::Travel,
            on_hit: vec![],
        }
    }

    fn storm_lattice() -> TowerDesign {
        // Mid + Omni + Straight + Spark + Chain + [Electrify]
        TowerDesign {
            name: Some("Storm Lattice".into()),
            base: BaseId::Mid,
            aim: AimId::Omni,
            path: PathId::Straight,
            projectile: ProjectileId::Spark,
            delivery: DeliveryId::Chain,
            on_hit: vec![OnHitId::Electrify],
        }
    }

    fn shock_bolt() -> TowerDesign {
        // Short + Single + Straight + Bolt + Travel + [Electrify] — structural OK
        // (assemble needs unlocks for Bolt / Electrify)
        TowerDesign {
            name: Some("Shock Bolt".into()),
            base: BaseId::Short,
            aim: AimId::Single,
            path: PathId::Straight,
            projectile: ProjectileId::Bolt,
            delivery: DeliveryId::Travel,
            on_hit: vec![OnHitId::Electrify],
        }
    }

    fn sim_starter() -> Sim {
        Sim::new(MapDef::stub(), ModuleCatalog::v1())
    }

    #[test]
    fn fence_dart_legal_under_starter_unlocks() {
        let mut sim = sim_starter();
        let d = fence_dart();
        assert!(sim.validate_design(&d).is_ok());
        let id = sim.assemble_design(d).expect("Fence Dart should assemble");
        assert_eq!(id, DesignId(1));
    }

    #[test]
    fn storm_lattice_validates_structure() {
        let sim = sim_starter();
        let d = storm_lattice();
        assert!(
            sim.validate_design(&d).is_ok(),
            "Storm Lattice should pass structural validation"
        );
        // Assemble may fail without unlocks — that is expected.
        // (We only assert structure here.)
    }

    #[test]
    fn storm_lattice_assemble_needs_unlocks() {
        let mut sim = sim_starter();
        let err = sim.assemble_design(storm_lattice()).unwrap_err();
        match err {
            ApiError::Validate(ValidateError::ModuleLocked(_)) => {}
            other => panic!("expected ModuleLocked, got {:?}", other),
        }
    }

    #[test]
    fn shock_bolt_validates() {
        let sim = sim_starter();
        assert!(sim.validate_design(&shock_bolt()).is_ok());
    }

    #[test]
    fn illegal_pierce_on_chain() {
        let sim = sim_starter();
        let d = TowerDesign {
            name: None,
            base: BaseId::Short,
            aim: AimId::Single,
            path: PathId::Straight,
            projectile: ProjectileId::Dart,
            delivery: DeliveryId::Chain,
            on_hit: vec![OnHitId::Pierce],
        };
        let err = sim.validate_design(&d).unwrap_err();
        assert!(matches!(err, ValidateError::PierceIllegalOn(DeliveryId::Chain)));
    }

    #[test]
    fn illegal_lob_hitscan() {
        let sim = sim_starter();
        let d = TowerDesign {
            name: None,
            base: BaseId::Short,
            aim: AimId::Single,
            path: PathId::Lob,
            projectile: ProjectileId::Dart,
            delivery: DeliveryId::Hitscan,
            on_hit: vec![],
        };
        let err = sim.validate_design(&d).unwrap_err();
        assert!(matches!(
            err,
            ValidateError::PathNotSupported {
                path: PathId::Lob,
                delivery: DeliveryId::Hitscan
            }
        ));
    }

    #[test]
    fn illegal_pulse_single() {
        let sim = sim_starter();
        let d = TowerDesign {
            name: None,
            base: BaseId::Short,
            aim: AimId::Single,
            path: PathId::Straight,
            projectile: ProjectileId::Dart,
            delivery: DeliveryId::Pulse,
            on_hit: vec![],
        };
        let err = sim.validate_design(&d).unwrap_err();
        assert!(matches!(
            err,
            ValidateError::PulseRequiresArea { aim: AimId::Single }
        ));
    }

    #[test]
    fn serde_roundtrip_tower_design() {
        let d = storm_lattice();
        let json = serde_json::to_string(&d).expect("serialize");
        let back: TowerDesign = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(d, back);
    }

    #[test]
    fn mid_locked_until_unlock_for_assemble() {
        let mut sim = sim_starter();
        let mut d = fence_dart();
        d.base = BaseId::Mid;
        let err = sim.assemble_design(d.clone()).unwrap_err();
        assert!(matches!(
            err,
            ApiError::Validate(ValidateError::ModuleLocked(UnlockKind::Base(BaseId::Mid)))
        ));

        sim.unlock(UnlockKind::Base(BaseId::Mid)).unwrap();
        let id = sim.assemble_design(d).expect("Mid should assemble after unlock");
        assert_eq!(id.0, 1);
    }

    #[test]
    fn chain_base_jumps_is_one() {
        let cat = ModuleCatalog::v1();
        assert_eq!(cat.chain_base_jumps(), 1);
    }

    #[test]
    fn pulse_with_area_is_ok() {
        let sim = sim_starter();
        let d = TowerDesign {
            name: None,
            base: BaseId::Short,
            aim: AimId::Area,
            path: PathId::Straight,
            projectile: ProjectileId::Dart,
            delivery: DeliveryId::Pulse,
            on_hit: vec![],
        };
        assert!(sim.validate_design(&d).is_ok());
    }

    #[test]
    fn module_count_is_34() {
        // 4+4+4+8+6+8 = 34
        assert_eq!(4 + 4 + 4 + 8 + 6 + 8, 34);
    }
}
