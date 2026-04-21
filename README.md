# REQUIEM, Project Documentation

> A stark, minimalist bullet heaven built on the Journey Engine.  
> Fewer, faster, deadlier enemies. Monochrome geometry. Heavy impact. Zero assets.

---

## Status: **Complete** (see [Development History](#development-history) for details)

| Command                                | Result                          |
| -------------------------------------- | ------------------------------- |
| cargo build                            | 0 errors                        |
| cargo run                              | Window opens, game runs         |
| wasm-pack build --target web --release | ~4.2 MB pkg @ --out-dir web/pkg |
| python -m http.server 8080 (in web/)   | WASM runs in browser            |

---

## Table of Contents

1. [Overview](#overview)
2. [Architecture](#architecture)
3. [Engine Contract](#engine-contract)
4. [Module Reference](#module-reference)
5. [Upgrade System](#upgrade-system)
6. [Audio System](#audio-system)
7. [Rendering Pipeline](#rendering-pipeline)
8. [UI/HUD](#uihud)
9. [Build & Run](#build--run)
10. [Development History](#development-history)

---

## Overview

**Requiem** is a Vampire Survivors-like (bullet heaven) game where the player survives infinite waves of enemies in a 3000×3000 arena. All attacks are automatic, the player only controls movement. On level-up, the game pauses and presents 3 random upgrade choices from a pool of 15 upgrades across 4 categories.

### Design Pillars

| Pillar          | Description                                                                                                                 |
| --------------- | --------------------------------------------------------------------------------------------------------------------------- |
| **Aesthetic**   | Terminal-inspired monochrome. No textures, no sprites, just white pixel rects and additive glow                             |
| **Constraint**  | Engine hard-caps at 1024 sprites/frame. Design favors fewer, deadlier enemies over swarms                                   |
| **Feel**        | Screen shake, hitstop freeze, damage flash. Every kill and hit has tactile feedback                                         |
| **Zero Assets** | No image files, no audio files. Everything is code-generated, geometry from `draw_rect()`, sound from synthesized WAV bytes |

### Key Stats

| Metric              | Value                                                   |
| ------------------- | ------------------------------------------------------- |
| Lines of Rust       | ~1,800 across 15 modules                                |
| Dependencies        | `journey-engine` v1.1.2 only (+`wasm-bindgen` for WASM) |
| WASM Size (release) | ~4.2 MB                                                 |
| Asset Files         | 0                                                       |
| Textures            | 0 (uses built-in white pixel, `texture_id = 0`)         |
| Audio Files         | 5 sounds synthesized from sine/square/noise waveforms   |

---

## Architecture

```bash
requiem/
├── .gitignore
├── .vscode/                 # Optional editor config
├── Cargo.toml               # Standalone crate, engine from crates.io
├── LICENSE
├── README.md                # This file
├── docs/
│   └── requiem.rs           # Auto generated for all .rs files, for easy distribution without source access
├── src/
│   ├── main.rs              # Native entry: requiem::run_game()
│   ├── lib.rs               # Module declarations + WASM entry
│   ├── action.rs            # Input: RequiemAction enum + key bindings
│   ├── audio.rs             # Procedural sound synthesis + SFX dispatch
│   ├── util.rs              # Constants, colors, LCG random, helpers
│   ├── player.rs            # Player struct + PlayerUpgrades
│   ├── enemy.rs             # Enemy struct + EnemyKind (Drone/Brute/Phantom)
│   ├── projectile.rs        # Projectile with pierce support
│   ├── xp.rs                # XP gems with magnetic attraction
│   ├── spawner.rs           # Wave spawner with scaling difficulty
│   ├── upgrades.rs          # 15 upgrades, 4 categories, selection logic
│   ├── render.rs            # All draw calls with sub-tick interpolation
│   ├── hud.rs               # egui: HUD bars, level-up cards, game over
│   └── game.rs              # RequiemGame: GameApp impl, full game loop
└── web/
    ├── index.html           # Static WASM loader (no npm/Vite)
    └── pkg/                 # wasm-pack output (gitignored)
```

### Standalone Pattern

Follows a non-dep standalone architecture:

- **No workspace**, no sub-crates, no path dependencies
- Engine from crates.io: `engine = { package = "journey-engine", version = "1.1.2" }`
- Web harness is a single `index.html`, no build tools needed
- WASM entry via `#[wasm_bindgen(start)]` inside `lib.rs`
- Native entry via `pub fn run_game()` → `engine::run::<RequiemGame>()`

---

## Engine Contract

Requiem implements the `GameApp` trait from Journey Engine:

| Method           | Frequency   | Purpose                                                                                                           |
| ---------------- | ----------- | ----------------------------------------------------------------------------------------------------------------- |
| `init()`         | Once        | Bind inputs, create player, generate audio assets                                                                 |
| `fixed_update()` | 60 Hz fixed | All physics, combat, spawning, collision, XP, damage. Visual effects (damage flash) still decay even when paused. |
| `update()`       | Every frame | Camera interpolation, SFX dispatch, restart check                                                                 |
| `render()`       | Every frame | All draw calls (grid, entities, VFX) with position interpolation                                                  |
| `ui()`           | Every frame | egui HUD, level-up screen, game over screen, scene params                                                         |

### Key Engine APIs Used

| API                                    | Usage                                                                   |
| -------------------------------------- | ----------------------------------------------------------------------- |
| `draw_rect(pos, size, color)`          | All solid geometry, player, enemies, projectiles, grid, bars            |
| `draw_sprite_from_sheet_additive(...)` | Glow effects, player aura, projectile trails, death pulse               |
| `AABB::new(center, size)`              | All collision detection (projectile→enemy, enemy→player, orbital→enemy) |
| `move_towards(current, target, delta)` | Enemy AI, chase player per-axis                                         |
| `trigger_shake(intensity, duration)`   | Kill feedback, hit impact, level-up                                     |
| `trigger_freeze(frames)`               | Hitstop on player damage, level-up pause                                |
| `camera_offset_x/y`                    | Smooth camera follow with interpolation                                 |
| `input.get_move_x/y()`                 | Normalized directional input from bound actions                         |
| `load_sound_data(bytes)`               | Load procedurally generated WAV from leaked byte slices                 |
| `audio.play_oneshot(data, track)`      | Fire-and-forget SFX playback                                            |
| `SceneParams`                          | Background color, fog density/color/speed                               |

### Position System

All entities store positions as world-space `Vec2` (center of the entity):

- **AABB collision**: Uses center directly, `AABB::new(center, size)`
- **Drawing**: Converts center → top-left via `draw_pos(center, size)` helper
- **Camera**: World-space positions are offset by the engine's camera system

---

## Module Reference

### `action.rs`, Input System

4-action enum: `MoveLeft`, `MoveRight`, `MoveUp`, `MoveDown`. Attacks are automatic.

Bindings: WASD + Arrow keys + Gamepad D-pad (native only).

The `move_negative_x/y` and `move_positive_x/y` trait methods enable the engine's `get_move_x()` / `get_move_y()` helpers, which return -1.0 to 1.0 for smooth analog-style input.

### `util.rs`, Constants & Helpers

All game tuning constants live here:

- Arena: 3000×3000 at 640×360 internal resolution
- Player: 12px, 110 px/s, 5 HP, 0.75s attack cooldown
- Enemies: Drone (8px/65spd/1hp), Brute (18px/30spd/6hp), Phantom (10px/85spd/2hp)
- Projectiles: 280 px/s, 1.8s lifetime, 4px
- Full RGBA color palette (monochrome + accent colors)
- LCG pseudo-random: `lcg(seed)`, `lcg_range(seed, lo, hi)`, no `rand` crate dependency

### `player.rs`, Player + Upgrades

`Player` struct with upgrade-aware computed stats:

- `effective_speed()`, base × (1 + swiftness × 0.15)
- `effective_cooldown()`, base × 0.8^rapid_fire
- `effective_damage()`, base × (1 + heavy_caliber × 0.5)
- `effective_proj_speed()`, base × (1 + acceleration × 0.25)
- `effective_pickup_radius()`, base × (1 + magnetism × 0.4)

`PlayerUpgrades` tracks the level of all 15 upgrades as u32 fields.

### `enemy.rs`, Enemies

Three kinds with distinct personalities:

| Kind    | Size | Speed | HP  | XP  | Color                  | Role                 |
| ------- | ---- | ----- | --- | --- | ---------------------- | -------------------- |
| Drone   | 8px  | 65    | 1   | 1   | Dim gray               | Swarm fodder         |
| Brute   | 18px | 30    | 6   | 5   | Bright white           | Tank, has health bar |
| Phantom | 10px | 85    | 2   | 2   | Blue-gray, translucent | Fast harasser        |

Hit flash: 6-tick white overlay on taking damage.

### `spawner.rs`, Wave System

Waves spawn every 5s (decreasing to 1.8s minimum). Each wave brings 3–12 enemies based on wave number, capped at 60 total enemies. Enemy type distribution shifts toward Phantoms and Brutes in later waves.

Enemies spawn at random positions just outside the camera viewport.

### `game.rs`, Game Loop

The central `RequiemGame` struct implements `GameApp`. Three phases: `Playing`, `LevelUp`, `GameOver`.

**fixed_update tick order:**

1. Snapshot all prev_pos for interpolation
2. Player movement (normalized diagonal)
3. Arena clamping, timer ticks, barrier updates
4. Auto-fire projectiles toward nearest enemy
5. Update projectile positions, remove expired
6. Spawn enemy waves
7. Enemy AI: `move_towards` player per-axis
8. Projectile→Enemy collision (AABB, pierce support)
9. Orbital blade contact damage
10. Death pulse aura damage
11. Process kills: shake, SFX, XP gem spawn, siphon heal, detonation chain
12. Remove dead enemies
13. Enemy→Player collision (damage, i-frames, freeze, shake)
14. XP gem collection (magnetic attraction → pickup → level check)
15. Level-up transition (pause, roll upgrades)
16. Death check
17. Camera follow (lerp toward player center)

---

## Upgrade System

15 upgrades across 4 categories, selected 3-at-a-time on level-up:

### Offense (Red)

| Upgrade         | Effect per Level                      | Max |
| --------------- | ------------------------------------- | --- |
| Spreadshot      | +2 projectiles in fan pattern         | 5   |
| Piercing Rounds | Projectiles pierce +1 enemy           | 5   |
| Rapid Fire      | Attack cooldown -20% (multiplicative) | 5   |
| Heavy Caliber   | Projectile damage +50%                | 5   |
| Volley          | +2 projectiles at random enemies      | 5   |

### Defense (Blue)

| Upgrade      | Effect per Level                              | Max |
| ------------ | --------------------------------------------- | --- |
| Void Armor   | +1 max HP, heal to full                       | 5   |
| Chronoshield | Invincibility duration +50%                   | 5   |
| Siphon       | Heal 1 HP every N kills (threshold decreases) | 5   |
| Barrier      | Auto-shield absorbs 1 hit / 25s               | 1   |

### Utility (Yellow)

| Upgrade      | Effect per Level      | Max |
| ------------ | --------------------- | --- |
| Swiftness    | Movement speed +15%   | 5   |
| Magnetism    | XP pickup radius +40% | 5   |
| Acceleration | Projectile speed +25% | 5   |

### Exotic (Purple)

| Upgrade       | Effect per Level                           | Max |
| ------------- | ------------------------------------------ | --- |
| Death Pulse   | Aura deals 1 dmg/1.5s to nearby enemies    | 5   |
| Orbital Blade | +2 orbiting blades dealing contact damage  | 3   |
| Detonation    | Killed enemies explode, dealing AoE damage | 5   |

Upgrades are randomly rolled from the pool of those not yet maxed. Each selection screen shows exactly 3 choices (or fewer if the pool is exhausted).

---

## Audio System

All 5 sound effects are **procedurally generated** from basic waveforms, no audio files exist:

| Sound     | Trigger              | Waveform                                  | Duration |
| --------- | -------------------- | ----------------------------------------- | -------- |
| Shoot     | Projectile fired     | Square wave 880Hz + noise transient       | 60ms     |
| Kill      | Enemy killed         | Descending sine (500→120Hz) + noise burst | 100ms    |
| Hit       | Player takes damage  | Low sine 65Hz + impact noise              | 180ms    |
| Level Up  | XP threshold reached | C-E-G major arpeggio (3 sine tones)       | 350ms    |
| XP Pickup | Gem collected        | High sine 1600Hz, fast decay              | 35ms     |

### How It Works

1. **Synthesis**: Raw PCM samples generated from math (sine, square, noise with envelope shaping)
2. **Encoding**: Samples encoded as 16-bit mono WAV at 44100Hz in-memory
3. **Static leak**: WAV bytes leaked via `Box::leak()` to get `&'static [u8]`
4. **Loading**: Passed to `engine::load_sound_data()` → Kira `StaticSoundData`

### Event Queue Pattern

Sound events are generated in `fixed_update()` (physics tick) but played in `update()` (render frame), avoiding borrow conflicts:

```bash
fixed_update → push SfxEvent to queue
update → drain queue, dispatch via audio.play_oneshot()
```

---

## Rendering Pipeline

All rendering uses `texture_id = 0` (engine's built-in 1×1 white pixel). No textures are loaded.

### Draw Order (back to front)

1. Background grid (subtle lines every 60px)
2. Arena border (2px bright edges)
3. XP gems (green squares + additive glow)
4. Enemies (colored rects, health bars on Brutes)
5. Projectiles (white squares + additive glow)
6. Death pulse aura (additive red circle)
7. Orbital blades (bright squares orbiting player)
8. Player (white square + glow, blinks during i-frames, barrier outline)
9. Damage flash overlay (full-screen red fade)

### Sub-Tick Interpolation

All entities store `prev_pos` alongside `pos`. At the start of each `fixed_update`, prev positions are snapshotted. During `render()`, positions are interpolated:

```bash
render_pos = prev_pos + (pos - prev_pos) × interpolation_alpha
```

This eliminates visual jitter at any display refresh rate (60Hz, 120Hz, 144Hz+).

### Camera

- Smooth lerp follow: `camera += (target - camera) × 0.12` per tick
- Interpolated between ticks using `interpolation_alpha`
- Clamped to arena bounds
- **Paused states**: Snapped to current position (no interpolation wobble)

---

## UI/HUD

Built entirely with egui (re-exported from the engine). Consistent 2px corner radius throughout.

Uses **`egui::Area`-based overlays** (not `CentralPanel`) to avoid layout conflicts with the HUD. Both the level-up and game-over screens paint a dark rect on the painter as a background, then layer content via a separate `Area` with `Order::Foreground`.

### Playing State

- **Top-left**: HP bar (visual red fill) with numeric + XP bar (green fill) with level
- **Top-right**: Timer (MM:SS), kill count, wave number
- **Bottom-right**: FPS counter (vsync-locked, typically 58–60)

### Level-Up State

- Dark overlay (85% opacity, Area-based)
- "LEVEL UP" title + current level, centered
- 3 upgrade cards in horizontal row with category-colored accent bar, name, description, level badge
- Click-to-select with hover highlight border

### Game Over State

- Very dark overlay (90% opacity)  
- "REQUIEM" title in red, vertically + horizontally centered
- "you have fallen" subtitle
- Stats panel with constrained width (time, kills, level, wave)
- Gold `[ SPACE ]` restart prompt
- Damage flash decays during GameOver so the red screen clears naturally

---

## Build & Run

### Prerequisites

- Rust toolchain (edition 2024)
- `wasm-pack` (for WASM builds)
- Python 3 or any static file server (for local WASM testing)

### Native

```bash
cargo run
```

### WASM (Development)

```bash
wasm-pack build --target web --out-dir web/pkg --dev
cd web
python -m http.server 8080
# Open http://localhost:8080
```

### WASM (Release, ~4.2 MB)

```bash
wasm-pack build --target web --out-dir web/pkg --release
cd web
python -m http.server 8080
```

---

## Development History

1. **Engine API Review**: Analyzed `ENGINE_API.md` to understand the `GameApp` lifecycle, `Context` handle, `FixedTime` loop, `egui` integration, sprite limits, and rendering APIs.

2. **Pattern Analysis**: Studied the `dino-blink` project as a reference implementation. Confirmed it's a standalone crate using `journey-engine` from crates.io (not a workspace).

3. **Architecture Decision**: Chose standalone crate pattern over workspace. The game pulls `journey-engine` as a regular dependency, no path deps, no sub-crates.

4. **Scaffolding**: Created project structure, `Cargo.toml`, entry points (`main.rs`, `lib.rs`), module declarations, web harness (`web/index.html`).

5. **Core Systems**: Implemented all 14 modules:
   - Input system (4-directional `RequiemAction`)
   - Player with upgrade-aware stats
   - 3 enemy types (Drone, Brute, Phantom)
   - Projectile system with piercing
   - XP gem attraction + collection
   - Wave spawner with difficulty scaling
   - 15 upgrades across 4 categories
   - Full rendering pipeline (grid, entities, VFX, glow)
   - egui HUD (health, XP, stats, FPS, level-up cards, game over)
   - Core game loop with full `GameApp` implementation

6. **Bug Fixes**:
   - **Camera jitter on pause**: Fixed by snapping camera position (no interpolation) during `LevelUp` and `GameOver` phases
   - **Entity jitter during gameplay**: Fixed by adding `prev_pos` to all entities and interpolating positions in render using `interpolation_alpha`
   - **WASM path issue**: Fixed by outputting `wasm-pack` to `web/pkg/` so the static server can resolve relative imports
   - **Damage flash persisting on death**: Fixed by decaying `damage_flash` even during non-Playing phases (was stuck because `fixed_update` returned early)
   - **Level-up UI showing only 1 card**: Root cause was `CentralPanel` conflicting with egui's layout system. Switched all overlays to `Area`-based rendering

7. **Audio**: Added procedural sound synthesis, 5 sound effects generated from waveforms (sine, square, noise) encoded as WAV in memory. No audio files.

8. **UI Polish**: Rewrote `hud.rs` twice. First pass used `CentralPanel` overlays which broke card layout and stretched panels. Second pass uses `Area`-based overlays with `Order::Foreground`, proper card sizing, visual HP/XP bars, and constrained stat panels.

### Key Design Decisions

| Decision                     | Rationale                                                                |
| ---------------------------- | ------------------------------------------------------------------------ |
| Modular files (14 .rs)       | User requested clean architecture over dino-blink's single-file approach |
| LCG random (no `rand`)       | Minimal dependencies, matches dino-blink pattern                         |
| Procedural audio             | Zero asset files, matches terminal aesthetic                             |
| Event queue for SFX          | Avoids borrow conflicts between fixed_update and audio playback          |
| 640×360 internal res         | Larger than dino-blink's 360×180, room for a bullet heaven arena         |
| 3000×3000 arena              | Large scrolling world with camera follow, not single-screen              |
| Upgrade selection (not auto) | User requirement, interactive 3-choice cards per level                   |
| Sub-tick interpolation       | Smooth rendering at any refresh rate, not locked to 60fps                |

---

## Sprite Budget

Engine hard limit: **1024 sprites per frame**.

| Element                 | Max Count | Sprites Each | Total    |
| ----------------------- | --------- | ------------ | -------- |
| Grid lines              | ~30       | 1            | 30       |
| Arena border            | 4         | 1            | 4        |
| Player + glow + barrier | 1         | 6            | 6        |
| Enemies + HP bars       | 60        | 2            | 120      |
| Projectiles + glow      | 120       | 2            | 240      |
| XP Gems + glow          | 250       | 2            | 500      |
| Orbital blades          | 6         | 2            | 12       |
| Death pulse             | 1         | 1            | 1        |
| Damage flash            | 1         | 1            | 1        |
| **Worst case total**    |           |              | **~914** |

Comfortably within the 1024 budget.
