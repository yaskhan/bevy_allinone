# Player System - Comprehensive Documentation

## Table of Contents

1. [Overview](#overview)
2. [Core Components](#core-components)
   - [PlayerStateSystem](#playerstatesystem)
   - [PlayerModesSystem](#playermodessystem)
   - [CharacterController Integration](#charactercontroller-integration)
3. [System Architecture](#system-architecture)
4. [Player States](#player-states)
   - [State Priorities](#state-priorities)
   - [State Interruption](#state-interruption)
   - [State Visuals (Icons)](#state-visuals-icons)
5. [Player Modes & Control States](#player-modes--control-states)
   - [Player Modes (Weapons, Powers)](#player-modes-weapons-powers)
   - [Control States (Driving, Flying)](#control-states-driving-flying)
6. [Extra Movements](#extra-movements)
   - [Fly & Jetpack](#fly--jetpack)
   - [Swim & Wall Run](#swim--wall-run)
   - [Paraglider & Sphere Mode](#paraglider--sphere-mode)
7. [Advanced Features](#advanced-features)
   - [NavMesh Override](#navmesh-override)
   - [Ragdoll System](#ragdoll-system)
   - [Sprite Animator (2D/2.5D)](#sprite-animator-2d25d)
   - [Upper Body Rotation](#upper-body-rotation)
8. [Setup and Usage](#setup-and-usage)
9. [Performance Considerations](#performance-considerations)
10. [Customization Guide](#customization-guide)

## Overview

The Player System is the central hub for player-specific logic, managing high-level states, modes, and advanced movement mechanics. It works in tandem with the [Character Controller](character_controller_system.md) to provide a responsive and feature-rich player experience. Whether it's switching between weapon modes, wall-running, or handling cutscene overrides, the Player System provides a modular and extensible framework.

## Core Components

### PlayerStateSystem

Manages a list of mutually exclusive states with priorities and durations:

```rust
#[derive(Component, Debug, Reflect)]
pub struct PlayerStateSystem {
    pub player_states_enabled: bool,
    pub player_state_list: Vec<PlayerStateInfo>,
    pub current_state_name: String,
}

#[derive(Debug, Clone, Reflect)]
pub struct PlayerStateInfo {
    pub name: String,
    pub state_priority: i32,
    pub can_be_interrupted: bool,
    pub use_state_duration: bool,
    pub state_duration: f32,
    // ...
}
```

### PlayerModesSystem

Manages high-level modes (e.g., Weapons vs. Powers) and control states (e.g., Regular vs. Driving):

```rust
#[derive(Component, Debug, Reflect)]
pub struct PlayerModesSystem {
    pub current_players_mode_name: String,
    pub player_modes: Vec<PlayerMode>,
    
    pub current_control_state_name: String,
    pub player_control_states: Vec<PlayerControlState>,
}
```

### CharacterController Integration

The player system relies on the `CharacterController` and `CharacterMovementState` for low-level physics and input mapping. The `handle_player_input` system translates `InputState` into movement directions and requests.

## System Architecture

The Player System is organized into multiple sub-plugins:

1. **Input System**: `input::handle_player_input` processes raw inputs.
2. **State Management**: `PlayerStatePlugin` handles priority-based state switching.
3. **Mode Management**: `PlayerModesPlugin` handles high-level gameplay modes.
4. **Extra Movements**: `ExtraMovementsPlugin` provides specialized locomotion (Wall Run, Fly, etc.).
5. **Visual Feedback**: `PlayerStateIconPlugin` and `SpriteAnimatorPlugin` handle UI and animations.
6. **Interaction Overrides**: `NavMeshOverridePlugin` allows external control of the character.

## Player States

### State Priorities

Every `PlayerStateInfo` has a `state_priority`. When a new state is requested via `SetPlayerStateEvent`, the system:
1. Checks if the requested state is enabled.
2. Checks if current active states allow interruption based on priority.
3. Deactivates lower-priority states to activate the new one.

### State Interruption

States can be marked as `can_be_interrupted: false`. If a non-interruptible state is active, only a state with a higher priority can replace it.

### State Visuals (Icons)

The `PlayerStateIconSystem` listens for `PlayerStateChangedEvent` and automatically shows/hides UI icons associated with specific states. It supports auto-hiding after a set duration.

## Player Modes & Control States

### Player Modes (Weapons, Powers)

Modes represent high-level gameplay contexts. For example, a character might have a "Weapon Mode" where they can shoot, and a "Magic Mode" where they cast spells.

### Control States (Driving, Flying)

Control states represent the physical context of the character.
- **Regular Mode**: Standard on-foot movement.
- **Driving**: Character is inside a vehicle; input is redirected.
- **Flying**: Character is in a free-flying state.

## Extra Movements

The system includes a variety of specialized movement components:

- **Wall Run**: Allows running horizontally along walls. Automatically detects wall surfaces and project forward velocity.
- **Jetpack**: Consumes fuel to apply upward force and horizontal momentum.
- **Fly**: Disables gravity for free-directional movement.
- **Swim**: Adjusts physics for water-based movement, including buoyancy and drag.
- **Paraglider**: Reduces fall speed and allows horizontal gliding.
- **Sphere Mode**: Morphs the character into a ball for fast, physics-based rolling.

## Advanced Features

### NavMesh Override

Used for cutscenes or scripted events. When `NavMeshOverride` is active:
1. Local player input is ignored.
2. Character moves towards a target position or entity using pathfinding.
3. Reports status like "Moving" or "Reached".

### Ragdoll System

Manages the transition between kinematic/dynamic character movement and full physics ragdoll simulations. Triggered on death or heavy impacts.

### Sprite Animator (2D/2.5D)

Designed for games using sprite sheets. It automatically flips the sprite based on movement direction and selects animations like `Idle`, `Walk`, `Run`, `Jump`, and `Fall` based on the character's velocity.

### Upper Body Rotation

Allows the character's upper body to rotate towards the camera or a target while the legs maintain movement direction. Essential for third-person shooters.

## Setup and Usage

### Basic Player Setup

```rust
use bevy::prelude::*;
use bevy_allinone::player::*;

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Name::new("Player"),
        PlayerStateSystem::default(),
        PlayerModesSystem::default(),
        // Add movement components
        WallRun::default(),
        Jetpack::default(),
        // Add character controller
        CharacterController::default(),
        CharacterMovementState::default(),
    ));
}
```

### Requesting a State Change

```rust
fn trigger_special_move(
    mut events: ResMut<SetPlayerStateQueue>
) {
    events.0.push(SetPlayerStateEvent {
        player_entity: player_ent,
        state_name: "UltimateAbility".to_string(),
    });
}
```

## Performance Considerations

1. **State Checks**: State processing is O(N) where N is the number of defined states for that player. Keep the state list reasonably sized (usually < 20).
2. **Component Granularity**: Extra movements are separate components. Only add the ones necessary for your character to save on memory and processing.
3. **Raycast Distance**: Features like Wall Run and NavMesh override use raycasts. Be mindful of the `detection_distance` parameters.

## Customization Guide

### Creating a Custom Movement

1. Define a new component (e.g., `Crawl`).
2. Implement a system that checks `InputState`.
3. Modify `CharacterMovementState` or `Transform` based on your logic.
4. Add your system to the `ExtraMovementsPlugin`.

### Adding New Player Modes

Player modes are dynamic. You can add them at runtime to the `PlayerModesSystem::player_modes` vector:

```rust
modes_system.player_modes.push(PlayerMode {
    name: "StealthMode".to_string(),
    mode_enabled: true,
    ..default()
});
```
