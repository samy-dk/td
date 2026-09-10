# td_core

Tower-defense game **API contract skeleton** in Rust (edition 2021).

## Assumptions (locked)

1. Closed module-ID enums — **34** modules total (no stretch modules).
2. `Sim` exposes **inherent methods only** (no `GameApi` trait).
3. Chain jumps = **1** on `Delivery::Chain` (`ModuleCatalog::chain_base_jumps`).
4. Serde on designs, loadout, and module ID enums.
5. `Pulse` requires `Aim::Area` (hard `ValidateError`).
6. No `TargetPriority`.
7. Starter unlocks: `Base::Short`, `Aim::Single`, **all** `Path` variants, `Projectile::Dart`, `Delivery::Travel`, no OnHit, `loadout_slots = 1`.

## Layout

| Module | Role |
|--------|------|
| `ids` | Module ID enums, `DesignId`, `TowerInstanceId`, `PlaceSource`, `UnlockKind` |
| `modules` | `ModuleCatalog::v1()` stub costs / capabilities |
| `design` | `TowerDesign`, cost = sum of catalog module costs |
| `validate` | Structural + unlock validation |
| `research` / `loadout` | Unlock and loadout slot state |
| `map` / `enemy` / `wave` / `tower` | Grid map (`TileKind` + height), enemies, waves, towers |
| `sim` | `Sim::{new, validate_design, assemble_design, set_loadout_slot, unlock, place_tower, sell_tower, start_wave, tick, state}` |
| `events` / `error` | `GameEvent`, `ApiError`, `ValidateError` |

Balance numbers are marked `// STUB`.

## Workshop (Map Studio)

Phase A native desktop editor (`egui` / `eframe`) for authoring grid maps with height.

```bash
cargo run --bin td_workshop
```

Requires a display (X11). Headless CI: `cargo build --bin td_workshop` is enough.

### Map Studio features
- New map with configurable width × depth (default 16×12)
- Brushes: Path / Placeable / Blocked / Raise / Lower / Set Spawn / Set Leak
- Top-down paint grid (click-drag) colored by kind, tinted/labeled by height
- Fake-isometric extruded preview (stacked boxes by height)
- Rebuild path: orthogonal BFS from spawn across Path tiles (prefers reaching leak)
- `validate_path` errors shown in the status bar
- Save/Load `map.json` via path field (default `packs/dev_map.json`)

Tower Bench / Enemy Lab / Playtest menu entries are stubs (“Coming soon”).

### Toolchain note
This repo pins `eframe`/`egui` 0.27.2 (+ a few transitive crates) so it builds on **rustc 1.85**. Newer egui stacks need rustc ≥ 1.86.


## Validation rules

1. `on_hit` length ≤ 2, no duplicates  
2. `Path != Straight` only for `Travel` \| `Explode`; otherwise must be `Straight`  
3. `Pierce` illegal on `Chain` \| `Beam`  
4. `Splash` illegal on `Beam`  
5. `Pulse` requires `Aim::Area`  
6. Research unlock check when assembling  

## Build / test

```bash
cargo test
```

Requires `serde` (derive) and `serde_json` (Workshop save/load + tests).
