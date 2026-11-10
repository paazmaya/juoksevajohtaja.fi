# Juokseva Johtaja - Bevy Reference Implementation

This directory provides a reference architecture for running **Juokseva Johtaja** directly on the **[Bevy engine](https://bevyengine.org/)** using the same assets and JSON level configurations as the WebAssembly production game.

---

## Architecture Overview

```text
bevy_reference/
├── Cargo.toml.example      # Project dependencies (Bevy 0.14, bevy_parallax, bevy_rapier2d)
└── src/
    ├── main.rs             # Application entry point, window setup & Bevy plugins
    ├── level.rs            # Serde loader for episode_1_undisclosed.json
    ├── parallax.rs         # 3-layer parallax camera and background system (bevy_parallax)
    ├── obstacles.rs        # Independent obstacle entity spawning & Rapier colliders
    └── character.rs        # TextureAtlasLayout sprite sheet animations & runner physics
```

---

## 1. Running the Bevy Project

1. Copy `Cargo.toml.example` to `Cargo.toml`:
   ```bash
   cp Cargo.toml.example Cargo.toml
   ```
2. Symlink or copy the game assets into an `assets/` directory:
   ```bash
   ln -s ../assets assets
   ```
3. Run with Cargo:
   ```bash
   cargo run --release
   ```

---

## 2. Asset Mapping in Bevy

| Game Element | Asset Path | Bevy Component / Crate |
|---|---|---|
| **Layer 1: Sky** | `assets/backgrounds/helsinki/layer1_sky.png` | `ParallaxLayer { z: -30.0, speed: 0.05 }` via `bevy_parallax` |
| **Layer 2: Midground** | `assets/backgrounds/helsinki/layer2_midground.png` | `ParallaxLayer { z: -20.0, speed: 0.25 }` via `bevy_parallax` |
| **Layer 3: Foreground** | `assets/backgrounds/helsinki/layer3_foreground.png` | `ParallaxLayer { z: -10.0, speed: 0.65 }` via `bevy_parallax` |
| **Obstacles** | `assets/obstacles/obstacle_*.png` | `SpriteBundle` + `Collider::cuboid` via `bevy_rapier2d` |
| **Guest Runner** | `assets/characters/adventurer/spritesheet.png` | `TextureAtlasLayout` (96x96 px cells, 5 frames) |
| **Host Runner** | `assets/characters/player/spritesheet.png` | `TextureAtlasLayout` (96x96 px cells, 5 frames) |
