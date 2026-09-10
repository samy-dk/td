# Module Catalog v1 — LOCKED

**Status:** LOCKED by LordDevin (2026-09-07). Official vertical-slice module set (**34 modules**).  
**Do not expand** the six-layer catalog without his say. Stretch / gap systems (aura, farm, spike, abilities, transforms, multi-pipeline, mobile bases, etc.) stay **out of v1**.  
**Next (when asked):** turn this into an API contract.

**Pipeline (working names):** Base → Aim → Path → Projectile → Delivery → On-Hit  
**Gold standard researched:** Bloons TD 6 (BTD5 same families, 2 paths). Also skimmed Kingdom Rush, PvZ, Infinitode.  
**Research pack:** `/workspace/design/research-td-mechanics.md`  
**Goal:** Concrete modules to implement — not more pipeline debate.

**Legend:** `[S]` = slice / v1 implement first · `[X]` = stretch (post-slice)

---

## 1) Short research notes — BTD families (mechanics only)

| Family | What it really does |
|--------|---------------------|
| **Dart** | Single aim, traveling dart, low pierce. Upgrades flip to spray×3, high-pierce rebound balls, or crossbow bolts+crits. Ability can temp-transform nearby Darts. |
| **Tack** | Fixed omni N-way short radial volleys. Can become flame **ring burst** (area) or extreme omni count (Tack Zone). |
| **Boomerang** | Curved **return path**; or straight heavy “Kylie”; or **ricochet target-to-target**; or orbiting glaive zone. Knockback on MOABs. |
| **Bomb** | Travel → **impact explosion**. Clusters/frags cascade. Stun, anti-blimp missile ability, leak-triggered screen clear. |
| **Ice** | Omni **area freeze pulse** (status tower). Paths: permafrost/brittle amp, slow aura, global freeze ability, or ranged ice bombs / anti-MOAB icicles. |
| **Glue** | Travel glue blob, **slow with little/no base damage**. Corrosive DoT, splatter, MOAB glue, global glue storm ability. |
| **Sniper** | Infinite-range **hitscan**. Shrapnel cone, ricochet bounce, MOAB maim/cripple, cash drops, rate scales with round progress. |
| **Sub** | Water; **seeking** torps; intel range-share; submerge camo aura; missiles; airburst split; commander buff. |
| **Buccaneer** | Water; multi-weapon (darts + grapes + planes); income path; hook-remove MOAB; carrier summons. |
| **Ace** | **Mobile** flight-path platform; omni dart carpets; bombers; seeking streams; screen nukes. |
| **Heli** | Player-directed hover; contact rotors; blowback; gunship; **relocate other towers**; permanent mini escorts. |
| **Mortar** | **Ground reticle** + lob + scatter explosion. Stun, burn, decamo, strip properties, track fire, screen ability. |
| **Dartling** | Cursor/locked aim + spread; escalates dart → laser shock → plasma **beam** → Ray of Doom; or rocket storm / mega anti-BAD. |
| **Wizard** | Bolts; seeking; fireball; **Wall of Fire track zone**; dragon breath; shimmer decamo; necro summons; phoenix transform; Archmage hybrid. |
| **Super** | Hypersonic stream; laser/plasma/sun; Temple **sacrifice** system; robo dual-aim; darkricochet + leak black-hole. |
| **Ninja** | Innate camo; shuriken spray; seeking; caltrops on track; flash stun; sticky MOAB bomb; shinobi aura; global sabotage slow. |
| **Alchemist** | Acid flasks (ignore LOS) + **ally-aimed brews**; permanent buffs; lead→gold; shrink; transform ability. |
| **Druid** | Thorn spray; **chain lightning** + tornado blowback; vine grab + thorn piles; density-scaling wrath. |
| **Farm** | No attack — income only. |
| **Spike Factory** | Passive **track hazard piles** (pierce-as-HP); mines; whole-track carpets; multi-round perma-spikes. |
| **Village** | Support auras (range, AS, camo, MIB all-types, discounts, cash-on-pop); absorbs farms (Monkeyopolis). |
| **Engineer** | Nails + **sentry summons**; foam strip; pin; overclock buff; bloon trap → cash. |
| **Beast Handler** | Controllable beast sub-units + merge power — modularize poorly. |

---

## 2) Other TDs worth stealing (brief)

| Game | Steal for modules |
|------|-------------------|
| **Kingdom Rush** | Barracks **body-block / rally point** (blocker Aim+Delivery outlier); armor vs magic resist tags; heroes as mobile towers (post-slice). |
| **PvZ** | Lane-locked Aim; multi-lane spray; lob pults; Torchwood-style **projectile rewrite mid-flight** (stretch); Spikeweed = track hazard; Wall-nut blockers; global freeze shroom. |
| **Infinitode** | Explicit targeting priority enum; maze + tower as pathing; modifier tiles (meta, not modules). |
| **Arknights / Defense Grid** | Block count; redeploy; funnelling — map/design, not shot pipeline. |

---

## 3) Proposed MODULE LIST by layer

### Base — structure / range `[S]` unless noted

| ID | Name | Role |
|----|------|------|
| `base_short` | Short Mount | Short range, cheap, med fire rate |
| `base_mid` | Standard Mount | Med range / rate |
| `base_long` | Long Mount | Long range, slower |
| `base_rapid` | Rapid Frame | Short–med range, high fire rate, fragile budget |
| `base_heavy` | Heavy Emplacement | Med range, slow, high projectile budget |
| `base_water` | Water Platform `[X]` | Placeable on water only |
| `base_mobile_orbit` | Orbit Rail `[X]` | Ace-like flight path mover |
| `base_mobile_hover` | Hover Rig `[X]` | Heli-like directed hover |
| `base_support` | Support Hub `[X]` | No inherent shot; aura host (Village-like) |
| `base_producer` | Producer Frame `[X]` | Farm / spike factory host |

### Aim — who/where `[S]` unless noted

| ID | Name | Shape |
|----|------|-------|
| `aim_single` | Pinpoint | One target (First/Last/Close/Strong priority lives here or in UI) |
| `aim_spray` | Spray | Multi-projectile fan / triple-shot style |
| `aim_cone` | Cone | Cone volume |
| `aim_omni` | Omni | Full radial N-way (param: count 8/12/16) |
| `aim_area` | Pulse Zone | Radius around tower (Ice pulse) |
| `aim_reticle` | Reticle `[X]` | Player/ground-aimed patch (Mortar) |
| `aim_cursor` | Cursor Lock `[X]` | Dartling-style aim at pointer |
| `aim_ally` | Ally Aim `[X]` | Targets friendly towers (Alchemist brew) |
| `aim_track` | Track Segment `[X]` | Pick path segment (WoF / spike drop) |

### Path — route through space `[S]` unless noted

| ID | Name | Motion |
|----|------|--------|
| `path_straight` | Straight | Default linear |
| `path_lob` | Lob | Arcing lob (mortar / pult) |
| `path_return` | Return Arc | Boomerang curve back |
| `path_seek` | Seeking | Homes on target (sub torp / guided) |
| `path_ricochet` | Ricochet | Bounce target→target or off obstacles |
| `path_spiral` | Spiral `[X]` | Spiral flourish |
| `path_wave` | Wave `[X]` | Sine weave |
| `path_split` | Split `[X]` | Mid/on-hit splits into children (Ultra-Jug / airburst) — *may belong On-Hit; flag* |

### Projectile — what it is `[S]` unless noted

| ID | Name | Identity |
|----|------|----------|
| `proj_dart` | Dart | Sharp kinetic baseline |
| `proj_tack` | Tack | Tiny sharp (omni ammo) |
| `proj_blade` | Blade | Heavier sharp / shatter frozen |
| `proj_bolt` | Bolt | Sniper/crossbow kinetic |
| `proj_bomb` | Bomb | Explosive payload |
| `proj_missile` | Missile | Fast explosive / anti-heavy |
| `proj_glue` | Glue Glob | Status carrier, low damage |
| `proj_ice` | Ice Shard | Cold carrier |
| `proj_ember` | Ember | Fire carrier |
| `proj_spark` | Spark | Lightning carrier |
| `proj_shuriken` | Shuriken | Fast sharp + camo-friendly fantasy |
| `proj_thorn` | Thorn | Druid-style sharp |
| `proj_acid` | Acid Flask `[X]` | Ignores LOS fantasy / melt |
| `proj_laser` | Laser `[X]` | Energy bolt / beam ammo |
| `proj_plasma` | Plasma `[X]` | Hot energy |
| `proj_spike_ball` | Spike Ball `[X]` | High-pierce juggernaut ball |
| `proj_glaive` | Glaive `[X]` | Boomerang/ricochet fantasy |
| `proj_brew` | Brew Potion `[X]` | Ally buff projectile |

### Delivery — how it arrives `[S]` unless noted

| ID | Name | Mode |
|----|------|------|
| `del_travel` | Launch | Traveling entity (Path applies) |
| `del_hitscan` | Hitscan | Instant (Path N/A) |
| `del_beam` | Beam | Sustained line / lock tick |
| `del_chain` | Chain | Jump channel primary→nearby |
| `del_explode` | Impact Blast | Travel then AoE explosion at impact |
| `del_pulse` | Omni Pulse | Instant radius pulse (Ice) — pairs with `aim_area` |
| `del_ring` | Ring Burst `[X]` | Omni flame/tack ring (Inferno) |
| `del_orbit` | Orbit Zone `[X]` | Contact damage around tower (Glaive Lord / rotors) |
| `del_zone_track` | Track Zone `[X]` | Persistent fire/glue/spike on path |
| `del_attach` | Attach `[X]` | Stick then detonate (sticky bomb) |
| `del_summon` | Summon `[X]` | Spawn sentry/minion/plane (post-slice system) |

### On-Hit — statuses / effects · ordered · cap 2 in slice `[S]` unless noted

| ID | Name | Effect |
|----|------|--------|
| `hit_pierce` | Pierce | +N pierce / continue |
| `hit_splash` | Splash | Small AoE at contact |
| `hit_burn` | Burn | Fire DoT |
| `hit_slow` | Slow | Move speed down |
| `hit_freeze` | Freeze | Stop + frost rules |
| `hit_stun` | Stun | Brief hard CC |
| `hit_electrify` | Electrify | Shock tick / lightning status |
| `hit_knockback` | Knockback | Push along / off path |
| `hit_frag` | Frag Spray | Spawn secondary sharp bits |
| `hit_decamo` | Reveal `[X]` | Strip camo / shimmer |
| `hit_brittle` | Brittle `[X]` | Amp incoming damage |
| `hit_corrosive` | Corrode `[X]` | Acid DoT |
| `hit_immobilize` | Pin `[X]` | Soft/hard root (glue/maim) |
| `hit_crit` | Crit `[X]` | Chance multihit damage |
| `hit_strip` | Strip `[X]` | Remove fortified/lead/regrow props |
| `hit_cash` | Bounty `[X]` | Cash on pop |

---

## 4) Mapping table — notable BTD towers → our modules

| BTD build | Base | Aim | Path | Projectile | Delivery | On-Hit |
|-----------|------|-----|------|------------|----------|--------|
| Dart 0-0-0 | short/mid | single | straight | dart | travel | pierce(1) |
| Triple Shot | mid | **spray** | straight | dart | travel | pierce |
| Juggernaut | heavy | single | lob/straight+bounce* | spike_ball | travel | pierce(high) |
| Tack 0-0-0 | short | **omni** | straight | tack | travel | — |
| Tack Zone | short+ | omni(16) | straight | tack | travel | — |
| Inferno Ring | short | area/omni | — | ember | **ring** / pulse | burn |
| Boomerang 0-0-0 | mid | single | **return** | glaive | travel | pierce |
| Glaive Ricochet | mid | single | **ricochet** | glaive | travel | pierce |
| Bomb 0-0-0 | mid | single | straight | bomb | **explode** | splash |
| Blooncrush | heavy | single | straight | bomb | explode | stun+splash |
| Ice 0-0-0 | short | **area** | — | ice | **pulse** | freeze |
| Super Brittle | short | area | — | ice | pulse | freeze+brittle |
| Glue 0-0-0 | mid | single | straight | glue | travel | slow |
| Sniper 0-0-0 | long∞ | single | — | bolt | **hitscan** | — |
| Cripple MOAB | long∞ | single | — | bolt | hitscan | immobilize |
| Sub seeking | water | single | **seek** | missile/torp | travel | — |
| Mortar 0-0-0 | heavy | **reticle** | **lob** | bomb | explode | splash |
| Blooncineration | heavy | reticle | lob | ember | explode+zone_track | burn+strip |
| Dartling laser | rapid | **cursor** | — | laser | beam | electrify? |
| Ray of Doom | rapid | cursor | — | plasma | beam | pierce line |
| Ninja Bloonjitsu | mid | spray | straight | shuriken | travel | pierce |
| Flash Bomb | mid | single | straight | shuriken/bomb | travel/explode | stun |
| Druid Thunder | mid | single | — | spark | **chain** | electrify |
| Wizard WoF | mid | track | — | ember | zone_track | burn |
| Spike Factory | producer | track | — | tack/spike | zone_track | pierce-as-HP* |
| Village MIB | support | — | — | — | — | *(aura system)* |

\*bounce / pierce-as-HP / auras = see Gaps.

**Contrast proof (ours):**  
- Lightning channel: `proj_spark` + `del_chain` (+ optional `hit_electrify`)  
- Shock bolt: `proj_spark` + `del_travel` + `path_straight` + `hit_electrify`

---

## 5) Gaps — BTD things that don’t fit cleanly

Flag for LordDevin (do **not** force into the 6 shot layers):

1. **Support auras / Village / drum / MIB / discounts** — need a **Passive/Aura** system or Support Base, not On-Hit.
2. **Income / Farms / Merchantman / Supply Drop** — economy modules or meta buildings.
3. **Spike Factory / caltrops / WoF** — **persistent track hazards** (producer + zone), not a single shot event.
4. **Abilities** (Fan Club, Ground Zero, Sabotage, Overclock) — orthogonal **Ability** slot / cooldown button.
5. **Transforms** (Phoenix, Fan Club supers, Temple, Tonic) — full pipeline swap; not a module.
6. **Sacrifice / Monkeyopolis absorb** — meta crafting, not combat modules.
7. **Dual-weapon / Archmage / Buccaneer / Ace** — **multiple pipelines per Base** (2+ independent compositions).
8. **Mobile platforms** (Ace orbit, Heli hover, Chinook relocate) — Base/placement system, not Aim/Path.
9. **Reticle / cursor aim** — player-input Aim modes; fine as Aim modules but need UX commitment.
10. **Damage-type immunities** (Sharp vs Lead vs Black vs Frozen) — combat **tag matrix**, not a pipeline stage.
11. **Beast Handler / summons / Carrier planes / Sentries** — minion system.
12. **Ally-targeted buffs** (Alchemist brew) — Aim=ally + Projectile=brew, but buff application ≠ enemy On-Hit.
13. **Leak-triggered passives** (Bomb Blitz, Legend black-hole) — tower passive scripts.
14. **`path_split` vs On-Hit frag** — pick one owner before API lock.

---

## 6) Recommended v1 catalog (implement first)

Tight set that still rebuilds Dart / Tack / Bomb / Ice-pulse / Sniper / Chain-spark / Glue-slow / Shock-bolt contrast.

### v1 modules (ship)

**Base:** `base_short`, `base_mid`, `base_long`, `base_rapid`  
**Aim:** `aim_single`, `aim_spray`, `aim_omni`, `aim_area`  
**Path:** `path_straight`, `path_lob`, `path_return`, `path_seek`  
**Projectile:** `proj_dart`, `proj_tack`, `proj_bomb`, `proj_glue`, `proj_ice`, `proj_ember`, `proj_spark`, `proj_bolt`  
**Delivery:** `del_travel`, `del_hitscan`, `del_explode`, `del_pulse`, `del_chain`, `del_beam`  
**On-Hit:** `hit_pierce`, `hit_splash`, `hit_burn`, `hit_slow`, `hit_freeze`, `hit_stun`, `hit_electrify`, `hit_knockback`

**Counts:** 4 + 4 + 4 + 8 + 6 + 8 = **34 modules**

### v1 example loadouts (sanity)

| Name | Assembly |
|------|----------|
| Fence Dart | short + single + straight + dart + travel + pierce |
| Tack Ring | short + omni + straight + tack + travel |
| Boom Shot | mid + single + straight + bomb + explode + splash |
| Frost Pulse | short + area + (n/a) + ice + pulse + freeze |
| Rail Sniper | long + single + (n/a) + bolt + hitscan |
| Storm Lattice | mid + single/omni + (n/a) + spark + chain + electrify |
| Shock Bolt | mid + single + straight + spark + travel + electrify |
| Glue Gun | mid + single + straight + glue + travel + slow |
| Lob Ember | mid + single/area + lob + ember + explode/travel + burn |
| Boomer | mid + single + return + dart/blade + travel + pierce |

### Immediately after v1 (stretch pack A)

`path_ricochet`, `proj_shuriken`, `proj_blade`, `proj_missile`, `del_ring`, `del_zone_track`, `hit_frag`, `hit_decamo`, `hit_brittle`  
Then decide Passive/Aura + Ability systems before chasing Temple/Farm/Spike Factory.

---

## Lock notes / deferred

**Locked:** the v1 34-module set in §6 (and matching `[S]` rows above). Item 1 and water/mobile out of v1 are settled by this lock.

**Still deferred (for API contract / later):**
1. `del_pulse` vs encoding pulse as `aim_area` + something else — implementation detail.
2. Owner of split/frag (`path_split` vs On-Hit) — stretch; not in v1 set.
3. Passive/Aura + Ability system schedule — post-v1.

— Game Design
