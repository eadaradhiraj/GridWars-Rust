# GridWars (Pure Rust Edition)

A 100% pure **Rust** port of the classic arcade twin-stick shooter *GridWars*, originally created by Mark Incitti in 2006.

This port completely replaces the legacy BlitzMax codebase with a modern, high-performance, cross-platform Rust architecture powered by [Macroquad](https://macroquad.rs).

## Features

- **Pure Rust Architecture:** Zero BlitzMax or GCC legacy C runtime dependencies.
- **Interactive 2D Elastic Grid:** Real-time spring physics simulation with Laplacian 4-neighbor relaxation, wave propagation, bomb shockwaves, and gravitational distortions.
- **Authentic Vector Font:** High-precision procedural line-segment typography faithful to the original 1980s vector arcade aesthetic.
- **Dynamic Particle Engine & Starfield:** High-speed particle explosions, exhaust trails, bullet sparks, and twinkling parallax starfield.
- **Full Enemy Ecosystem:**
  - **Pinwheel:** Spinning kinetic hazards that bounce across grid boundaries.
  - **Seeker Diamond:** Aggressive tracking enemies that home in on the player.
  - **Wanderer Square:** Linear diagonal bouncers.
  - **Black Hole / Sun:** Gravitational singularities that warp the grid, suck in nearby projectiles and fighters, and detonate upon destruction.
  - **Clone / Grunt:** Marching swarm units.
  - **Slithering Snake:** Segmented multi-node serpents.
  - **Shielded Triangle:** Front-armored hostiles vulnerable only from flanking angles.
  - **Butterfly / Weaver:** High-speed evasive weavers that dodge incoming fire.
  - **Splitter Cube:** Fractures into smaller cubes when destroyed.
  - **Generator:** Defensive spawners that periodically deploy enemy waves.
- **Weapon System & Powerups:**
  - Forward cannon, 3-Way spread shot, Side cannons, Rear shot, Super penetrating laser, and Bouncing munitions.
  - Screen-clearing Smart Bombs.
  - Energy Shields and Extra Lives.
- **High Scores & Attract Loop:**
  - Authentic multi-screen attract loop: Title Screen, Top 10 High Scores, Friends & Powerups overview, and Hostiles showcase.
  - In-game Options Menu (Esc / Tab) supporting multiple control schemes.

---

## Controls

| Action | Primary Controls | Twin-Stick Mode | Mouse Mode |
|---|---|---|---|
| **Move Ship** | <kbd>Arrow Keys</kbd> or <kbd>WASD</kbd> | <kbd>WASD</kbd> | <kbd>WASD</kbd> / Cursor |
| **Shoot** | <kbd>Space</kbd> | <kbd>Arrow Keys</kbd> (360°) | <kbd>Left Click</kbd> |
| **Smart Bomb** | <kbd>B</kbd> / <kbd>Shift</kbd> | <kbd>B</kbd> / <kbd>Shift</kbd> | <kbd>Right Click</kbd> |
| **Options / Pause** | <kbd>ESC</kbd> / <kbd>Tab</kbd> | <kbd>ESC</kbd> / <kbd>Tab</kbd> | <kbd>ESC</kbd> |

---

## Building and Running

Ensure you have Rust and Cargo installed:

```bash
cargo run --release
```

To build a standalone optimized binary:

```bash
cargo build --release
```
The compiled executable will be located at `target/release/gridwars`.
