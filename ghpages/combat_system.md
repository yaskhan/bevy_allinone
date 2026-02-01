# Combat System - Comprehensive Documentation

## Table of Contents

1. [Overview](#overview)
2. [Core Components](#core-components)
   - [Health & Shield](#health--shield)
   - [Damage Receiver](#damage-receiver)
3. [Combat Mechanics](#combat-mechanics)
   - [Damage System](#damage-system)
   - [Melee Combat](#melee-combat)
   - [Blocking & Parrying](#blocking--parrying)
   - [Destroyable Objects](#destroyable-objects)
4. [Advanced Effects](#advanced-effects)
   - [Damage Over Time (DoT)](#damage-over-time-dot)
   - [Area Effects (AoE)](#area-effects-aoe)
5. [User Interface & Feedback](#user-interface--feedback)
6. [Setup and Usage](#setup-and-usage)

## Overview

The Combat System is a robust framework designed to handle damage processing, health management, and combat interactions for Bevy. It supports complex damage calculation chains, including shielding, resistance, weak spots, and floating combat text feedback.

## Core Components

### Health & Shield

The foundation of any combat entity.

#### Health
Manages life points, regeneration, and death.

```rust
#[derive(Component)]
pub struct Health {
    pub current: f32,
    pub maximum: f32,
    pub can_regenerate: bool,
    pub regeneration_rate: f32, // HP per second
    pub regeneration_delay: f32,// Seconds after damage to start regen
    pub is_invulnerable: bool,
    pub temporal_invincibility_duration: f32, // i-frames
}
```

#### Shield
A secondary health pool that absorbs damage before Health.

```rust
#[derive(Component)]
pub struct Shield {
    pub current: f32,
    pub maximum: f32,
    pub can_regenerate: bool,
    pub regeneration_rate: f32,
    pub is_active: bool, // Can be toggled off (e.g., EMP effect)
}
```

### Damage Receiver

Used for hitboxes or body parts (head, limbs) that forward damage to a main `Health` component. This allows for locational damage multipliers.

- **`damage_multiplier`**: Scale incoming damage (e.g., Headshot = 2.0x).
- **`is_weak_spot`**: Flags hits as "Critical" for UI feedback.
- **`health_root`**: The entity with the `Health` component to damage.

## Combat Mechanics

### Damage System

The system uses an event-based approach (`DamageEventQueue`) to process hits.

**Processing Pipeline:**
1.  **Resolve Target**: Determines if hit was on a `DamageReceiver` and finds the root `Health` entity.
2.  **Multipliers**: Applies `general_damage_multiplier` and part-specific multipliers.
3.  **Resistance**: Checks for Blocking/Parrying.
4.  **Shield Absorption**: Deducts damage from Shield first (unless `ignore_shield` is true).
5.  **Health Application**: Deducts remaining damage from Health.
6.  **Death**: Pushes `DeathEvent` if Health reaches 0.

### Melee Combat

Handles close-quarters attacks with sphere-casting and combos.

```rust
#[derive(Component)]
pub struct MeleeCombat {
    pub damage: f32,
    pub range: f32,
    pub attack_speed: f32,
    pub combo_enabled: bool,
    pub combo_window: f32, // Time to chain next attack
}
```

-   **Combo System**: Successful attacks within the `combo_window` increment `combo_count`, increasing damage.
-   **Hit Detection**: Casts a sphere forward to detect targets.

### Blocking & Parrying

Defense mechanism to reduce or negate damage.

-   **Block**: Reduces damage by `block_reduction` factor (e.g., 0.5 = 50% damage).
-   **Parry**: Negates **all** damage if blocked within the `parry_window` (just-frame block). Triggers special feedback.

### Destroyable Objects

Entities that can be destroyed and optionally explode.

-   **Debris**: Can spawn debris on death.
-   **Explosions**: Configured via `ExplosionSettings` (radius, damage, force).
    -   Applies damage to all entities in radius.
    -   Applies physical impulse/velocity to push objects away.

## Advanced Effects

### Damage Over Time (DoT)

 Applies damage tick-by-tick. Useful for poison, fire, or bleeding.

-   **`damage_per_tick`**: Amount of damage per interval.
-   **`tick_frequency`**: How often to apply damage (seconds).
-   **`total_duration`**: How long the effect lasts.

### Area Effects (AoE)

Creates zones that apply damage or healing to entities inside.

-   **`radius`**: Size of the zone.
-   **`interval`**: How often to apply the effect.
-   **`damage_type`**: Can be used for environmental hazards (Fire) or Healing zones (`DamageType::Heal`).

## User Interface & Feedback

The system includes built-in "Floating Damage Numbers" support.

-   **Damage Numbers**: Spawns text that floats up and fades out.
-   **Color Coding**:
    -   **White/Red**: Normal damage.
    -   **Yellow**: Critical Hit / Weak Spot / Parry.
    -   **Green**: Healing.
    -   **Cyan**: Shield Hit.
    -   **Blue**: Blocked Hit.

## Setup and Usage

### Basic Entity Setup

```rust
// Spawn a Combat Entity (Player/Enemy)
commands.spawn((
    // 1. Health & Shield
    Health {
        current: 100.0,
        maximum: 100.0,
        ..default()
    },
    Shield {
        current: 50.0,
        maximum: 50.0,
        ..default()
    },
    // 2. Combat Capabilities
    MeleeCombat {
        damage: 15.0,
        range: 2.5,
        ..default()
    },
    Blocking::default(),
    // 3. Identification
    Name::new("Guard"),
));
```

### Applying Damage

To hurt an entity, push a `DamageEvent` to the queue:

```rust
fn apply_damage(mut damage_queue: ResMut<DamageEventQueue>, target: Entity) {
    damage_queue.0.push(DamageEvent {
        amount: 25.0,
        damage_type: DamageType::Ranged,
        source: None,
        target: target,
        position: None,
        direction: None,
        ignore_shield: false,
    });
}
```
