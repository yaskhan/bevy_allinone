# Camera System - Comprehensive Documentation

## Table of Contents

1. [Overview](#overview)
2. [Core Components](#core-components)
   - [CameraController](#cameracontroller)
   - [CameraState](#camerastate)
   - [CameraMode](#cameramode)
3. [System Architecture](#system-architecture)
4. [Camera Modes](#camera-modes)
   - [Third Person](#third-person)
   - [First Person](#first-person)
   - [Locked](#locked)
   - [Side Scroller](#side-scroller)
   - [Top Down](#top-down)
5. [Advanced Features](#advanced-features)
   - [Camera Shake](#camera-shake)
   - [Camera Bobbing](#camera-bobbing)
   - [Camera Zones](#camera-zones)
   - [Target Lock System](#target-lock-system)
   - [Waypoint System](#waypoint-system)
   - [Camera Collision](#camera-collision)
6. [Setup and Usage](#setup-and-usage)
7. [Performance Considerations](#performance-considerations)
8. [Customization Guide](#customization-guide)

## Overview

The Camera System is a robust and flexible solution for Bevy Engine, providing various camera behaviors for different game genres. It supports seamless switching between first-person and third-person perspectives, dynamic collision handling, cinematic shakes, procedurally generated bobbing, and zone-based configuration overrides.

## Core Components

### CameraController

The primary component that defines camera behavior, limits, and sensitivity:

```rust
#[derive(Component, Debug, Reflect)]
pub struct CameraController {
    pub follow_target: Option<Entity>, // Entity to follow
    pub mode: CameraMode,            // Current camera mode
    pub current_side: CameraSide,    // Right or Left shoulder
    
    // Sensitivity
    pub rot_sensitivity_3p: f32,
    pub rot_sensitivity_1p: f32,
    pub aim_zoom_sensitivity_mult: f32,
    
    // Limits
    pub min_vertical_angle: f32,
    pub max_vertical_angle: f32,
    
    // Zoom/Distance
    pub distance: f32,
    pub min_distance: f32,
    pub max_distance: f32,
    
    // Smoothing
    pub smooth_follow_speed: f32,
    pub smooth_rotation_speed: f32,
    pub pivot_smooth_speed: f32,
    pub distance_smooth_speed: f32,
    
    // Offsets
    pub default_pivot_offset: Vec3,
    pub aim_pivot_offset: Vec3,
    pub crouch_pivot_offset: Vec3,
    
    // Collision & FOV
    pub use_collision: bool,
    pub collision_radius: f32,
    pub default_fov: f32,
    pub aim_fov: f32,
}
```

### CameraState

Tracks the current runtime values and intermediate states:

```rust
#[derive(Component, Debug, Default, Reflect)]
pub struct CameraState {
    pub yaw: f32,                      // Horizontal rotation
    pub pitch: f32,                    // Vertical rotation
    pub current_distance: f32,         // Smoothly interpolated distance
    pub current_pivot: Vec3,           // Smoothly interpolated pivot
    pub current_lean: f32,             // Current leaning amount
    pub noise_offset: Vec2,            // Screen-space noise (shake/bob)
    pub bob_offset: Vec3,              // World-space bobbing offset
    pub is_aiming: bool,               // Aiming state
    pub is_crouching: bool,            // Crouching state
}
```

### CameraMode

Defines the perspective and control scheme:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect, Default)]
pub enum CameraMode {
    #[default]
    ThirdPerson,   // Classic over-the-shoulder
    FirstPerson,   // Eye-level view
    Locked,        // Fixed orientation following target
    SideScroller,  // Restricted to 2D plane
    TopDown,       // Overhead view
}
```

## System Architecture

The Camera System utilizes a modular approach with specialized systems handled in a specific order:

### Update Systems
- `update_camera_state_offsets` - Updates pivot and FOV based on character state (aiming, crouching)
- `update_target_marking` - Identifies potential targets in view
- `update_target_lock` - Handles smooth orientation towards a locked target
- `update_camera_zones` - Detects when the player enters a `CameraZone`
- `apply_camera_zone_settings` - Overrides camera settings based on active zones
- `update_camera_rotation` - Processes mouse/joystick input for rotation
- `update_camera_shake` - Calculates and applies screen shake effects
- `update_camera_bob` - Calculates view bobbing based on movement
- `update_camera_follow` - Main system that positions the camera in the world
- `handle_camera_collision` - Adjusts camera distance to prevent clipping through walls
- `update_camera_fov` - Smoothly transitions FOV values

## Camera Modes

### Third Person
The default mode. It follows the target with a pivot offset and allows full orbital rotation. It supports shoulder switching (Left/Right) and dynamic distance adjustment.

### First Person
Attaches the camera directly to the target's "head" (pivot). It disables camera collision with the player model and typically uses higher rotation sensitivity.

### Locked
Keeps the camera at a fixed orientation relative to the world or the target, regardless of input. Useful for cinematic sequences or specific gameplay mechanics.

### Side Scroller
Locks the camera to a specific axis (usually Z or X), allowing only 2D movement. It transitions the `CameraController` to ignore depth-based rotation.

### Top Down
Positions the camera above the player looking down. It often uses a fixed pitch and allows the player to rotate only the yaw.

## Advanced Features

### Camera Shake
Uses a `ShakeQueue` resource and `CameraShakeInstance` components to provide high-frequency noise.
- **Point Shakes**: World-space entities that trigger shakes for nearby cameras (e.g., explosions).
- **Presets**: Configurable intensity, duration, and frequency for different effects.

### Camera Bobbing
Procedural movement applied when the target character is moving.
- **Idle**: Subtle "breathing" movement.
- **Walk/Sprint**: rhythmic movement synchronized with movement speed.
- **Aim**: Minimal bobbing to avoid disrupting player aim.

### Camera Zones
Trigger volumes (`CameraZone`) that change camera behavior when entered.
- **Overridable Settings**: Mode, distance, FOV, pivot offset, etc.
- **Priority System**: Handles overlapping zones.
- **Smoothing**: Settings transition smoothly when entering/exiting zones.

### Target Lock System
Allows the camera to automatically track an entity.
- **Scanning**: Detects entities within a central screen radius.
- **Smooth Tracking**: Uses interpolation for natural-feeling camera movement.
- **Manual Break**: Players can break the lock by moving the camera forcefully.

### Waypoint System
Enables cinematic movement through pre-defined paths.
- **CameraWaypointTrack**: A collection of entities defining a path.
- **Interpolation**: Supports linear or smooth transitions between points.
- **Wait Times**: Configurable pauses at specific waypoints.

### Camera Collision
Uses raycasting to detect obstacles between the pivot and the camera.
- **Smoothing**: Prevents jarring "pops" when hitting walls.
- **Lean Collision**: Special handling for corner cases.
- **Transparency**: Can make obstructing objects transparent instead of moving the camera.

## Setup and Usage

### Spawning a Camera

```rust
use bevy::prelude::*;
use bevy_allinone::camera::*;

fn setup(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    if let Ok(player) = player_query.get_single() {
        spawn_camera(&mut commands, player);
    }
}
```

### Manual Configuration

```rust
commands.spawn((
    Camera3d::default(),
    CameraController {
        mode: CameraMode::ThirdPerson,
        distance: 5.0,
        max_distance: 15.0,
        smooth_follow_speed: 10.0,
        ..default()
    },
    CameraState::default(),
    CameraBobState::default(),
));
```

## Performance Considerations

1. **Raycast Frequency**: Camera collision uses raycasts every frame. Limit the complexity of collision layers.
2. **Entity Queries**: Zone detection checks for triggers; use spatial partitioning for large levels.
3. **Interpolation**: Extensive use of `lerp` for smoothing is efficient, but ensure `dt` is handled correctly.

## Customization Guide

### Adding a New Shake Preset

```rust
let request = ShakeRequest {
    name: "HeavyExplosion".to_string(),
    intensity: 2.0,
    duration: Some(1.5),
};
shake_queue.0.push(request);
```

### Creating a Camera Zone

```rust
commands.spawn((
    CameraZone {
        settings: CameraZoneSettings {
            mode: CameraMode::SideScroller,
            distance: Some(8.0),
            fixed_pitch: Some(-10.0),
            ..default()
        },
        priority: 1,
    },
    Collider::cuboid(10.0, 5.0, 10.0),
    Sensor,
    Transform::from_xyz(50.0, 0.0, 0.0),
));
```

## Troubleshooting

- **Camera clipping**: Check `collision_radius` and collision layers.
- **Jumpy movement**: Ensure `smooth_follow_speed` is not too high.
- **Mode switch issues**: Verify `handle_camera_mode_switch` system order.
