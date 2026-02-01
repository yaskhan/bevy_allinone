# Character Controller System - Comprehensive Documentation

## Table of Contents

1. [Overview](#overview)
2. [Core Components](#core-components)
   - [CharacterController](#charactercontroller)
   - [CharacterMovementState](#charactermovementstate)
   - [CharacterAnimationState](#characteranimationstate)
3. [System Architecture](#system-architecture)
4. [Movement Mechanics](#movement-mechanics)
   - [Basic Movement](#basic-movement)
   - [Jumping and Falling](#jumping-and-falling)
   - [Crouching and Sliding](#crouching-and-sliding)
   - [Obstacle Detection](#obstacle-detection)
   - [Wall Running](#wall-running)
5. [Advanced Features](#advanced-features)
   - [Tank Controls](#tank-controls)
   - [Root Motion](#root-motion)
   - [Axis Constraints (2.5D)](#axis-constraints-25d)
   - [Zero Gravity Mode](#zero-gravity-mode)
6. [Physics Integration](#physics-integration)
   - [Step Up Logic](#step-up-logic)
   - [Step Down Logic](#step-down-logic)
   - [Friction Management](#friction-management)
   - [Falling Damage](#falling-damage)
7. [Animation System](#animation-system)
8. [Setup and Usage](#setup-and-usage)
9. [Performance Considerations](#performance-considerations)
10. [Customization Guide](#customization-guide)

## Overview

The Character Controller System is a comprehensive 3D/2.5D movement system designed for Bevy Engine. It provides realistic character movement with features like walking, running, sprinting, jumping, crouching, sliding, wall running, and advanced physics interactions.

## Core Components

### CharacterController

The main component that defines character behavior and capabilities:

```rust
#[derive(Component, Debug, Reflect)]
pub struct CharacterController {
    // Movement speeds
    pub walk_speed: f32,           // Default: 4.0
    pub run_speed: f32,            // Default: 7.0
    pub sprint_speed: f32,         // Default: 10.0
    pub crouch_speed: f32,         // Default: 2.5

    // Rotation settings
    pub turn_speed: f32,           // Default: 10.0
    pub stationary_turn_speed: f32, // Default: 180.0
    pub moving_turn_speed: f32,    // Default: 200.0
    pub use_tank_controls: bool,    // Default: false

    // Jump settings
    pub jump_power: f32,           // Default: 6.0
    pub jump_hold_bonus: f32,      // Default: 2.0
    pub max_jump_hold_time: f32,   // Default: 0.25

    // State flags
    pub can_move: bool,            // Default: true
    pub is_dead: bool,             // Default: false
    pub is_strafing: bool,         // Default: false

    // Movement smoothing
    pub acceleration: f32,         // Default: 10.0
    pub deceleration: f32,         // Default: 15.0

    // Falling damage
    pub fall_damage_enabled: bool, // Default: true
    pub min_velocity_for_damage: f32, // Default: 12.0
    pub falling_damage_multiplier: f32, // Default: 5.0

    // Crouch sliding
    pub crouch_sliding_enabled: bool, // Default: true
    pub crouch_sliding_speed: f32,    // Default: 12.0
    pub crouch_sliding_duration: f32, // Default: 1.0

    // Obstacle detection
    pub obstacle_detection_distance: f32, // Default: 0.5

    // Advanced features
    pub fixed_axis: Option<Vec3>,   // For 2.5D games
    pub use_root_motion: bool,      // Default: false
    pub zero_gravity_mode: bool,    // Default: false
    pub free_floating_mode: bool,   // Default: false
}
```

### CharacterMovementState

Tracks the current movement state and internal timers:

```rust
#[derive(Component, Debug, Default, Reflect)]
pub struct CharacterMovementState {
    // Movement data
    pub raw_move_dir: Vec3,        // Raw input direction
    pub lerped_move_dir: Vec3,     // Smoothed movement direction
    pub is_running: bool,          // Currently running
    pub is_sprinting: bool,        // Currently sprinting
    pub is_crouching: bool,        // Currently crouching
    pub wants_to_jump: bool,       // Jump requested
    pub jump_held: bool,           // Jump button held
    pub current_speed: f32,        // Current movement speed
    pub current_normal: Vec3,      // Current ground normal

    // Internal state
    pub last_vertical_velocity: f32, // For fall damage calculation
    pub air_time: f32,              // Time spent in air
    pub jump_hold_timer: f32,       // Jump hold duration
    pub crouch_sliding_active: bool, // Crouch slide active
    pub crouch_sliding_timer: f32,  // Crouch slide duration
    pub obstacle_found: bool,       // Obstacle detected
    pub quick_turn_active: bool,    // Quick turn in progress
    pub quick_turn_timer: f32,      // Quick turn duration

    // Wall running
    pub wall_running_active: bool,  // Wall running active
    pub wall_side: Option<Vec3>,    // Wall normal for wall running

    // Root motion
    pub root_motion_translation: Vec3, // Root motion translation
    pub root_motion_rotation: Quat,    // Root motion rotation

    // Vehicle state
    pub is_in_vehicle: bool,        // In vehicle flag
    pub vehicle_entity: Option<Entity>, // Vehicle entity
}
```

### CharacterAnimationState

Manages animation transitions and blending:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
pub enum CharacterAnimationMode {
    #[default]
    Idle,
    Walk,
    Run,
    Sprint,
    CrouchIdle,
    CrouchWalk,
    JumpStart,
    JumpAir,
    Fall,
    Land,
}

#[derive(Component, Debug, Default, Reflect)]
pub struct CharacterAnimationState {
    pub mode: CharacterAnimationMode, // Current animation mode
    pub forward: f32,                // Forward movement blend
    pub turn: f32,                   // Turn movement blend
}
```

## System Architecture

The character controller uses a multi-system approach with proper execution order:

### Update Systems (Frame-rate dependent)
- `handle_character_input` - Processes input and updates movement state
- `update_character_movement` - Calculates current movement speed
- `update_character_rotation` - Handles character orientation
- `update_character_animation` - Updates animation state

### FixedUpdate Systems (Physics timestep)
- `apply_character_physics` - Applies physics and movement
- `check_ground_state` - Updates ground detection
- `update_friction_material` - Adjusts friction based on movement
- `handle_falling_damage` - Calculates fall damage
- `handle_crouch_sliding` - Manages crouch sliding
- `handle_obstacle_detection` - Detects obstacles
- `handle_wall_running_detection` - Detects wall running opportunities

## Movement Mechanics

### Basic Movement

The system uses a dual-direction approach:
- `raw_move_dir`: Direct input from player
- `lerped_move_dir`: Smoothed direction for natural movement

Movement speed is determined by:
1. Current state (crouching, sprinting, etc.)
2. Acceleration/deceleration curves
3. Obstacle detection

### Jumping and Falling

**Jump Features:**
- Variable jump height based on hold duration
- Jump buffering for responsive controls
- Air time tracking for animation
- Fall damage calculation

**Jump Process:**
1. Player presses jump button
2. System checks if grounded
3. Applies initial jump velocity
4. Tracks jump hold time for variable height
5. Applies bonus velocity while holding jump

### Crouching and Sliding

**Crouch Features:**
- Reduces movement speed
- Changes collision shape
- Enables sliding when sprinting into crouch

**Slide Process:**
1. Player sprints and crouches simultaneously
2. System activates slide state
3. Applies slide speed and duration
4. Gradually decelerates to crouch speed

### Obstacle Detection

Uses dual raycasts (left and right) to detect obstacles:
- Prevents movement through walls
- Enables wall running when appropriate
- Provides feedback for animation systems

### Wall Running

**Wall Running Features:**
- Automatic detection of runnable walls
- Maintains momentum while on walls
- Applies force to stick to walls
- Smooth transitions to/from wall running

## Advanced Features

### Tank Controls

Alternative control scheme where:
- Movement is relative to character orientation
- Left/right inputs rotate the character
- Forward/backward moves in current facing direction

### Root Motion

Allows animation-driven movement:
- Animation system provides translation/rotation deltas
- Physics system applies these deltas
- Enables cinematic movement and precise animation matching

### Axis Constraints (2.5D)

Locks character to specific axes for 2.5D games:
- `fixed_axis` defines the constraint plane
- Prevents movement outside the plane
- Useful for side-scrollers and isometric games

### Zero Gravity Mode

Disables gravity and enables 3D movement:
- Movement in all directions
- Special physics handling
- Useful for space games or special abilities

## Physics Integration

### Step Up Logic

Allows characters to climb small obstacles:
1. Detects steps using dual raycasts (feet and knee level)
2. Calculates step height
3. Smoothly lifts character onto step
4. Prevents upward bounce

### Step Down Logic

Prevents characters from getting stuck on ledges:
1. Detects when character leaves a ledge
2. Searches for ground below
3. Snaps character down if ground is within range
4. Prevents floating at edge of platforms

### Friction Management

Dynamically adjusts friction based on movement:
- High friction when stationary
- Low friction when moving
- Prevents sliding when stopping

### Falling Damage

Calculates damage based on:
- Fall velocity
- Air time
- Configurable damage formula
- Can be disabled per character

## Animation System

The animation system automatically determines the appropriate animation mode based on:

**Grounded States:**
- Idle (no movement)
- Walk (slow movement)
- Run (normal movement)
- Sprint (fast movement)
- CrouchIdle (crouching, no movement)
- CrouchWalk (crouching, moving)

**Airborne States:**
- JumpStart (beginning of jump)
- JumpAir (ascending)
- Fall (descending)
- Land (landing)

## Setup and Usage

### Basic Setup

```rust
use bevy::prelude::*;
use bevy_allinone::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GameControllerPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Spawn a character
    let character = spawn_character(&mut commands, Vec3::new(0.0, 1.0, 0.0));
    
    // You can customize the controller
    commands.entity(character).insert(CharacterController {
        walk_speed: 5.0,
        sprint_speed: 12.0,
        jump_power: 7.0,
        ..default()
    });
}
```

### Advanced Setup

```rust
fn setup_advanced(mut commands: Commands) {
    // Create custom character
    let character = commands.spawn((
        Name::new("Custom Player"),
        CharacterController {
            walk_speed: 3.5,
            run_speed: 6.0,
            sprint_speed: 9.0,
            crouch_speed: 2.0,
            jump_power: 5.5,
            jump_hold_bonus: 1.5,
            max_jump_hold_time: 0.2,
            acceleration: 8.0,
            deceleration: 12.0,
            crouch_sliding_enabled: true,
            crouch_sliding_speed: 10.0,
            crouch_sliding_duration: 0.8,
            ..default()
        },
        CharacterMovementState::default(),
        CharacterAnimationState::default(),
        Transform::from_translation(Vec3::new(0.0, 2.0, 0.0)),
        GlobalTransform::default(),
    ))
    .insert((
        RigidBody::Dynamic,
        Collider::capsule(0.4, 1.0),
        LockedAxes::ROTATION_LOCKED,
        GravityScale(1.0),
        Friction::new(0.0),
        Restitution::new(0.0),
        LinearVelocity::default(),
        AngularVelocity::default(),
    ))
    .id();
}
```

## Performance Considerations

### Optimization Techniques

1. **System Chaining**: Systems are properly chained for efficient execution
2. **Query Filtering**: Only processes entities with required components
3. **Spatial Queries**: Uses efficient raycasting for physics interactions
4. **Component Organization**: Data is organized for cache efficiency

### Performance Tips

- Limit the number of active characters
- Use simpler collision shapes for NPCs
- Disable unnecessary features for background characters
- Use fixed timestep for physics consistency

## Customization Guide

### Creating Custom Movement Modes

```rust
// Add custom fields to CharacterController
#[derive(Component, Debug, Reflect)]
pub struct CharacterController {
    // ... existing fields ...
    pub custom_movement_enabled: bool,
    pub custom_movement_speed: f32,
}

// Extend CharacterMovementState
#[derive(Component, Debug, Default, Reflect)]
pub struct CharacterMovementState {
    // ... existing fields ...
    pub is_custom_moving: bool,
}

// Add custom movement logic
fn update_custom_movement(
    mut query: Query<(&CharacterController, &mut CharacterMovementState)>
) {
    for (controller, mut state) in query.iter_mut() {
        if controller.custom_movement_enabled {
            state.is_custom_moving = true;
            state.current_speed = controller.custom_movement_speed;
        }
    }
}
```

### Extending Animation System

```rust
// Add custom animation modes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
pub enum CharacterAnimationMode {
    // ... existing modes ...
    CustomMode,
    SpecialAbility,
}

// Update animation system
fn update_character_animation(
    mut query: Query<(&CharacterMovementState, &mut CharacterAnimationState)>
) {
    for (movement, mut anim) in query.iter_mut() {
        // ... existing logic ...
        
        if movement.is_custom_moving {
            anim.mode = CharacterAnimationMode::CustomMode;
        }
    }
}
```

## Best Practices

1. **Component Organization**: Keep related data together in components
2. **System Separation**: Each system should have a single responsibility
3. **Configuration**: Use default values for easy setup
4. **Documentation**: Document custom extensions thoroughly
5. **Testing**: Test custom movement in various scenarios

## Troubleshooting

### Common Issues

**Character doesn't move:**
- Check `can_move` flag
- Verify input system is working
- Ensure physics components are attached

**Character falls through ground:**
- Check ground detection settings
- Verify collision layers
- Ensure proper physics setup

**Animation issues:**
- Verify animation state transitions
- Check blend values
- Ensure animation controller is properly configured

## Future Enhancements

Potential areas for expansion:
- Swimming and underwater movement
- Climbing systems
- Advanced parkour mechanics
- Character states (fatigue, injuries)
- Environmental effects (wind, slippery surfaces)
