//! Loadout slot state.

use serde::{Deserialize, Serialize};

use crate::ids::DesignId;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LoadoutState {
    /// Slots; `None` means empty. Length equals unlocked slot count.
    pub slots: Vec<Option<DesignId>>,
}

impl LoadoutState {
    pub fn with_slots(n: usize) -> Self {
        Self {
            slots: vec![None; n],
        }
    }

    pub fn set_slot(&mut self, index: usize, design: Option<DesignId>) -> Result<(), ()> {
        if index >= self.slots.len() {
            return Err(());
        }
        self.slots[index] = design;
        Ok(())
    }
}
