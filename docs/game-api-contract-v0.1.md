# Game API Contract — Draft v0.1 (Rust)

**Status:** DRAFT with **assumed locks** (API lock deferred; designer defaults applied 2026-09-07 per Chief of Staff). Override later OK. Derived from **Module Catalog v1 (LOCKED)** + pipeline  
`Base → Aim → Path → Projectile → Delivery → On-Hit`.  
**Scope:** Vertical slice. Signatures / types — not a full engine.  
**Owner:** LordDevin. Open items replaced by assumed locks in §8.

Research/design refs: `module-catalog-v1.md`, `modular-tower-composition-v0.3.md`, `LOCKS.md`.

---

## 0) Crate sketch

Suggested layout (one lib crate for slice; split later if needed):

```text
game_api/           # or td_core /
  src/
    lib.rs          # re-exports public contract
    ids.rs          # module IDs, entity IDs
    modules.rs      # static catalog + ModuleDef
    design.rs       # TowerDesign, validation
    loadout.rs      # saved compositions / slots
    research.rs     # unlock bitset / gates
    map.rs          # Map, path polyline, placeable cells
    enemy.rs        # EnemyDef, EnemyInstance, statuses
    wave.rs         # WaveDef, WaveRuntime
    tower.rs        # PlacedTower, fire cooldown
    sim.rs          # RunState, tick, commands
    events.rs       # GameEvent
    error.rs        # ApiError / ValidateError
```

Public surface: types + **`Sim` inherent methods** (assumed lock for v1). A `GameApi` trait can wait until a second consumer needs it.

---

## 1) Core types

### 1.1 Module IDs (v1 locked — do not expand)

```rust
/// Stable stringly IDs match catalog (`base_short`, …).
/// Newtype wrappers prevent cross-layer mixups.

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BaseId(pub &'static str);
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AimId(pub &'static str);
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PathId(pub &'static str);
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ProjectileId(pub &'static str);
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DeliveryId(pub &'static str);
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct OnHitId(pub &'static str);

// --- v1 catalog constants (34) ---
pub mod v1 {
    use super::*;

    // Base (4)
    pub const BASE_SHORT: BaseId = BaseId("base_short");
    pub const BASE_MID: BaseId = BaseId("base_mid");
    pub const BASE_LONG: BaseId = BaseId("base_long");
    pub const BASE_RAPID: BaseId = BaseId("base_rapid");

    // Aim (4)
    pub const AIM_SINGLE: AimId = AimId("aim_single");
    pub const AIM_SPRAY: AimId = AimId("aim_spray");
    pub const AIM_OMNI: AimId = AimId("aim_omni");
    pub const AIM_AREA: AimId = AimId("aim_area");

    // Path (4)
    pub const PATH_STRAIGHT: PathId = PathId("path_straight");
    pub const PATH_LOB: PathId = PathId("path_lob");
    pub const PATH_RETURN: PathId = PathId("path_return");
    pub const PATH_SEEK: PathId = PathId("path_seek");

    // Projectile (8)
    pub const PROJ_DART: ProjectileId = ProjectileId("proj_dart");
    pub const PROJ_TACK: ProjectileId = ProjectileId("proj_tack");
    pub const PROJ_BOMB: ProjectileId = ProjectileId("proj_bomb");
    pub const PROJ_GLUE: ProjectileId = ProjectileId("proj_glue");
    pub const PROJ_ICE: ProjectileId = ProjectileId("proj_ice");
    pub const PROJ_EMBER: ProjectileId = ProjectileId("proj_ember");
    pub const PROJ_SPARK: ProjectileId = ProjectileId("proj_spark");
    pub const PROJ_BOLT: ProjectileId = ProjectileId("proj_bolt");

    // Delivery (6)
    pub const DEL_TRAVEL: DeliveryId = DeliveryId("del_travel");
    pub const DEL_HITSCAN: DeliveryId = DeliveryId("del_hitscan");
    pub const DEL_EXPLODE: DeliveryId = DeliveryId("del_explode");
    pub const DEL_PULSE: DeliveryId = DeliveryId("del_pulse");
    pub const DEL_CHAIN: DeliveryId = DeliveryId("del_chain");
    pub const DEL_BEAM: DeliveryId = DeliveryId("del_beam");

    // On-Hit (8)
    pub const HIT_PIERCE: OnHitId = OnHitId("hit_pierce");
    pub const HIT_SPLASH: OnHitId = OnHitId("hit_splash");
    pub const HIT_BURN: OnHitId = OnHitId("hit_burn");
    pub const HIT_SLOW: OnHitId = OnHitId("hit_slow");
    pub const HIT_FREEZE: OnHitId = OnHitId("hit_freeze");
    pub const HIT_STUN: OnHitId = OnHitId("hit_stun");
    pub const HIT_ELECTRIFY: OnHitId = OnHitId("hit_electrify");
    pub const HIT_KNOCKBACK: OnHitId = OnHitId("hit_knockback");
}
```

**Assumed lock:** module IDs are **closed Rust `enum`s** per layer (exhaustiveness + serde).

Enum form (locked for slice):

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum BaseId { Short, Mid, Long, Rapid }

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum AimId { Single, Spray, Omni, Area }

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum PathId { Straight, Lob, Return, Seek }

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum ProjectileId { Dart, Tack, Bomb, Glue, Ice, Ember, Spark, Bolt }

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum DeliveryId { Travel, Hitscan, Explode, Pulse, Chain, Beam }

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum OnHitId { Pierce, Splash, Burn, Slow, Freeze, Stun, Electrify, Knockback }
```

### 1.2 Static module defs (data, not behavior)

```rust
pub struct ModuleCatalog { /* private maps */ }

impl ModuleCatalog {
    pub fn v1() -> Self { /* embed locked table */ }

    pub fn base(&self, id: BaseId) -> &BaseDef;
    pub fn aim(&self, id: AimId) -> &AimDef;
    pub fn path(&self, id: PathId) -> &PathDef;
    pub fn projectile(&self, id: ProjectileId) -> &ProjectileDef;
    pub fn delivery(&self, id: DeliveryId) -> &DeliveryDef;
    pub fn on_hit(&self, id: OnHitId) -> &OnHitDef;
}

pub struct BaseDef {
    pub id: BaseId,
    pub cost: u32,
    pub range: f32,
    pub fire_interval: f32, // seconds
}

pub struct AimDef {
    pub id: AimId,
    pub shape: AimShape,
    /// Omni projectile count, spray count, etc. None if N/A.
    pub count: Option<u8>,
}

pub enum AimShape { Single, Spray, Omni, AreaPulse }

pub struct PathDef {
    pub id: PathId,
    pub kind: PathKind,
}

pub enum PathKind { Straight, Lob, ReturnArc, Seeking }

pub struct ProjectileDef {
    pub id: ProjectileId,
    pub damage: f32,
    pub damage_tags: DamageTags, // sharp/fire/frost/lightning/… minimal bitflags
    pub cost: u32,
}

pub struct DeliveryDef {
    pub id: DeliveryId,
    pub mode: DeliveryMode,
    pub supports_path: bool,
    pub cost: u32,
}

pub enum DeliveryMode {
    Travel,
    Hitscan,
    Explode, // travel then AoE — may share travel phase
    Pulse,   // instant radius
    Chain { base_jumps: u8 }, // v1: 1
    Beam,
}

pub struct OnHitDef {
    pub id: OnHitId,
    pub cost: u32,
    // effect params live in sim application, not necessarily here
}
```

### 1.3 TowerDesign (composition)

```rust
pub const MAX_ON_HIT: usize = 2;

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TowerDesign {
    pub name: Option<String>,
    pub base: BaseId,
    pub aim: AimId,
    pub path: PathId,                 // default Straight when unused
    pub projectile: ProjectileId,
    pub delivery: DeliveryId,
    pub on_hit: Vec<OnHitId>,         // 0..=MAX_ON_HIT, ordered = apply order
}

impl TowerDesign {
    pub fn total_cost(&self, catalog: &ModuleCatalog) -> u32;
}
```

### 1.4 Identity / placement

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TowerInstanceId(pub u64);
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EnemyId(pub u64);
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DesignId(pub u64); // saved loadout entry

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CellCoord { pub x: i32, pub y: i32 }

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec2 { pub x: f32, pub y: f32 }
```

### 1.5 Map (minimal)

```rust
pub struct MapDef {
    pub id: String,
    pub width: u32,
    pub height: u32,
    pub path: Vec<Vec2>,          // enemy polyline
    pub placeable: Vec<CellCoord>, // or grid mask
}

pub struct MapState {
    pub def: MapDef,
    // occupancy derived from placed towers
}
```

### 1.6 Enemy / Wave (minimal)

```rust
pub struct EnemyDef {
    pub id: String,
    pub max_hp: f32,
    pub speed: f32,
    pub armor_tags: DamageTags, // immunities/resist — minimal
    pub reward: u32,
}

pub struct EnemyInstance {
    pub id: EnemyId,
    pub def_id: String,
    pub hp: f32,
    pub path_t: f32,              // 0..=1 along map path
    pub statuses: StatusSet,      // burn/slow/freeze/stun/electrify…
}

pub struct WaveDef {
    pub index: u32,
    pub spawns: Vec<SpawnEntry>,  // { enemy_def, count, interval }
}

pub struct WaveRuntime {
    pub def: WaveDef,
    pub elapsed: f32,
    pub spawned: u32,
    pub done_spawning: bool,
}
```

### 1.7 RunState

```rust
pub struct RunState {
    pub catalog: ModuleCatalog,
    pub map: MapState,
    pub research: ResearchState,
    pub loadout: LoadoutState,
    pub designs: DesignStore,       // DesignId -> TowerDesign
    pub towers: Vec<PlacedTower>,
    pub enemies: Vec<EnemyInstance>,
    pub projectiles: Vec<ActiveProjectile>, // travel/beam/chain runtime
    pub wave: Option<WaveRuntime>,
    pub wave_index: u32,
    pub cash: u32,
    pub lives: u32,
    pub tick_index: u64,
    pub time: f32,
    pub phase: RunPhase,
}

pub enum RunPhase {
    BetweenWaves,   // research / edit loadout / place
    WaveActive,
    Victory,
    Defeat,
}

pub struct PlacedTower {
    pub id: TowerInstanceId,
    pub design_id: DesignId,
    pub cell: CellCoord,
    pub cooldown: f32,
    pub facing: f32, // radians; unused for omni/area
}
```

### 1.8 Research / loadout

```rust
pub struct ResearchState {
    /// Unlocked module IDs — v1 can use bitflags or HashSet per layer.
    pub unlocked: Unlocks,
}

pub struct Unlocks {
    pub bases: EnumSet<BaseId>,       // or HashSet — DECISION on enum-set crate
    pub aims: EnumSet<AimId>,
    pub paths: EnumSet<PathId>,
    pub projectiles: EnumSet<ProjectileId>,
    pub deliveries: EnumSet<DeliveryId>,
    pub on_hits: EnumSet<OnHitId>,
    pub loadout_slots: u8,            // 1..N meta unlock
}

pub struct LoadoutState {
    pub slots: Vec<Option<DesignId>>, // len == loadout_slots
}

pub struct DesignStore { /* map DesignId -> TowerDesign */ }
```

**Assumed lock — starter unlocks** (wood-equivalent / Fence Dart set):
- Base: `Short`
- Aim: `Single`
- Path: **all four** (`Straight`, `Lob`, `Return`, `Seek`)
- Projectile: `Dart`
- Delivery: `Travel`
- On-Hit: none unlocked at start
- Loadout slots: **1**

Everything else unlocks via research between runs/waves.

---

## 2) Commands / `Sim` surface

**Assumed lock:** expose as **`impl Sim` methods** for v1.

```rust
// Method list — implement on Sim (trait optional later)
pub trait GameApi {
    // --- Design / validate ---
    fn assemble_design(&mut self, design: TowerDesign) -> Result<DesignId, ApiError>;
    fn validate_design(&self, design: &TowerDesign) -> Result<(), ValidateError>;
    fn design_cost(&self, design: &TowerDesign) -> Result<u32, ApiError>;

    // --- Loadout ---
    fn set_loadout_slot(&mut self, slot: u8, design: Option<DesignId>) -> Result<(), ApiError>;
    fn loadout(&self) -> &LoadoutState;

    // --- Research (between waves) ---
    fn unlock_module(&mut self, unlock: UnlockKind) -> Result<(), ApiError>;
    fn research(&self) -> &ResearchState;

    // --- Placement ---
    fn place_tower(&mut self, slot_or_design: PlaceSource, cell: CellCoord) -> Result<TowerInstanceId, ApiError>;
    fn sell_tower(&mut self, id: TowerInstanceId) -> Result<u32, ApiError>; // returns refund

    // --- Run loop ---
    fn start_wave(&mut self) -> Result<(), ApiError>;
    fn tick(&mut self, dt: f32) -> Vec<GameEvent>;

    // --- Introspection ---
    fn state(&self) -> &RunState;
}

pub enum PlaceSource {
    LoadoutSlot(u8),
    DesignId(DesignId),
}

pub enum UnlockKind {
    Base(BaseId),
    Aim(AimId),
    Path(PathId),
    Projectile(ProjectileId),
    Delivery(DeliveryId),
    OnHit(OnHitId),
    LoadoutSlot, // +1 slot
}
```

**Assemble semantics:**  
`assemble_design` validates + stores + returns `DesignId`. Does not place.  
Requires all modules unlocked (`ValidateError::ModuleLocked`).

**Tick semantics (slice):**  
Advance cooldowns, spawn wave enemies, targeting, spawn shots, move projectiles, resolve hits / statuses, leaks, wave-clear → `BetweenWaves`. Returns events for that frame (UI/audio later).

**Assumed for slice:** `tick` returns `Vec<GameEvent>`.

---

## 3) Events

```rust
#[derive(Clone, Debug)]
pub enum GameEvent {
    WaveStarted { index: u32 },
    WaveCleared { index: u32 },
    EnemySpawned { id: EnemyId, def_id: String },
    EnemyDamaged { id: EnemyId, amount: f32, source: TowerInstanceId },
    EnemyStatusApplied { id: EnemyId, status: StatusKind, source: TowerInstanceId },
    EnemyKilled { id: EnemyId, reward: u32 },
    EnemyLeaked { id: EnemyId },
    TowerFired { tower: TowerInstanceId, delivery: DeliveryId },
    ProjectileSpawned { /* id, kind */ },
    ProjectileHit { /* … */ },
    TowerPlaced { id: TowerInstanceId, cell: CellCoord },
    TowerSold { id: TowerInstanceId, refund: u32 },
    DesignSaved { id: DesignId },
    ModuleUnlocked { kind: UnlockKind },
    CashChanged { cash: u32 },
    LivesChanged { lives: u32 },
    RunVictory,
    RunDefeat,
}
```

Statuses for On-Hit mapping:

```rust
pub enum StatusKind { Burn, Slow, Freeze, Stun, Electrify }
// Pierce/splash/knockback are hit resolution, not necessarily lingering status
```

---

## 4) Validation API (illegal compositions)

```rust
pub fn validate_design(
    design: &TowerDesign,
    catalog: &ModuleCatalog,
    research: Option<&ResearchState>, // None = ignore unlocks (editor)
) -> Result<(), ValidateError>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ValidateError {
    TooManyOnHit { count: usize, max: usize },
    DuplicateOnHit { id: OnHitId },
    PathNotSupported { path: PathId, delivery: DeliveryId },
    PierceIllegal { delivery: DeliveryId },
    SplashIllegal { delivery: DeliveryId },
    PulseRequiresAreaAim { aim: AimId },
    ModuleLocked { layer: &'static str, /* id via Display */ },
    UnknownOrStretchModule, // should be unreachable if enums are closed
}

/// Rules (v1), mirrored from design docs:
/// 1. on_hit.len() <= 2; no duplicate OnHitId
/// 2. Path != Straight only if delivery.supports_path
///    - Travel, Explode: support path
///    - Hitscan, Pulse, Chain, Beam: Path must be Straight (or ignored; validate as Straight-only)
/// 3. hit_pierce illegal on Chain, Beam
/// 4. hit_splash illegal on Beam
/// 5. `Delivery::Pulse` **requires** `Aim::Area` — hard `ValidateError::PulseRequiresAreaAim`
/// 6. All module IDs must be unlocked if research provided
```

Suggested helper:

```rust
impl DeliveryId {
    pub fn supports_path(self) -> bool {
        matches!(self, DeliveryId::Travel | DeliveryId::Explode)
    }
}
```

Chain jumps: `DeliveryDef` carries `base_jumps: 1`. **Assumed lock:** jumps fixed at **1** in v1 (`Delivery::Chain` only; no chain-add On-Hit in the 34).
`DECISION`: confirm chain jump count fixed at 1 for v1 (catalog assumed lock had mod_chain; new pipeline has no separate chain-mod in 34 — **jumps = Delivery::Chain base only**).

---

## 5) Deliberately NOT in the API yet

| Out of scope | Why |
|--------------|-----|
| Render / sprites / camera | Client concern; consume `GameEvent` + `state()` |
| Audio | Same |
| Aura / Village / Farm / Spike Factory | Stretch systems — locked out of v1 catalog |
| Abilities / cooldowns buttons | Orthogonal system |
| Transforms / sacrifice / multi-pipeline towers | Post-v1 |
| Water / mobile bases | Stretch Bases |
| Hero / barracks blockers | KR steal — later |
| Full damage-immunity matrix polish | Minimal tags only in slice |
| Networking / replays / save format versioning | Can add `serde` on designs later |
| Exact numeric balance table | Catalog stubs; tune in data files |
| UI targeting priority / `TargetPriority` | **Deferred** (assumed lock) — not in v1 API |

---

## 6) Suggested Rust module layout + key signatures

```rust
// lib.rs
pub mod ids;
pub mod modules;
pub mod design;
pub mod validate;
pub mod loadout;
pub mod research;
pub mod map;
pub mod enemy;
pub mod wave;
pub mod tower;
pub mod sim;
pub mod events;
pub mod error;

pub use crate::sim::GameApi;
pub use crate::design::TowerDesign;
pub use crate::events::GameEvent;
pub use crate::error::{ApiError, ValidateError};
```

```rust
// error.rs
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ApiError {
    Validate(ValidateError),
    NotBetweenWaves,
    WaveAlreadyActive,
    CannotAfford { cost: u32, cash: u32 },
    CellNotPlaceable,
    CellOccupied,
    InvalidSlot,
    UnknownDesign,
    UnknownTower,
    ResearchUnavailable, // wrong phase or already unlocked
    RunOver,
}
```

```rust
// sim.rs — free functions acceptable alternative to trait
pub struct Sim {
    state: RunState,
}

impl Sim {
    pub fn new(map: MapDef, catalog: ModuleCatalog) -> Self;
    pub fn validate_design(&self, d: &TowerDesign) -> Result<(), ValidateError>;
    pub fn assemble_design(&mut self, d: TowerDesign) -> Result<DesignId, ApiError>;
    pub fn set_loadout_slot(&mut self, slot: u8, design: Option<DesignId>) -> Result<(), ApiError>;
    pub fn unlock(&mut self, kind: UnlockKind) -> Result<(), ApiError>;
    pub fn place_tower(&mut self, src: PlaceSource, cell: CellCoord) -> Result<TowerInstanceId, ApiError>;
    pub fn sell_tower(&mut self, id: TowerInstanceId) -> Result<u32, ApiError>;
    pub fn start_wave(&mut self) -> Result<(), ApiError>;
    pub fn tick(&mut self, dt: f32) -> Vec<GameEvent>;
    pub fn state(&self) -> &RunState;
}
```

**Internal (not public contract) for coding agent:** targeting query, projectile integration, status ticks — keep behind `sim` privacy.

---

## 7) Composition → runtime fire (informative)

For implementers; not extra public API:

1. Read `PlacedTower` + `TowerDesign`.
2. If cooldown ready and Aim finds target(s) in Base.range:
   - **Pulse:** apply projectile damage + on-hit in Aim radius; no Path.
   - **Hitscan:** ray to target; apply hit.
   - **Travel / Explode:** spawn `ActiveProjectile` with Path; on impact Explode adds splash radius.
   - **Chain:** hit primary, jump `base_jumps` times to nearby; on-hit each.
   - **Beam:** sustain lock, tick damage while valid.
3. Emit events.

---

## 8) Assumed locks (designer defaults — override later OK)

| # | Topic | Assumed lock |
|---|--------|--------------|
| 1 | Module IDs | Closed Rust **`enum`s** per layer |
| 2 | API surface | **`Sim` inherent methods** for v1 (not trait-first) |
| 3 | Chain jumps | **1** on `Delivery::Chain` only |
| 4 | Serde | **Day one** on designs / loadout / IDs |
| 5 | Pulse + Aim | `Delivery::Pulse` **requires** `Aim::Area` (hard fail) |
| 6 | TargetPriority | **Deferred** — out of v1 API |
| 7 | Starter unlocks | Fence Dart / wood-equivalent: `Short`+`Single`+all Paths+`Dart`+`Travel`, 0 on-hit unlocked, **1** loadout slot |

---

#