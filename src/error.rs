//! Public API error types. Public methods return these instead of panicking.

use crate::ids::{AimId, DeliveryId, OnHitId, PathId, UnlockKind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidateError {
    /// On-hit list longer than 2.
    TooManyOnHit { len: usize },
    /// Duplicate on-hit module.
    DuplicateOnHit(OnHitId),
    /// Non-Straight path used with a delivery that does not support pathing.
    PathNotSupported {
        path: PathId,
        delivery: DeliveryId,
    },
    /// Pierce is illegal on Chain or Beam.
    PierceIllegalOn(DeliveryId),
    /// Splash is illegal on Beam.
    SplashIllegalOn(DeliveryId),
    /// Pulse delivery requires Aim::Area.
    PulseRequiresArea { aim: AimId },
    /// A module in the design is not unlocked in research.
    ModuleLocked(UnlockKind),
    /// Generic / future validation failure.
    Other(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApiError {
    Validate(ValidateError),
    /// Loadout slot index out of range or not unlocked.
    InvalidSlot(usize),
    /// Design id unknown.
    UnknownDesign(u64),
    /// Tower instance unknown.
    UnknownTower(u64),
    /// Placement cell invalid / occupied / not placeable.
    InvalidPlacement,
    /// Not enough currency / resources (stub).
    InsufficientFunds,
    /// Wrong run phase for the requested action.
    BadState(String),
    /// Other API failure.
    Other(String),
}

impl From<ValidateError> for ApiError {
    fn from(v: ValidateError) -> Self {
        ApiError::Validate(v)
    }
}
