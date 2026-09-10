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

Requires `serde` (derive). Tests that need JSON roundtrip pull in `serde_json` as a **dev-dependency**.
