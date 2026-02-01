# Camera System Documentation

## Table of Contents

1. [Overview](#overview)
2. [Core Concepts](#core-concepts)
3. [Component Reference](#component-reference)
4. [Camera Modes](#camera-modes)
5. [Advanced Features](#advanced-features)
6. [System Integration](#system-integration)
7. [Usage Patterns](#usage-patterns)
8. [Best Practices](#best-practices)
9. [Troubleshooting](#troubleshooting)

---

## Overview

The Camera System is a comprehensive, modular camera controller designed for 3D/2.5D games built with Bevy 0.18. It provides sophisticated camera controls, physics-based collision detection, dynamic state management, and advanced cinematic features. The system is built with flexibility and performance in mind, supporting multiple camera modes, real-time effects, and seamless integration with other game systems.

### Key Features

- **Multiple Camera Modes**: Third-person, first-person, locked, side-scroller, and top-down perspectives
- **Physics-Based Collision**: Raycast-based camera collision with configurable transparency and culling
- **Dynamic State Management**: Real-time camera state transitions based on player actions
- **Advanced Targeting**: Target marking, locking, and tracking with configurable behavior
- **Cinematic Systems**: Waypoint-based cutscenes, camera zones, and bounds
- **Motion Simulation**: Camera bobbing, shake effects, and environmental responses
- **Performance Optimized**: Efficient spatial queries and smooth interpolation algorithms

### Architecture Overview

The Camera System follows a plugin-based architecture with clear separation of concerns:

```
Camera Plugin
├── Core Components (CameraController, CameraState)
├── Movement Systems (Follow, Rotation, Collision)
├── Effect Systems (Shake, Bob, FOV)
├── Targeting Systems (Marking, Locking)
├── Cinematic Systems (Waypoints, Zones, Bounds)
├── Advanced Features (Effects, Captures, Vehicles)
└── Integration Systems (State Offsets, Lean, Transparency)
```

The system integrates deeply with:
- **Input System**: Mouse/keyboard controls, sensitivity settings
- **Character System**: Movement state synchronization, pivot point calculation
- **Physics System**: Raycast-based collision detection and spatial queries
- **Combat System**: Target marking and locking functionality
- **UI System**: Crosshair, reticle, and camera-based overlays

---

## Core Concepts

### Camera Coordinate System

The Camera System uses a right-handed 3D coordinate system with the following conventions:
- **X-axis**: Horizontal movement (right = positive)
- **Y-axis**: Vertical movement (up = positive)
- **Z-axis**: Depth movement (forward = negative)

### Pivot Point System

The camera system operates on a pivot-based architecture where the camera position is calculated relative to a target pivot point:

1. **Base Pivot**: Calculated from character transform + offset
2. **Dynamic Pivot**: Modified by aiming, crouching, and state changes
3. **Collision Pivot**: Adjusted based on physics collision detection
4. **Final Position**: Pivot point + backward direction * distance + effects

### State Machine Architecture

Camera states are managed through a sophisticated state machine that supports:

- **Manual Controls**: User-driven camera rotation and movement
- **Automatic Systems**: Auto-centering, target following, zone-based overrides
- **Cinematic Modes**: Waypoint following, cutscene playback, scripted movements
- **Emergency States**: Collision avoidance, boundary enforcement

### Smooth Interpolation

All camera movements use exponential smoothing for natural motion:

```rust
// Exponential smoothing formula
let alpha = 1.0 - (-speed * delta_time).exp();
current_value = current_value + (target_value - current_value) * alpha;
```

This provides frame-rate independent smoothing with configurable response curves.

### Spatial Query Integration

The system heavily relies on Avian3D's spatial query system for:
- Camera collision detection
- Target acquisition and tracking
- Zone detection and management
- Transparency and culling calculations

---

## Component Reference

### Core Camera Components

#### CameraController

The primary controller component that manages camera behavior and configuration:

```rust
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct CameraController {
    pub follow_target: Option<Entity>,
    pub mode: CameraMode,
    pub current_side: CameraSide,
    
    // Sensitivity Settings
    pub rot_sensitivity_3p: f32,
    pub rot_sensitivity_1p: f32,
    pub aim_zoom_sensitivity_mult: f32,
    
    // Angular Limits
    pub min_vertical_angle: f32,
    pub max_vertical_angle: f32,
    
    // Distance Settings
    pub distance: f32,
    pub min_distance: f32,
    pub max_distance: f32,
    
    // Smoothing Parameters
    pub smooth_follow_speed: f32,
    pub smooth_rotation_speed: f32,
    pub pivot_smooth_speed: f32,
    pub distance_smooth_speed: f32,
    
    // Dynamic Offsets
    pub side_offset: f32,
    pub default_pivot_offset: Vec3,
    pub aim_pivot_offset: Vec3,
    pub crouch_pivot_offset: Vec3,
    
    // Leaning Configuration
    pub lean_amount: f32,
    pub lean_angle: f32,
    pub lean_speed: f32,
    pub lean_raycast_dist: f32,
    pub lean_wall_angle: f32,
    
    // Field of View Settings
    pub default_fov: f32,
    pub aim_fov: f32,
    pub fov_speed: f32,
    
    // Collision Settings
    pub use_collision: bool,
    pub collision_radius: f32,
    
    // Targeting System
    pub target_lock: TargetLockSettings,
    
    // State Management
    pub states: Vec<CameraStateInfo>,
    pub current_state_name: String,
    
    // Baseline Configuration
    pub base_mode: CameraMode,
    pub base_distance: f32,
    pub base_fov: f32,
    pub base_pivot_offset: Vec3,
    pub base_transition_speed: f32,
    pub enabled: bool,
}
```

**Key Properties:**

- `follow_target`: Entity the camera should track (typically player character)
- `mode`: Current camera mode (ThirdPerson, FirstPerson, Locked, etc.)
- `sensitivity_*`: Mouse/control sensitivity multipliers for different modes
- `min/max_vertical_angle`: Pitch limits in degrees
- `distance_*`: Camera distance from pivot point
- `smooth_*_speed`: Interpolation speeds for different camera aspects
- `pivot_offset_*`: Pivot point offsets for different player states
- `lean_*`: Wall-leaning configuration and raycast settings
- `fov_*`: Field of view settings for different camera states
- `collision_*`: Camera collision detection configuration
- `target_lock`: Target acquisition and locking parameters
- `states`: Predefined camera states for different game situations
- `base_*`: Baseline values for smooth restoration after overrides

#### CameraState

Runtime camera state tracking component:

```rust
#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct CameraState {
    pub yaw: f32,
    pub pitch: f32,
    pub current_distance: f32,
    pub current_pivot: Vec3,
    pub current_side_interpolator: f32,
    pub current_lean: f32,
    pub noise_offset: Vec2,
    pub bob_offset: Vec3,
    pub is_aiming: bool,
    pub is_crouching: bool,
    pub fov_override: Option<f32>,
    pub fov_override_speed: Option<f32>,
}
```

**Properties:**

- `yaw`, `pitch`: Current camera rotation in degrees
- `current_distance`: Actual distance from pivot (may differ from target)
- `current_pivot`: Current world-space pivot position
- `current_side_interpolator`: Smoothed side value (-1.0 to 1.0)
- `current_lean`: Current lean angle
- `noise_offset`: Additive rotation offsets from effects
- `bob_offset`: Position offsets from bobbing effects
- `is_aiming`, `is_crouching`: State flags for dynamic behavior
- `fov_override`, `fov_override_speed`: Temporary FOV overrides

#### CameraTargetState

Target tracking and management component:

```rust
#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct CameraTargetState {
    pub marked_target: Option<Entity>,
    pub locked_target: Option<Entity>,
    pub is_locking: bool,
}
```

**Properties:**

- `marked_target`: Currently highlighted/marked target entity
- `locked_target`: Entity currently locked by camera
- `is_locking`: Whether target locking is active

### Camera Modes

#### CameraMode Enum

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect, Default)]
pub enum CameraMode {
    #[default]
    ThirdPerson,
    FirstPerson,
    Locked,
    SideScroller,
    TopDown,
}
```

**Mode Characteristics:**

- **ThirdPerson**: Behind-shoulder view with full rotation and zoom
- **FirstPerson**: Eye-level view with limited pitch range
- **Locked**: Fixed orientation following character movement
- **SideScroller**: 2D side view with limited depth
- **TopDown**: Overhead view with orthographic projection

### Advanced Components

#### CameraZone

Trigger volumes that modify camera behavior:

```rust
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct CameraZone {
    pub settings: CameraZoneSettings,
    pub priority: i32,
}
```

#### CameraZoneSettings

Configuration for zone-based camera modifications:

```rust
#[derive(Debug, Clone, Reflect)]
pub struct CameraZoneSettings {
    pub mode: CameraMode,
    pub distance: Option<f32>,
    pub pivot_offset: Option<Vec3>,
    pub fov: Option<f32>,
    pub fixed_yaw: Option<f32>,
    pub fixed_pitch: Option<f32>,
    pub follow_rotation: bool,
    pub look_at_player: bool,
    pub transition_speed: f32,
}
```

#### CameraWaypoint and Track System

```rust
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct CameraWaypoint {
    pub wait_time: f32,
    pub movement_speed: Option<f32>,
    pub rotation_speed: Option<f32>,
    pub rotation_mode: WaypointRotationMode,
    pub look_at_target: Option<Entity>,
}

#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct CameraWaypointTrack {
    pub waypoints: Vec<Entity>,
    pub loop_track: bool,
}
```

#### Shake and Effect Components

```rust
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct CameraShakeInstance {
    pub camera_entity: Option<Entity>,
    pub name: String,
    pub timer: f32,
    pub duration: f32,
    pub intensity: f32,
    pub pos_amount: Vec3,
    pub rot_amount: Vec3,
    pub pos_speed: Vec3,
    pub rot_speed: Vec3,
    pub pos_smooth: f32,
    pub rot_smooth: f32,
    pub current_pos: Vec3,
    pub current_rot: Vec3,
    pub decrease_in_time: bool,
}
```

#### Bobbing System

```rust
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct CameraBobState {
    pub phase: f32,
    pub current_pos_offset: Vec3,
    pub current_rot_offset: Vec3,
    
    pub idle: BobPreset,
    pub walk: BobPreset,
    pub sprint: BobPreset,
    pub aim: BobPreset,
}
```

#### Transparency and Culling

```rust
#[derive(Resource, Reflect, Clone)]
#[reflect(Resource)]
pub struct TransparencySettings {
    pub enabled: bool,
    pub alpha_target: f32,
    pub fade_speed: f32,
    pub ray_radius: f32,
}

#[derive(Resource, Reflect, Clone)]
#[reflect(Resource)]
pub struct PlayerCullingSettings {
    pub enabled: bool,
    pub min_dist: f32,
    pub fade_speed: f32,
    pub min_alpha: f32,
}
```

---

## Camera Modes

### Third-Person Mode

The default camera mode providing a behind-shoulder perspective:

**Characteristics:**
- Full 360-degree rotation around character
- Adjustable distance from character
- Collision detection enabled
- Dynamic FOV changes for aiming
- Side preference (left/right shoulder)

**Configuration:**
```rust
camera.mode = CameraMode::ThirdPerson;
camera.distance = 4.0;
camera.min_distance = 1.5;
camera.max_distance = 8.0;
camera.side_offset = 0.5;
```

**Dynamic Behavior:**
- Auto-centers on character movement when no input
- Aiming mode reduces distance and FOV
- Crouching lowers pivot point
- Wall leaning adjusts camera position

### First-Person Mode

Eye-level perspective with limited vertical rotation:

**Characteristics:**
- Limited pitch range (typically -80° to 80°)
- Fixed distance (no zoom)
- Higher sensitivity for precise aiming
- No collision detection by default

**Configuration:**
```rust
camera.mode = CameraMode::FirstPerson;
camera.rot_sensitivity_1p = 0.1;
camera.min_vertical_angle = -85.0;
camera.max_vertical_angle = 85.0;
camera.use_collision = false; // Optional
```

**Special Considerations:**
- Head bobbing is more subtle
- Weapon view models integrate seamlessly
- Sniper scopes provide FOV overrides

### Locked Mode

Fixed orientation camera that follows character movement:

**Characteristics:**
- Camera rotation follows character rotation
- No manual camera rotation input
- Fixed distance and FOV
- Useful for specific gameplay sections

**Configuration:**
```rust
camera.mode = CameraMode::Locked;
camera.smooth_rotation_speed = 15.0;
camera.distance = 3.0;
```

### Side-Scroller Mode

2D-style camera with limited depth:

**Characteristics:**
- Constrained to horizontal movement
- Fixed camera height
- Character-centered tracking
- Limited vertical rotation

**Configuration:**
```rust
camera.mode = CameraMode::SideScroller;
camera.distance = 6.0;
camera.min_vertical_angle = 0.0;
camera.max_vertical_angle = 0.0;
```

### Top-Down Mode

Overhead perspective with orthographic or perspective projection:

**Characteristics:**
- High vantage point
- Minimal rotation
- Wide field of view
- Strategic overview of surroundings

**Configuration:**
```rust
camera.mode = CameraMode::TopDown;
camera.distance = 15.0;
camera.default_fov = 75.0;
camera.pivot_smooth_speed = 20.0;
```

---

## Advanced Features

### Target Locking System

The target locking system provides sophisticated enemy tracking:

#### TargetMarking

Automatic target highlighting based on:
- Distance from camera
- Angle relative to camera forward
- Line of sight validation
- Target health status

**Scoring Algorithm:**
```rust
let score = dist * 0.5 + angle * 2.0;
```

**Configuration:**
```rust
camera.target_lock.enabled = true;
camera.target_lock.max_distance = 30.0;
camera.target_lock.fov_threshold = 45.0;
camera.target_lock.scan_radius = 2.0;
```

#### TargetLocking

Manual target acquisition with:
- Toggle locking via input
- Smooth camera rotation to target
- Lock maintenance with distance/angle limits
- Auto-unlock on target death

**Locking Behavior:**
- Smooth rotation interpolation to target
- Distance-based auto-unlock (max_distance + 5.0)
- Visual indicators and UI integration

### Camera Collision System

Physics-based collision detection prevents camera clipping:

#### Collision Detection

```rust
pub fn handle_camera_collision(
    spatial_query: SpatialQuery,
    mut query: Query<(&CameraController, &CameraState, &mut Transform)>,
) {
    for (camera, state, mut transform) in query.iter_mut() {
        if !camera.use_collision { continue; }

        let start = state.current_pivot;
        let direction = transform.back();
        let max_dist = state.current_distance;
        let filter = SpatialQueryFilter::default();

        if let Some(hit) = spatial_query.cast_ray(start, direction, max_dist, true, &filter) {
            transform.translation = start + direction * (hit.distance - camera.collision_radius);
        }
    }
}
```

#### Wall Leaning

Camera leans against walls to maintain visibility:

**Leaning Process:**
1. Raycast forward from camera position
2. Detect wall proximity within lean distance
3. Calculate lean angle based on wall angle
4. Smoothly interpolate lean offset
5. Apply lean to camera position and rotation

**Configuration:**
```rust
camera.lean_amount = 0.4;
camera.lean_angle = 15.0;
camera.lean_raycast_dist = 0.8;
camera.lean_wall_angle = 5.0;
```

### Camera Zones

Area-based camera behavior modification:

#### Zone Detection

Uses spatial point intersections to detect player position within zones:

```rust
let intersections = spatial_query.point_intersections(player_pos, &filter);
```

#### Zone Priority System

Zones are evaluated by priority with the highest priority taking effect:

```rust
for &zone_ent in tracker.active_zones.iter() {
    if let Ok(zone) = zone_query.get(zone_ent) {
        if zone.priority > max_priority {
            max_priority = zone.priority;
            best_zone = Some(zone_ent);
        }
    }
}
```

#### Transition Smoothing

Zone transitions use exponential smoothing:

```rust
let alpha = 1.0 - (-speed * dt).exp();
controller.distance = controller.distance + (target_dist - controller.distance) * alpha;
```

### Cinematic Camera System

#### Waypoint System

Complex camera movements through predefined paths:

**Waypoint Configuration:**
```rust
let waypoint = CameraWaypoint {
    wait_time: 2.0,
    movement_speed: Some(5.0),
    rotation_speed: Some(10.0),
    rotation_mode: WaypointRotationMode::LookAtTarget,
    look_at_target: Some(player_entity),
};
```

**Rotation Modes:**
- `UseWaypointRotation`: Use waypoint's rotation
- `FaceMovement`: Look in direction of movement
- `LookAtTarget`: Focus on specific entity

#### Cutscene Integration

Camera cutscenes integrate with:
- Dialog system for talking heads
- Quest system for story progression
- Puzzle system for tutorials
- Combat system for dramatic angles

### Shake and Effect System

#### Camera Shake

Procedural camera movement for impact feedback:

**Shake Types:**
- Point-based shakes (explosions, impacts)
- Directional shakes (nearby events)
- Screen-space shakes (UI feedback)

**Shake Parameters:**
```rust
let shake = CameraShakeInstance {
    duration: 0.5,
    intensity: 1.0,
    pos_amount: Vec3::new(0.05, 0.05, 0.05),
    rot_amount: Vec3::new(2.0, 2.0, 2.0),
    pos_speed: Vec3::new(15.0, 15.0, 15.0),
    rot_speed: Vec3::new(15.0, 15.0, 15.0),
    decrease_in_time: true,
};
```

#### Camera Bobbing

Movement-based camera simulation:

**Bob Presets:**
- **Idle**: Subtle breathing motion
- **Walk**: Walking rhythm simulation
- **Sprint**: High-intensity running motion
- **Aim**: Minimal stabilization

**Bob Calculation:**
```rust
let target_pos = Vec3::new(
    (t * preset.pos_speed.x).sin() * preset.pos_amount.x,
    (t * preset.pos_speed.y).sin() * preset.pos_amount.y,
    (t * preset.pos_speed.z).cos() * preset.pos_amount.z,
);
```

### Field of View Management

Dynamic FOV changes based on:

#### State-Based FOV
- Default FOV for normal viewing
- Aim FOV for precision targeting
- Sprint FOV for motion intensity

#### Override System
```rust
state.fov_override = Some(target_fov);
state.fov_override_speed = Some(speed);
```

#### FOV Transition
```rust
let alpha = 1.0 - (-speed * time.delta_secs()).exp();
p.fov = p.fov + (target_rad - p.fov) * alpha;
```

---

## System Integration

### Input System Integration

The camera system integrates with the Input System for:

#### Mouse Input
- Look direction and sensitivity
- Smooth interpolation of rotation
- Dynamic sensitivity based on camera mode

#### Keyboard Input
- Camera mode switching
- Side switching (left/right shoulder)
- Zoom controls
- Special camera functions

#### Input Configuration
```rust
pub struct InputState {
    pub look: Vec2,              // Mouse input
    pub aim_pressed: bool,       // Aim state
    pub switch_camera_mode_pressed: bool,
    pub side_switch_pressed: bool,
    pub lock_on_pressed: bool,
}
```

### Character System Integration

#### Movement State Synchronization
- Pivot point calculation based on character position
- State-based offset application (crouch, sprint, aim)
- Animation state integration for realistic movement

#### Character Movement States
```rust
pub struct CharacterMovementState {
    pub current_speed: f32,
    pub is_sprinting: bool,
    pub is_crouching: bool,
    pub is_climbing: bool,
    // ... additional states
}
```

### Physics System Integration

#### Spatial Query Usage
- Camera collision detection
- Target acquisition
- Zone detection
- Transparency calculations

#### Collision Handling
- Raycast-based collision
- Transparent surface detection
- Player model culling
- Environment interaction

### Combat System Integration

#### Target Marking Integration
- Health-based target filtering
- Faction-based target selection
- Combat state awareness
- Damage feedback integration

#### Target Lock Integration
- Lock persistence across frames
- Target death handling
- Lock break conditions
- Combat UI integration

### UI System Integration

#### Crosshair System
- Dynamic crosshair behavior
- Target indication
- Accuracy feedback

#### Camera UI Overlays
- Reticle customization
- Scope overlays
- Effect visualization
- Status indicators

### Audio System Integration

#### Camera-Based Audio
- 3D positional audio
- Environmental audio zones
- Camera movement audio cues
- Effect synchronization

---

## Usage Patterns

### Basic Camera Setup

#### Player Camera Initialization
```rust
// Spawn camera with player tracking
let camera_entity = commands.spawn((
    Camera3d::default(),
    CameraController {
        follow_target: Some(player_entity),
        mode: CameraMode::ThirdPerson,
        distance: 4.0,
        ..default()
    },
    CameraState::default(),
    CameraTargetState::default(),
    CameraBobState::default(),
    Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
)).id();
```

### Advanced Camera Configuration

#### Custom Camera State
```rust
let custom_state = CameraStateInfo {
    name: "Combat Mode".to_string(),
    cam_position_offset: Vec3::new(0.0, 0.0, 0.0),
    pivot_position_offset: Vec3::new(0.5, 1.5, 0.0),
    initial_fov: 50.0,
    max_fov: 80.0,
    lean_enabled: false,
    ..default()
};

camera_controller.states.push(custom_state);
```

#### Zone-Based Camera Areas
```rust
// Create a stealth zone with different camera behavior
commands.spawn((
    Collider::cuboid(10.0, 5.0, 10.0),
    Transform::from_xyz(0.0, 2.5, 0.0),
    CameraZone {
        settings: CameraZoneSettings {
            mode: CameraMode::FirstPerson,
            distance: Some(1.0),
            fov: Some(70.0),
            pivot_offset: Some(Vec3::new(0.0, 1.8, 0.0)),
            transition_speed: 3.0,
            ..default()
        },
        priority: 10,
    },
));
```

#### Cinematic Camera Path
```rust
// Create waypoints for a cutscene
let mut waypoints = Vec::new();

// Waypoint 1: Wide shot
let wp1 = commands.spawn((
    Transform::from_xyz(10.0, 8.0, 10.0),
    CameraWaypoint {
        wait_time: 0.0,
        movement_speed: Some(2.0),
        rotation_speed: Some(5.0),
        rotation_mode: WaypointRotationMode::LookAtTarget,
        look_at_target: Some(player_entity),
        ..default()
    },
)).id();
waypoints.push(wp1);

// Waypoint 2: Close-up
let wp2 = commands.spawn((
    Transform::from_xyz(2.0, 2.0, 2.0),
    CameraWaypoint {
        wait_time: 3.0,
        movement_speed: Some(1.0),
        rotation_speed: Some(8.0),
        rotation_mode: WaypointRotationMode::LookAtTarget,
        look_at_target: Some(player_entity),
        ..default()
    },
)).id();
waypoints.push(wp2);

// Create the track
commands.spawn((
    CameraWaypointTrack {
        waypoints,
        loop_track: false,
    },
    CameraWaypointFollower {
        current_track: Some(track_entity),
        current_waypoint_index: 0,
        waiting_timer: 0.0,
        is_moving: false,
    },
));
```

### Camera Shake Triggers

#### Explosion Shake
```rust
// Queue a shake for an explosion
let shake_request = ShakeRequest {
    name: "Explosion".to_string(),
    intensity: 1.5,
    duration: Some(0.8),
};

commands.insert_resource(ShakeQueue(vec![shake_request]));
```

#### Point Shake Source
```rust
// Create a persistent shake source
commands.spawn((
    Transform::from_xyz(explosion_x, explosion_y, explosion_z),
    PointShake {
        name: "Grenade Explosion".to_string(),
        radius: 15.0,
        intensity: 2.0,
        active: true,
        shake_using_distance: true,
    },
));
```

### Target Lock Integration

#### Manual Target Selection
```rust
// Programmatic target locking
let mut camera_target_state = camera_target_state_query.get_mut(camera_entity).unwrap();
if let Some(target_entity) = find_target_in_crosshair() {
    camera_target_state.locked_target = Some(target_entity);
    camera_target_state.is_locking = true;
}
```

#### Target Lock Callbacks
```rust
// Listen for target lock events
fn on_target_locked(mut event_reader: EventReader<TargetLockEvent>) {
    for event in event_reader.read() {
        info!("Locked onto target: {:?}", event.target_entity);
        // Trigger UI updates, audio cues, etc.
    }
}
```

### Camera Effect Integration

#### Dynamic FOV Changes
```rust
// Scope aiming effect
let mut camera_state = camera_state_query.get_mut(camera_entity).unwrap();
camera_state.fov_override = Some(25.0); // Sniper scope
camera_state.fov_override_speed = Some(15.0);
```

#### Camera Transparency
```rust
// Enable camera transparency for nearby objects
commands.insert_resource(TransparencySettings {
    enabled: true,
    alpha_target: 0.3,
    fade_speed: 8.0,
    ray_radius: 0.3,
});
```

---

## Best Practices

### Performance Optimization

#### Spatial Query Efficiency
- Minimize spatial query frequency
- Use appropriate collision filters
- Cache spatial query results when possible
- Consider object pooling for frequent operations

```rust
// Efficient spatial query usage
let filter = SpatialQueryFilter::from_excluded_entities([player_entity]);
if let Some(hit) = spatial_query.cast_ray(start, direction, max_dist, true, &filter) {
    // Process collision
}
```

#### Component Update Optimization
- Use appropriate system scheduling
- Minimize component queries
- Avoid unnecessary allocations
- Use query filtering effectively

```rust
// Optimized query pattern
pub fn optimized_camera_update(
    time: Res<Time>,
    mut camera_query: Query<(&CameraController, &mut CameraState), With<MainCamera>>,
    // ... other filtered queries
) {
    // System implementation
}
```

### Code Organization

#### Modular Camera States
Organize camera behavior into logical state modules:

```rust
pub struct CameraStateModule {
    pub name: String,
    pub enter_fn: fn(&mut CameraController),
    pub update_fn: fn(&mut CameraController, &CameraState, f32),
    pub exit_fn: fn(&mut CameraController),
}
```

#### Component Separation
Keep camera concerns separate:

- **CameraController**: Configuration and behavior settings
- **CameraState**: Runtime state tracking
- **CameraEffects**: Visual and audio effects
- **CameraZones**: Area-based behavior

### State Management

#### Smooth Transitions
Always use smooth interpolation for camera movements:

```rust
let alpha = 1.0 - (-speed * delta_time).exp();
current_value = current_value + (target_value - current_value) * alpha;
```

#### State Persistence
Maintain camera state across scene transitions:

```rust
// Save camera state before level change
let saved_state = camera_controller.serialize();

// Restore after level load
camera_controller.deserialize(saved_state);
```

### Input Handling

#### Input Buffering
Implement input buffering for responsive camera controls:

```rust
pub struct CameraInputBuffer {
    pub look_input: Vec2,
    pub buffered_look: Vec2,
    pub input_buffer_time: f32,
}
```

#### Sensitivity Scaling
Scale sensitivity appropriately for different frame rates:

```rust
let frame_rate_multiplier = 60.0 / actual_frame_rate;
let scaled_sensitivity = base_sensitivity * frame_rate_multiplier;
```

### Collision and Physics

#### Collision Optimization
- Use appropriate collision layers
- Minimize raycast frequency
- Implement collision caching
- Use LOD for distant objects

```rust
// Optimized collision checking
let collision_mask = CollisionGroups::new(
    Group::GROUP_1, // Camera collision group
    Group::GROUP_2 | Group::GROUP_3, // Environment collision groups
);
```

#### Transparency Management
Implement efficient transparency sorting:

```rust
pub struct TransparencyManager {
    pub sorted_surfaces: Vec<TransparentSurface>,
    pub update_frequency: f32,
    pub last_update_time: f32,
}
```

### Memory Management

#### Component Pooling
Pool frequently created/destroyed camera components:

```rust
pub struct CameraComponentPool {
    pub shake_instances: Vec<CameraShakeInstance>,
    pub target_states: Vec<CameraTargetState>,
    pub available_entities: Vec<Entity>,
}
```

#### Resource Management
Properly manage camera resources:

```rust
impl Drop for CameraEffectManager {
    fn drop(&mut self) {
        // Cleanup effects, audio, resources
        self.cleanup_active_effects();
        self.stop_all_audio();
    }
}
```

### Testing and Debugging

#### Camera Debug Visualization
Implement debug visualization for camera state:

```rust
#[cfg(debug_assertions)]
pub fn debug_camera_state(
    gizmos: Gizmos,
    camera_query: Query<(&CameraController, &CameraState, &Transform)>,
) {
    for (controller, state, transform) in camera_query.iter() {
        // Draw pivot point
        gizmos.circle(state.current_pivot, 0.2, Color::YELLOW);
        
        // Draw camera frustum
        draw_camera_frustum(gizmos, transform, state.current_distance);
        
        // Draw target lock
        if let Some(target) = state.locked_target {
            draw_target_lock(gizmos, target, Color::RED);
        }
    }
}
```

#### Logging and Metrics
Implement comprehensive logging:

```rust
pub struct CameraMetrics {
    pub frame_time: f32,
    pub spatial_queries_per_frame: u32,
    pub collision_checks_per_frame: u32,
    pub active_zones: u32,
    pub active_effects: u32,
}
```

---

## Troubleshooting

### Common Issues and Solutions

#### Camera Jitter or Stuttering

**Symptoms:**
- Camera movement appears choppy or inconsistent
- Sudden jumps in camera position
- Frame-rate dependent behavior

**Possible Causes:**
- Insufficient smoothing speeds
- Conflicting system updates
- Spatial query performance issues
- Component synchronization problems

**Solutions:**
```rust
// Increase smoothing speeds
camera.smooth_follow_speed = 20.0;
camera.smooth_rotation_speed = 25.0;
camera.pivot_smooth_speed = 15.0;

// Check system ordering
.add_systems(Update, (
    update_camera_state_offsets,
    update_target_marking,
    update_target_lock,
).chain())

// Optimize spatial queries
let filter = SpatialQueryFilter::from_excluded_entities([player_entity]);
```

#### Camera Collision Issues

**Symptoms:**
- Camera clips through walls
- Collision detection doesn't work
- Camera gets stuck in objects

**Possible Causes:**
- Collision radius too small
- Spatial query filter issues
- Missing collision components
- Raycast direction problems

**Solutions:**
```rust
// Adjust collision settings
camera.use_collision = true;
camera.collision_radius = 0.3;

// Verify spatial query setup
let direction = transform.back(); // Ensure correct direction
let max_dist = state.current_distance;

// Check collision layers
let filter = SpatialQueryFilter::default();
// Ensure proper collision groups
```

#### Target Lock Problems

**Symptoms:**
- Target locking doesn't work
- Camera won't track locked targets
- Lock breaks unexpectedly

**Possible Causes:**
- Target distance too far
- FOV threshold too restrictive
- Missing health components
- Input event handling issues

**Solutions:**
```rust
// Adjust target lock settings
camera.target_lock.max_distance = 50.0;
camera.target_lock.fov_threshold = 60.0;
camera.target_lock.scan_radius = 3.0;

// Verify target components
for (entity, target_gt, health, name) in target_query.iter() {
    if health.current <= 0.0 { continue; } // Ensure health > 0
    // Check other required components
}
```

#### FOV Transition Issues

**Symptoms:**
- FOV changes too slowly
- FOV doesn't transition smoothly
- Override FOV doesn't work

**Possible Causes:**
- Low FOV transition speed
- Conflicting FOV overrides
- Projection component issues
- Frame rate dependency

**Solutions:**
```rust
// Increase FOV transition speed
camera.fov_speed = 15.0;

// Clear existing overrides
state.fov_override = None;

// Ensure projection component exists
if let Projection::Perspective(ref mut p) = *projection {
    // FOV updates should work correctly
}
```

#### Zone Transition Problems

**Symptoms:**
- Camera zones don't activate
- Zone transitions are jarring
- Multiple zones conflict

**Possible Causes:**
- Zone collision detection issues
- Priority system problems
- Spatial query filter conflicts
- Transition speed too low

**Solutions:**
```rust
// Check zone collision setup
commands.spawn((
    Collider::cuboid(10.0, 5.0, 10.0), // Ensure proper collider
    CameraZone { /* ... */ },
));

// Adjust priority and transition speed
CameraZone {
    priority: 10, // Higher priority
    settings: CameraZoneSettings {
        transition_speed: 10.0, // Faster transitions
        // ...
    }
}
```

#### Performance Issues

**Symptoms:**
- Low frame rates
- High CPU usage
- Stuttering during camera movement

**Possible Causes:**
- Too many spatial queries
- Inefficient component queries
- Unnecessary system updates
- Memory allocation issues

**Solutions:**
```rust
// Optimize queries
Query<(&CameraController, &mut CameraState), With<MainCamera>>

// Use query filtering
Query<&Transform, (With<CameraController>, Without<Player>)>

// Implement query caching
fn system_with_cached_query(
    cached_transforms: QueryCache<(&Transform, &CameraController)>,
    // ...
) {
    // Use cached results
}
```

### Debug Techniques

#### Camera State Debugging

**Enable Debug Visualization:**
```rust
#[cfg(debug_assertions)]
fn debug_camera_systems(
    mut commands: Commands,
    camera_query: Query<(&CameraController, &CameraState)>,
) {
    for (controller, state) in camera_query.iter() {
        println!("Camera Mode: {:?}", controller.mode);
        println!("Distance: {}", state.current_distance);
        println!("Pivot: {:?}", state.current_pivot);
        println!("Yaw: {}, Pitch: {}", state.yaw, state.pitch);
    }
}
```

#### Spatial Query Debugging

**Visualize Spatial Queries:**
```rust
fn debug_spatial_queries(
    spatial_query: SpatialQuery,
    gizmos: Gizmos,
    camera_query: Query<(&CameraState, &Transform)>,
) {
    for (state, transform) in camera_query.iter() {
        // Draw collision ray
        let direction = transform.back();
        gizmos.ray(
            state.current_pivot,
            direction * state.current_distance,
            Color::RED,
        );
    }
}
```

#### Performance Profiling

**Frame Timing Analysis:**
```rust
struct CameraProfiler {
    last_frame_time: Instant,
    frame_times: Vec<f32>,
}

fn profile_camera_performance(
    mut profiler: ResMut<CameraProfiler>,
    time: Res<Time>,
) {
    let frame_time = time.delta_secs();
    profiler.frame_times.push(frame_time);
    
    if profiler.frame_times.len() > 100 {
        let avg_frame_time: f32 = profiler.frame_times.iter().sum::<f32>() / 100.0;
        println!("Average camera frame time: {:.3}ms", avg_frame_time * 1000.0);
        profiler.frame_times.clear();
    }
}
```

### Integration Issues

#### Input System Integration

**Common Problems:**
- Input state not updating
- Sensitivity not working
- Input buffering issues

**Debug Steps:**
```rust
fn debug_input_integration(
    input: Res<InputState>,
    mut camera_query: Query<&mut CameraController>,
) {
    println!("Look input: {:?}", input.look);
    println!("Aim pressed: {}", input.aim_pressed);
    
    for mut controller in camera_query.iter_mut() {
        println!("3P Sensitivity: {}", controller.rot_sensitivity_3p);
        println!("1P Sensitivity: {}", controller.rot_sensitivity_1p);
    }
}
```

#### Physics Integration

**Common Problems:**
- Collision detection not working
- Spatial queries failing
- Raycast direction issues

**Debug Steps:**
```rust
fn debug_physics_integration(
    spatial_query: SpatialQuery,
    gizmos: Gizmos,
    camera_query: Query<(&CameraState, &Transform)>,
) {
    for (state, transform) in camera_query.iter() {
        // Visualize raycast
        let ray_origin = state.current_pivot;
        let ray_direction = transform.back();
        let ray_length = state.current_distance;
        
        gizmos.ray(ray_origin, ray_direction * ray_length, Color::BLUE);
        
        // Test actual collision
        if let Some(hit) = spatial_query.cast_ray(
            ray_origin,
            ray_direction,
            ray_length,
            true,
            &SpatialQueryFilter::default()
        ) {
            gizmos.circle(hit.point, 0.1, Color::GREEN);
        }
    }
}
```

### Advanced Troubleshooting

#### Memory Issues

**Detection:**
- Memory leaks in camera components
- Excessive allocations
- Component pool exhaustion

**Investigation:**
```rust
fn debug_memory_usage(
    entities: Res<Entities>,
    camera_shakes: Query<&CameraShakeInstance>,
) {
    let active_shakes = camera_shakes.iter().count();
    let total_entities = entities.iter().count();
    
    println!("Active camera shakes: {}", active_shakes);
    println!("Total entities: {}", total_entities);
    
    if active_shakes > 100 {
        warn!("High number of active camera shakes!");
    }
}
```

#### System Conflicts

**Detection:**
- Multiple systems updating same components
- Conflicting state changes
- Race conditions

**Investigation:**
```rust
fn debug_system_conflicts(
    system_order: Res<SystemExecutionOrder>,
) {
    for (system_name, order) in system_order.iter() {
        println!("System '{}' runs at order {}", system_name, order);
    }
}

// Check for conflicting updates
fn check_component_updates(
    camera_query: Query<&CameraController>,
    previous_states: Query<&CameraController>,
) {
    // Compare states to detect unexpected changes
}
```

### Performance Monitoring

#### Real-Time Metrics

**Implementation:**
```rust
struct CameraPerformanceMetrics {
    spatial_queries_per_frame: HashMap<String, u32>,
    component_updates_per_frame: HashMap<String, u32>,
    memory_usage_per_frame: f32,
}

fn collect_performance_metrics(
    mut metrics: ResMut<CameraPerformanceMetrics>,
    spatial_queries: Query<(&SpatialQuery, &CameraController)>,
) {
    metrics.spatial_queries_per_frame.insert(
        "spatial_query_count".to_string(),
        spatial_queries.iter().count() as u32,
    );
}
```

#### Historical Analysis

**Data Collection:**
```rust
struct CameraMetricsHistory {
    frame_time_history: Vec<f32>,
    spatial_query_history: Vec<u32>,
    max_history_size: usize,
}

fn update_metrics_history(
    mut history: ResMut<CameraMetricsHistory>,
    current_metrics: &CameraPerformanceMetrics,
) {
    history.frame_time_history.push(current_metrics.avg_frame_time);
    history.spatial_query_history.push(current_metrics.spatial_queries);
    
    if history.frame_time_history.len() > history.max_history_size {
        history.frame_time_history.remove(0);
    }
}
```

---

## Conclusion

The Camera System provides a comprehensive, modular solution for 3D game camera management in Bevy. With its extensive feature set, performance optimizations, and integration capabilities, it serves as a robust foundation for various game genres and camera requirements.

### Key Strengths

- **Modular Architecture**: Clear separation of concerns with plugin-based design
- **Performance Optimized**: Efficient spatial queries and smooth interpolation
- **Highly Configurable**: Extensive customization options for different game needs
- **Integration Ready**: Seamless integration with other Bevy systems
- **Developer Friendly**: Comprehensive debugging and profiling tools

### Future Enhancements

The system is designed for extensibility, with potential future additions including:

- **Advanced AI Integration**: Camera behavior driven by AI state
- **VR/AR Support**: Specialized camera modes for immersive technologies
- **Machine Learning**: Adaptive camera behavior based on player preferences
- **Cloud Synchronization**: Cross-platform camera state synchronization
- **Advanced Effects**: Particle systems and advanced post-processing integration

### Getting Started

1. **Basic Setup**: Add the Camera Plugin to your Bevy application
2. **Configure Controller**: Set up CameraController with desired parameters
3. **Add Components**: Include CameraState and related components
4. **Test Integration**: Verify input, physics, and character system integration
5. **Customize Features**: Enable specific camera features based on game needs

The Camera System documentation provides the foundation for implementing sophisticated camera behaviors in your Bevy-based game projects, with the flexibility to adapt to diverse gameplay requirements and performance constraints.

---

*This documentation covers the Camera System as of the latest version. For updates and additional resources, refer to the main project repository and related documentation.*
