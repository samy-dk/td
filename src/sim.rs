//! Core simulation API (inherent methods only; no GameApi trait).

use crate::design::{AssembledDesign, TowerDesign};
use crate::error::{ApiError, ValidateError};
use crate::events::GameEvent;
use crate::ids::{DesignId, PlaceSource, TowerInstanceId, UnlockKind};
use crate::loadout::LoadoutState;
use crate::map::{Cell, MapDef};
use crate::modules::ModuleCatalog;
use crate::research::ResearchState;
use crate::tower::PlacedTower;
use crate::validate;
use crate::wave::WaveState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunState {
    Ready,
    WaveActive,
    BetweenWaves,
    Defeat,
    Victory,
}

#[derive(Debug, Clone)]
pub struct SimState {
    pub run: RunState,
    pub lives: i32,
    pub gold: u32,
    pub research: ResearchState,
    pub loadout: LoadoutState,
    pub designs: Vec<AssembledDesign>,
    pub towers: Vec<PlacedTower>,
    pub wave: WaveState,
    pub map: MapDef,
}

pub struct Sim {
    catalog: ModuleCatalog,
    state: SimState,
    next_design_id: u64,
    next_tower_id: u64,
}

impl Sim {
    pub fn new(map: MapDef, catalog: ModuleCatalog) -> Self {
        let research = ResearchState::starter();
        let slots = research.loadout_slots;
        Self {
            catalog,
            state: SimState {
                run: RunState::Ready,
                lives: 20,  // STUB
                gold: 200,  // STUB
                research,
                loadout: LoadoutState::with_slots(slots),
                designs: Vec::new(),
                towers: Vec::new(),
                wave: WaveState {
                    current: None,
                    remaining_to_spawn: 0,
                    elapsed: 0.0,
                },
                map,
            },
            next_design_id: 1,
            next_tower_id: 1,
        }
    }

    pub fn catalog(&self) -> &ModuleCatalog {
        &self.catalog
    }

    pub fn state(&self) -> &SimState {
        &self.state
    }

    /// Structural validation only.
    pub fn validate_design(&self, design: &TowerDesign) -> Result<(), ValidateError> {
        validate::validate_structure(design, &self.catalog)
    }

    /// Validate structure + unlocks, register design, return DesignId.
    pub fn assemble_design(&mut self, design: TowerDesign) -> Result<DesignId, ApiError> {
        validate::validate_for_assemble(&design, &self.catalog, &self.state.research)?;
        let cost = design.cost(&self.catalog);
        let id = DesignId(self.next_design_id);
        self.next_design_id = self.next_design_id.saturating_add(1);
        self.state.designs.push(AssembledDesign {
            id,
            design,
            cost,
        });
        Ok(id)
    }

    pub fn set_loadout_slot(
        &mut self,
        slot: usize,
        design: Option<DesignId>,
    ) -> Result<(), ApiError> {
        if slot >= self.state.loadout.slots.len() {
            return Err(ApiError::InvalidSlot(slot));
        }
        if let Some(did) = design {
            if !self.state.designs.iter().any(|d| d.id == did) {
                return Err(ApiError::UnknownDesign(did.0));
            }
        }
        let _ = self.state.loadout.set_slot(slot, design);
        Ok(())
    }

    pub fn unlock(&mut self, kind: UnlockKind) -> Result<(), ApiError> {
        // Stub: free unlock for API skeleton; real game would spend research points.
        // STUB
        self.state.research.unlock(kind);
        if matches!(kind, UnlockKind::LoadoutSlot) {
            let n = self.state.research.loadout_slots;
            while self.state.loadout.slots.len() < n {
                self.state.loadout.slots.push(None);
            }
        }
        Ok(())
    }

    pub fn place_tower(&mut self, source: PlaceSource, cell: Cell) -> Result<TowerInstanceId, ApiError> {
        if !self.state.map.is_placeable(cell) {
            return Err(ApiError::InvalidPlacement);
        }
        if self.state.towers.iter().any(|t| t.cell == cell) {
            return Err(ApiError::InvalidPlacement);
        }

        let design_id = match source {
            PlaceSource::Design(id) => id,
            PlaceSource::LoadoutSlot(slot) => {
                let did = self
                    .state
                    .loadout
                    .slots
                    .get(slot)
                    .copied()
                    .flatten()
                    .ok_or(ApiError::InvalidSlot(slot))?;
                did
            }
        };

        let assembled = self
            .state
            .designs
            .iter()
            .find(|d| d.id == design_id)
            .ok_or(ApiError::UnknownDesign(design_id.0))?;

        if self.state.gold < assembled.cost {
            return Err(ApiError::InsufficientFunds);
        }

        let cost = assembled.cost;
        let sell_value = cost / 2; // STUB
        self.state.gold = self.state.gold.saturating_sub(cost);

        let id = TowerInstanceId(self.next_tower_id);
        self.next_tower_id = self.next_tower_id.saturating_add(1);
        self.state.towers.push(PlacedTower {
            id,
            design_id,
            cell,
            sell_value,
        });
        Ok(id)
    }

    pub fn sell_tower(&mut self, id: TowerInstanceId) -> Result<(), ApiError> {
        let idx = self
            .state
            .towers
            .iter()
            .position(|t| t.id == id)
            .ok_or(ApiError::UnknownTower(id.0))?;
        let tower = self.state.towers.remove(idx);
        self.state.gold = self.state.gold.saturating_add(tower.sell_value);
        Ok(())
    }

    pub fn start_wave(&mut self) -> Result<(), ApiError> {
        match self.state.run {
            RunState::Ready | RunState::BetweenWaves => {
                let next = self.state.wave.current.map(|c| c + 1).unwrap_or(0);
                self.state.wave.current = Some(next);
                self.state.wave.remaining_to_spawn = 10; // STUB
                self.state.wave.elapsed = 0.0;
                self.state.run = RunState::WaveActive;
                Ok(())
            }
            other => Err(ApiError::BadState(format!(
                "cannot start wave from {:?}",
                other
            ))),
        }
    }

    /// Stub tick: advances elapsed time; returns empty or minimal events.
    pub fn tick(&mut self, dt: f32) -> Vec<GameEvent> {
        if self.state.run != RunState::WaveActive {
            return Vec::new();
        }
        self.state.wave.elapsed += dt;
        // STUB: no real combat yet
        Vec::new()
    }
}
