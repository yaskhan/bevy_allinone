# Weapon System - Comprehensive Documentation

## Table of Contents

1. [Overview](#overview)
2. [Core Components](#core-components)
   - [Weapon](#weapon)
   - [WeaponManager](#weaponmanager)
   - [Inventory Pockets](#inventory-pockets)
3. [Weapon Attributes](#weapon-attributes)
   - [Fire Stats](#fire-stats)
   - [Handling & Recoil](#handling--recoil)
   - [Visuals & Audio](#visuals--audio)
4. [Mechanics](#mechanics)
   - [Firing Modes](#firing-modes)
   - [Ballistics vs Hitscan](#ballistics-vs-hitscan)
   - [Accuracy & Bloom](#accuracy--bloom)
   - [Reloading](#reloading)
5. [Attachment System](#attachment-system)
   - [Overview](#attachment-overview)
   - [Stat Modifiers](#stat-modifiers)
   - [Integration](#integration)
6. [Visual Effects](#visual-effects)
7. [Weapon Builder API](#weapon-builder-api)
8. [Specialty Weapons](#specialty-weapons)
9. [Setup and Usage](#setup-and-usage)

## Overview

The Weapon System is a highly modular and data-driven framework for creating FPS/TPS combat mechanics in Bevy. It supports everything from simple hitscan pistols to complex projectile-based launchers, featuring a robust attachment system, procedural recoil, realistic ballistics, and a flexible inventory system organized by "pockets".

## Core Components

### Weapon

The `Weapon` component is the heart of the system, defining all stats and behaviors for a specific gun.

```rust
#[derive(Component, Debug, Reflect)]
pub struct Weapon {
    pub weapon_name: String,
    pub weapon_type: WeaponType, // Pistol, Rifle, Shotgun, etc.
    
    // Core Stats
    pub damage: f32,
    pub range: f32,
    pub fire_rate: f32,      // Shots per second
    pub ammo_capacity: i32,
    
    // Mechanics
    pub firing_mode: FiringMode, // SemiAuto, FullAuto, Burst
    pub projectile_speed: f32,   // 0.0 = Hitscan
    pub spread: f32,
    
    // ... extensive configuration fields
}
```

### WeaponManager

The `WeaponManager` handles the player's inventory, weapon switching, and input processing. It acts as the central hub for the player's arsenal.

```rust
#[derive(Component, Debug, Reflect)]
pub struct WeaponManager {
    pub weapons_list: Vec<Entity>,
    pub current_index: usize,
    
    // Dual Wielding
    pub using_dual_weapon: bool,
    pub dual_weapons_enabled: bool,
    
    // State Flags
    pub reloading_with_animation: bool,
    pub aiming_in_first_person: bool,
    
    // Inventory Organization
    pub weapon_pockets: Vec<WeaponPocket>,
}
```

### Inventory Pockets

Weapons are organized into "Pockets" (e.g., Primary, Secondary, Melee, Grenades). This allows for structured inventory management rather than a flat list.

- **Primary**: Main rifles, shotguns.
- **Secondary**: Pistols, sidearms.
- **Melee**: Knives, tools.
- **Special**: Rocket launchers, experimental weapons.
- **Grenade**: Throwable items.

## Weapon Attributes

### Fire Stats

- **`damage`**: Base damage per hit.
- **`fire_rate`**: Delay between shots (calculated as `1.0 / fire_rate`).
- **`range`**: Maximum effective distance.
- **`projectiles_per_shot`**: Number of bullets per trigger pull (e.g., Shotguns use > 1).

### Handling & Recoil

Recoil is procedural and affects the camera/aim.

```rust
pub struct RecoilSettings {
    pub kick_back: f32,        // Z-axis visual kick
    pub vertical_recoil: f32,  // Upward aim rise
    pub horizontal_recoil: f32,// Side-to-side aim jitter
    pub recovery_speed: f32,   // How fast aim recenters
    pub ads_multiplier: f32,   // Recoil reduction when aiming
}
```

### Visuals & Audio

- **Muzzle Flash**: Spawns a visual effect entity at the barrel.
- **Shell Ejection**: Spawns physics-based casings.
- **Animations**: Specifies state-based animation names (`idle_anim`, `shoot_anim`, `reload_anim`, etc.).

## Mechanics

### Firing Modes

1.  **Semi-Auto**: One shot per click. Requires releasing the trigger to fire again.
2.  **Full-Auto**: Fires continuously while the trigger is held.
3.  **Burst**: Fires a fixed number of shots (`burst_settings.amount`) with a specific cycle rate per trigger pull.

### Ballistics vs Hitscan

The system supports both hitscan (instant) and physical projectiles seamlessly.

-   **Hitscan**: Set `projectile_speed` to `0.0`. Uses raycasting for instant hits. Best for standard bullets in fast-paced games.
-   **Projectiles**: Set `projectile_speed` > `0.0`. Spawns a physical `Projectile` entity with velocity, gravity (`use_gravity`), and drag (`projectile_drag_coeff`). Best for rocket launchers, arrows, or "realistic" shooters.

### Accuracy & Bloom

Accuracy is dynamic. Firing increases "Bloom", which adds random deviation to shots.

-   **Base Spread**: Inherent inaccuracy of the gun.
-   **Bloom per Shot**: How much the spread increases per shot.
-   **Recovery Rate**: How quickly accuracy returns to normal after firing stops.
-   **Modifiers**: Movement penalty, airborne multiplier, and ADS modifier (usually reduces spread).

### Reloading

Handles both "reload with ammo" (tactical reload) and "empty reload" (dry).
-   Checks `current_ammo < ammo_capacity`.
-   Triggers animation state `ReloadWithAmmo` or `ReloadWithoutAmmo`.
-   Replenishes ammo after `reload_time` elapses.

## Attachment System

### Attachment Overview

Weapons can have multiple **Attachment Places** (e.g., Scope, Muzzle, Magazine, Underbarrel). Each place can hold one active attachment from a list of available options.

### Stat Modifiers

Attachments use `AttachmentStatModifiers` to dynamically alter weapon performance:

-   **Silencer**: Reduces noise radius, slightly reduces damage.
-   **Extended Mag**: Increases `ammo_capacity`, slightly increases reload time.
-   **Scope**: Reduces spread, changes FOV/ADS speed.
-   **Heavy Barrel**: Increases damage and recoil.
-   **Laser Sight**: Improves hip-fire accuracy.

### Integration

To edit attachments, the `WeaponManager` can toggle an "Attachment Editor" mode (`open_attachment_editor`). This allows UI systems to display available payloads and modify the weapon in real-time.

## Visual Effects

-   **Tracers**: Visual lines that follow hitscan paths or projectiles.
-   **Muzzle Flash**: Flickering light/mesh at the gun barrel.
-   **Shell Ejection**: Physics objects ejected from the receiver.
-   **Laser Sight**: Raycasted beam that draws a dot on the target surface.
-   **Procedural Sway**: Weapon models sway with movement (`SwaySettings`) and bob while walking (`WeaponIkSettings`).

## Weapon Builder API

The `WeaponBuilder` provides a fluent API for constructing complex weapons in code.

```rust
let rifle = WeaponBuilder::new("Assault Rifle")
    .with_fire_rate(600.0) // RPM
    .with_firing_mode(FiringMode::FullAuto)
    .with_ammo(30)
    .with_recoil(0.5, 0.2, 5.0) // Vert, Horz, Recovery
    .with_accuracy(0.1, 1.5, 0.1) // Base, Max, Bloom
    .with_visuals(true, true) // Muzzle flash, Shells
    .with_attachment_slot("scope", "Optic")
    .with_attachment_slot("muzzle", "Barrel")
    .spawn(&mut commands);
```

## Specialty Weapons

The system includes built-in support for unique weapon behaviors:

-   **Bow**: Charge-up mechanic (`BowSettings`), arrow physics.
-   **Gravity Gun**: Physics manipulation (`GravityGunSettings`), pick up/throw objects.
-   **Beam Weapons**: Continuous damage ray (`BeamSettings`).
-   **Homing Projectiles**: Missiles that track targets (`Homing` component).

## Setup and Usage

### Basic Setup

1.  Ensure `WeaponManager` is added to your player entity.
2.  Initialize Pockets (`create_default_pockets`).
3.  Spawn weapons using `WeaponBuilder`.
4.  Add weapons to the manager's list or pockets.

```rust
// In your player spawn logic
commands.entity(player_entity).insert(WeaponManager::default());

// Spawning a weapon
let pistol = WeaponBuilder::new("M9 Pistol")
    .with_firing_mode(FiringMode::SemiAuto)
    .with_ammo(15)
    .spawn(&mut commands);

// Adding to inventory
if let Ok(mut manager) = weapon_manager_query.get_mut(player_entity) {
    manager.weapons_list.push(pistol);
    manager.add_weapon_to_pocket("M9 Pistol", "secondary");
}
```
