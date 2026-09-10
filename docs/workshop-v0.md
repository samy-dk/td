# Workshop — Design Brief v0.1

**Status:** Phase A direction locked by LordDevin (2026-09-10).

**Locks**
- **UI:** Native desktop editor first (egui/iced-class tool) — Option A
- **Maps:** **Grid-based**
- **Terrain:** Tiles can be **raised / lowered** (height) for a readable 3D-ish TD space
- Modules: still **Catalog v1 (34)** only; Workshop unlocks all for sandbox; legality via `td_core`

**Goal:** Creation + playtest space (you first, community packs later). Packs = data only.

**Repo:** https://github.com/samy-dk/td

---

## 1) Workshop tools

| Tool | Output |
|------|--------|
| **Map Studio** | Grid map with per-tile kind + **height** |
| **Tower Bench** | Legal `TowerDesign`s from 34 modules |
| **Enemy Lab** | `EnemyDef` + wave tables |
| **Playtest** | Place / run / reset against the pack |

Sandbox: all modules unlocked, editable cash/lives, validation still on.

---

## 2) Map model (grid + height)

### Grid
- Integer cells `(x, z)` on a rectangular grid (width × depth).
- Y is **up** (height). Rendering can be isometric or true 3D later; data is the same.

### Per-tile fields (Phase A)

```text
Tile {
  kind: Path | Placeable | Blocked
  height: i8    // elevation steps; 0 = default ground
}
```

**Raise / lower:** editor tools bump `height` by ±1 (clamp **-2..=4** in Phase A — locked).

### Path
- Ordered waypoints in cell space; path follows cell centers.
- **Slope rule:** path may step to an orthogonal neighbor if `|Δheight| <= 1`. Larger cliffs = blocked for ground enemies unless a bridge tile later.
- Spawn / leak mark cells (usually path ends).

### Placement
- Towers only on `Placeable` cells.
- **Height vs range (proposal for Phase A):** tower range measured in **horizontal** XZ distance; height does not yet change range. Optional later: height advantage bonus.
- Visual: towers sit on top of their tile’s height.

### Why height in A
Gives chokepoints, ramps, overlooks without new module systems — pure map craft.

---

## 3) Pack format

```text
my-pack/
  pack.toml
  map.json          # includes grid + tile heights
  enemies/*.json
  waves.json
  towers/*.json
  preview.png       # optional
```

Share as zip for players (Phase B); author as folder (Phase A).

---

## 4) Map Studio UX (Phase A)

- Brush: Path / Placeable / Blocked
- **Raise / Lower** brush (or scroll/keys on hovered tile)
- Height legend / contour tint so elevation reads in 2D top-down *and* in extruded preview
- Dual view ideal: **top-down edit** + **extruded 3D preview** (simple boxes OK)
- Path order tool, spawn/leak markers
- Grid size presets + custom

---

## 5) Phased delivery

**Phase A — Author kit:** Map (grid+height) + Tower + Enemy/Wave + Playtest + save/reload pack  
**Phase B — Community:** zip import/export, in-game Import, `td_workshop check`  
**Phase C — Discovery:** online listing later

---

## 6) Engineering implications

1. Extend `td_core::map::MapDef` from flat placeable list → **grid of tiles with `kind` + `height`**.
2. Path validation: connectivity + slope rule.
3. Flesh `Sim::tick` enough for playtest (dependency of A).
4. New binary/crate `td_workshop` (or `td_workshop` lib + `td_workshop_app` egui app).
5. Seed pack showing a small raised-ridge chokepoint map.

---

## 7) Still open (non-blocking defaults)

| Topic | Default unless you override |
|-------|-----------------------------|
| Height clamp | `-2..=4` steps |
| Cell size (world units) | `1.0` per cell |
| Range vs height | Horizontal only in A |
| Multi-path | Single path in A |
| Exact egui vs iced | Implementer pick; egui is fine |

---

## 8) Phase A success

- [ ] Raise a ridge, cut a ramp path through it  
- [ ] Place towers on high and low placeable tiles  
- [ ] Custom enemy waves playtest to win/lose  
- [ ] Save / reload pack preserves heights  
- [ ] Illegal tower still errors clearly  

— Game Design
