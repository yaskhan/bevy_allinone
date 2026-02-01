# Stealth System - Comprehensive Documentation
 
## Table of Contents
 
1. [Overview](#overview)
2. [Core Components](#core-components)
   - [StealthController](#stealthcontroller)
   - [StealthState](#stealthstate)
   - [CoverDetection](#coverdetection)
   - [VisibilityMeter](#visibilitymeter)
3. [System Architecture](#system-architecture)
4. [Hide States](#hide-states)
   - [State Types](#state-types)
   - [State Transitions](#state-transitions)
5. [Cover System](#cover-system)
   - [Cover Detection](#cover-detection)
   - [Cover Types](#cover-types)
   - [Cover Mechanics](#cover-mechanics)
6. [Visibility & Detection](#visibility--detection)
   - [Line of Sight](#line-of-sight)
   - [Detection Levels](#detection-levels)
   - [Sound System](#sound-system)
7. [Camera System](#camera-system)
   - [Camera Controls](#camera-controls)
   - [Peek Mechanics](#peek-mechanics)
   - [Zoom System](#zoom-system)
8. [Advanced Features](#advanced-features)
   - [Corner Leaning](#corner-leaning)
   - [Spring Camera](#spring-camera)
   - [Fixed Place Hide](#fixed-place-hide)
9. [AI Integration](#ai-integration)
10. [Setup and Usage](#setup-and-usage)
11. [Performance Considerations](#performance-considerations)
12. [Customization Guide](#customization-guide)
 
## Overview
 
The Stealth System provides comprehensive mechanics for hiding, cover-based gameplay, and detection avoidance. It enables players to hide from enemies, take cover behind objects, peek around corners, and manage their visibility through multiple states and mechanics. The system seamlessly integrates with the Character Controller, AI System, and Camera System to create immersive stealth gameplay experiences.
 
### Key Features
 
- **Multiple Hide States**: Crouch hiding, prone hiding, peeking, corner leaning, and fixed-place hiding
- **Cover Detection**: Advanced raycasting system to detect and utilize cover objects
- **Visibility System**: Dynamic visibility meter tracking detection levels, sound, and light exposure
- **Camera Controls**: Specialized camera controls for peeking, looking around while hidden, and zooming
- **AI Detection**: Real-time line-of-sight checks with AI enemies and gradual detection mechanics
- **Spring Camera**: Optional auto-return camera behavior for intuitive controls
- **Sound Management**: Footstep noise levels affecting detection risk
- **Flexible Configuration**: Extensive customization options for all stealth mechanics
 
### Design Philosophy
 
The Stealth System follows a state-based approach where the player transitions between various hide states based on input, cover availability, and detection status. Each state has unique camera behaviors, movement restrictions, and detection properties, allowing for varied stealth gameplay styles from static hiding to dynamic cover-to-cover movement.
 
## Core Components
 
### StealthController
 
The main configuration component that defines stealth behavior and capabilities:
 
**Cover Detection Settings:**
 
| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `cover_detection_distance` | `f32` | `2.0` | Maximum distance for cover detection raycasts |
| `cover_detection_angle` | `f32` | `90.0` | Field of view angle for cover detection (degrees) |
| `cover_layer` | `u32` | `256` | Physics layer mask for cover objects (layer 8) |
**Hide Requirements:**
 
| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `character_need_to_crouch` | `bool` | `true` | Whether crouching is required to hide |
| `character_cant_move` | `bool` | `false` | Whether movement breaks hiding |
| `max_move_amount` | `f32` | `0.1` | Maximum movement allowed before breaking hide |
**Detection Settings:**
 
| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `hidden_for_a_time` | `bool` | `false` | Enable time-limited hiding |
| `hidden_for_a_time_amount` | `f32` | `5.0` | Maximum duration for timed hiding (seconds) |
| `time_delay_to_hide_again_if_discovered` | `f32` | `2.0` | Cooldown after being discovered (seconds) |
| `check_if_character_can_be_hidden_again_rate` | `f32` | `0.5` | Rate for checking hide eligibility (seconds) |
| `check_if_detected_while_hidden` | `bool` | `false` | Enable detection checks while hiding |
**Camera Rotation Settings:**
 
| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `camera_can_rotate` | `bool` | `true` | Enable camera rotation while hidden |
| `rotation_speed` | `f32` | `10.0` | Camera rotation speed multiplier |
| `range_angle_x` | `Vec2` | `(-90, 90)` | Vertical rotation limits (degrees) |
| `range_angle_y` | `Vec2` | `(-90, 90)` | Horizontal rotation limits (degrees) |
| `use_spring_rotation` | `bool` | `false` | Enable auto-return camera rotation |
| `spring_rotation_delay` | `f32` | `1.0` | Delay before camera auto-returns (seconds) |
| `smooth_camera_rotation_speed` | `f32` | `5.0` | Speed of smooth camera transitions |
**Camera Movement Settings:**
 
| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `camera_can_move` | `bool` | `true` | Enable camera translation while hidden |
| `move_camera_speed` | `f32` | `10.0` | Camera movement speed multiplier |
| `smooth_move_camera_speed` | `f32` | `5.0` | Speed of smooth camera position transitions |
| `move_camera_limits_x` | `Vec2` | `(-2, 2)` | Horizontal movement limits (units) |
| `move_camera_limits_y` | `Vec2` | `(-2, 2)` | Vertical movement limits (units) |
| `use_spring_movement` | `bool` | `false` | Enable auto-return camera movement |
| `spring_movement_delay` | `f32` | `1.0` | Delay before camera auto-returns (seconds) |
**Zoom Settings:**
 
| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `zoom_enabled` | `bool` | `false` | Enable zoom functionality |
| `zoom_speed` | `f32` | `10.0` | Zoom in/out speed |
| `max_zoom` | `f32` | `10.0` | Maximum zoom level (minimum FOV) |
| `min_zoom` | `f32` | `90.0` | Minimum zoom level (maximum FOV) |
| `set_hidden_fov` | `bool` | `false` | Override FOV when hidden |
| `hidden_fov` | `f32` | `20.0` | FOV value when hidden (if enabled) |
**Camera Reset Settings:**
 
| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `can_reset_camera_rotation` | `bool` | `true` | Allow manual camera rotation reset |
| `can_reset_camera_position` | `bool` | `true` | Allow manual camera position reset |
**UI Settings:**
 
| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `use_character_state_icon` | `bool` | `true` | Display state icon for visibility status |
| `visible_character_state_name` | `String` | `"Visible"` | Icon/text when visible |
| `not_visible_character_state_name` | `String` | `"Not Visible"` | Icon/text when hidden |
| `use_message_when_unable_to_hide` | `bool` | `false` | Show message when hiding is blocked |
| `unable_to_hide_message` | `String` | `""` | Custom message text |
| `show_message_time` | `f32` | `2.0` | Duration to display message (seconds) |
### StealthState
 
Runtime state component tracking the current stealth status:
 
**Hide State:**
 
| Field | Type | Description |
|-------|------|-------------|
| `is_hidden` | `bool` | Whether character is currently hidden |
| `can_be_hidden` | `bool` | Whether hiding is currently allowed |
| `is_detected` | `bool` | Whether character has been detected by AI |
| `last_time_hidden` | `f32` | Timestamp when last entered hiding |
| `last_time_discovered` | `f32` | Timestamp when last discovered |
| `last_time_check_if_can_be_hidden_again` | `f32` | Timestamp for eligibility checks |
| `hidden_time` | `f32` | Duration of current hiding session |
**Hide State Enum:**
 
| Field | Type | Description |
|-------|------|-------------|
| `hide_state` | `HideState` | Current hide state (see [Hide States](#hide-states)) |
| `is_peeking` | `bool` | Whether currently in peek state |
| `is_corner_leaning` | `bool` | Whether currently leaning around corner |
**Camera State:**
 
| Field | Type | Description |
|-------|------|-------------|
| `camera_is_free` | `bool` | Whether camera is in free-look mode |
| `current_look_angle` | `Vec2` | Current camera look angles (pitch, yaw) |
| `current_camera_rotation` | `Quat` | Current camera rotation quaternion |
| `current_pivot_rotation` | `Quat` | Current pivot rotation quaternion |
| `current_move_camera_position` | `Vec3` | Current camera translation offset |
| `current_camera_movement_position` | `Vec3` | Calculated camera movement position |
| `current_fov_value` | `f32` | Current field of view value |
**Input State:**
 
| Field | Type | Description |
|-------|------|-------------|
| `horizontal_mouse` | `f32` | Horizontal mouse input value |
| `vertical_mouse` | `f32` | Vertical mouse input value |
| `horizontal_input` | `f32` | Horizontal movement input value |
| `vertical_input` | `f32` | Vertical movement input value |
**Zoom State:**
 
| Field | Type | Description |
|-------|------|-------------|
| `increase_zoom` | `bool` | Zoom in flag |
| `decrease_zoom` | `bool` | Zoom out flag |
| `last_time_mouse_wheel_used` | `f32` | Timestamp of last zoom input |
| `mouse_wheel_used_previously` | `bool` | Whether zoom was recently used |
**Spring Camera State:**
 
| Field | Type | Description |
|-------|------|-------------|
| `last_time_spring_rotation` | `f32` | Timestamp for spring rotation timer |
| `last_time_spring_movement` | `f32` | Timestamp for spring movement timer |
### CoverDetection
 
Component managing cover detection and current cover status:
 
| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `cover_objects` | `Vec<CoverObject>` | `[]` | List of detected cover objects |
| `is_in_cover` | `bool` | `false` | Whether character is currently in cover |
| `current_cover` | `Option<Entity>` | `None` | Entity of the current cover object |
| `cover_direction` | `Vec3` | `Vec3::ZERO` | Direction vector to cover |
| `cover_normal` | `Vec3` | `Vec3::ZERO` | Surface normal of cover |
| `cover_height` | `f32` | `0.0` | Height of cover relative to character |
| `cover_type` | `CoverType` | `Low` | Type of current cover |
**CoverObject Struct:**
 
| Field | Type | Description |
|-------|------|-------------|
| `entity` | `Entity` | Entity reference |
| `position` | `Vec3` | World position of cover |
| `normal` | `Vec3` | Surface normal |
| `height` | `f32` | Cover height |
| `cover_type` | `CoverType` | Type classification |
| `is_corner` | `bool` | Whether cover is a corner |
**CoverType Enum:**
- `Low` - Waist-high cover (height < 0.5)
- `Medium` - Chest-high cover (0.5 <= height < 1.5)
- `High` - Full cover (height >= 1.5)
- `Corner` - Corner cover for leaning
- `Full` - Complete concealment
 
### VisibilityMeter
 
Component tracking player visibility and detection status:
 
| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `current_visibility` | `f32` | `0.0` | Current visibility level (0.0 = hidden, 1.0 = visible) |
| `detection_level` | `f32` | `0.0` | Detection progress (0.0 = undetected, 1.0 = detected) |
| `sound_level` | `f32` | `0.0` | Current noise level (0.0 = silent, 1.0 = loud) |
| `light_level` | `f32` | `0.0` | Current light exposure (0.0 = dark, 1.0 = bright) |
| `visibility_decay_rate` | `f32` | `0.5` | Speed of detection decay per second |
| `detection_increase_rate` | `f32` | `0.3` | Speed of detection increase per second |
| `sound_decay_rate` | `f32` | `0.2` | Speed of sound decay per second |
| `is_visible_to_ai` | `bool` | `false` | Whether AI can see character |
| `is_detected_by_ai` | `bool` | `false` | Whether AI has detected character |
## System Architecture
 
The Stealth System uses a dual-timestep approach with systems running in both `Update` and `FixedUpdate`:
 
### Update Systems (Frame-rate dependent)
 
**System Execution Chain:**
1. `handle_stealth_input` - Processes player input for hide, peek, lean, zoom, and camera controls
2. `update_stealth_state` - Updates camera rotation, movement, zoom, and spring behaviors
3. `update_visibility_meter` - Updates visibility, detection, and sound levels
 
### FixedUpdate Systems (Physics timestep)
 
**System Execution:**
1. `detect_cover_objects` - Performs raycasts to detect nearby cover objects
2. `check_line_of_sight` - Checks AI line-of-sight and updates detection status
3. `update_hide_states` - Validates hide state requirements and enforces rules
 
### System Dependencies
 
The Stealth System integrates with:
- **Character Controller**: For movement state (crouching, sprinting) and pose management
- **AI System**: For enemy detection ranges and line-of-sight checks
- **Input System**: For capturing hide, peek, lean, and camera control inputs
- **Camera System**: For applying camera transformations during hiding
- **Physics System (Avian3D)**: For raycasting and spatial queries
 
## Hide States
 
### State Types
 
The system supports five distinct hide states, each with unique behaviors:
 
**HideState Enum:**
 
```
Visible         - Default state, not hiding
CrouchHide      - Hiding while crouched
ProneHide       - Hiding while prone
Peek            - Peeking from cover
CornerLean      - Leaning around corner
FixedPlaceHide  - Hiding at fixed location
```
 
**State Characteristics:**
 
| State | Movement | Camera Freedom | Detection Risk | Crouch Required |
|-------|----------|----------------|----------------|-----------------|
| Visible | Full | Normal | High | No |
| CrouchHide | Limited | Free-look | Low | Yes |
| ProneHide | Minimal | Free-look | Very Low | Yes |
| Peek | None | Limited | Medium | Yes |
| CornerLean | None | Limited | Medium | Optional |
| FixedPlaceHide | None | Free-look | Low | Optional |
### State Transitions
 
**Transition Rules:**
 
```
Visible → CrouchHide
  - Press Hide action
  - character_need_to_crouch must be true
  - Not currently detected
  - Can be hidden is true
CrouchHide → Peek
  - Press Peek action while hiding
  - In cover
  
CrouchHide → CornerLean
  - Press Corner Lean action while hiding
  - At corner cover
Peek/CornerLean → CrouchHide
  - Release Peek/Corner Lean action
Any Hidden State → Visible
  - Press Hide action (toggle off)
  - Detected by AI
  - Move too much (if character_cant_move is true)
  - Stand up (if character_need_to_crouch is true)
  - Hide time limit exceeded (if hidden_for_a_time is true)
```
 
**State Transition Diagram:**
 
```
┌─────────┐
│ Visible │
└────┬────┘
     │ Hide pressed
     ▼
┌──────────────┐
│ CrouchHide   │◄────────┐
└──┬───────┬───┘         │
   │       │             │
   │ Peek  │ Corner Lean │ Release/Detected
   │       │             │
   ▼       ▼             │
┌──────┐ ┌──────────────┐│
│ Peek │ │ CornerLean   ││
└──────┘ └──────────────┘│
   │           │         │
   └───────────┴─────────┘
```
 
## Cover System
 
### Cover Detection
 
The cover detection system uses raycasting to identify nearby cover objects:
 
**Detection Process:**
1. Raycast forward from character position
2. Maximum distance: `cover_detection_distance`
3. Filter by physics layer: `cover_layer`
4. Calculate angle between character forward and hit direction
5. Accept if angle <= `cover_detection_angle`
6. Classify cover type based on height
7. Update cover information in `CoverDetection` component
 
**Raycast Configuration:**
- **Origin**: Character's world position
- **Direction**: Character's forward vector
- **Max Distance**: Configurable (default 2.0 units)
- **Layer Mask**: Cover-specific physics layer
- **Collision Type**: First hit only
 
### Cover Types
 
Cover is automatically classified based on height relative to character:
 
**Height-Based Classification:**
 
| Cover Type | Height Range | Description | Typical Objects |
|------------|--------------|-------------|-----------------|
| Low | < 0.5m | Waist-high | Crates, low walls, sandbags |
| Medium | 0.5m - 1.5m | Chest-high | Barrels, counters, half-walls |
| High | >= 1.5m | Full body | Walls, columns, large objects |
| Corner | Variable | Corner edges | Building corners, wall ends |
| Full | Variable | Complete concealment | Closets, alcoves |
**Cover Properties:**
 
Each cover type provides different gameplay characteristics:
 
**Low Cover:**
- Requires crouching
- Allows peeking over top
- Moderate protection
- Easy to spot from elevated positions
 
**Medium Cover:**
- Can hide standing
- Allows peeking over top
- Good protection
- Balanced visibility/protection
 
**High Cover:**
- Full standing protection
- Requires corner leaning to see
- Maximum protection
- Limited visibility
 
**Corner Cover:**
- Special lean mechanics
- Directional exposure
- Tactical positioning
- Quick peek capability
 
### Cover Mechanics
 
**Entering Cover:**
1. Character approaches cover object
2. System detects cover via raycast
3. `is_in_cover` flag set to true
4. Cover information stored in `CoverDetection`
5. Character can activate hiding
 
**Using Cover:**
- While in cover and hidden:
  - Camera controls enabled
  - Movement restricted based on settings
  - Detection risk reduced
  - Peek and lean actions available
 
**Leaving Cover:**
- Move away from cover object
- Stand up (if crouching required)
- Distance exceeds detection range
- Cover object destroyed/moved
 
**Cover-Based Detection:**
- Being in cover doesn't guarantee safety
- AI can detect if:
  - Character moves too much
  - Character peeks/leans
  - AI flanks to different position
  - Line of sight is established
 
## Visibility & Detection
 
### Line of Sight
 
The system performs real-time line-of-sight checks between player and AI enemies:
 
**Line of Sight Algorithm:**
 
```
For each AI enemy:
  1. Calculate distance between character and AI
  2. Check if distance <= AI detection range
  3. Calculate direction vector from character to AI
  4. Perform raycast from character to AI
  5. If raycast hits nothing (clear path):
     - Character is visible to AI
     - Set detection flag
     - Start detection timer
  6. If raycast hits obstacle:
     - Character has cover
     - No detection from this AI
```
 
**Detection Validation:**
- Must be within AI's detection range
- Must have clear line of sight (no obstacles)
- Hide state affects detection probability
- Cover type influences visibility
 
**Multiple AI Handling:**
- Checks against all AI entities
- Any single AI can trigger detection
- Detection is global (one AI detecting alerts others)
- System exits early on first detection for performance
 
### Detection Levels
 
The visibility meter tracks gradual detection using multiple factors:
 
**Detection Calculation:**
 
**Visibility Level (0.0 - 1.0):**
- `0.0` - Fully hidden, invisible to AI
- `1.0` - Fully visible, easily detected
- Updated based on hide state
 
**Detection Progress (0.0 - 1.0):**
- `0.0` - Undetected, safe
- `1.0` - Fully detected, AI alerted
- Increases at `detection_increase_rate` when visible
- Decreases at `visibility_decay_rate` when hidden
 
**Detection States:**
 
| Detection Range | Status | AI Behavior | Player Feedback |
|----------------|--------|-------------|-----------------|
| 0.0 - 0.25 | Safe | Normal patrol | Green indicator |
| 0.25 - 0.5 | Suspicious | Investigate | Yellow indicator |
| 0.5 - 0.75 | Alert | Search actively | Orange indicator |
| 0.75 - 1.0 | Detected | Chase/Attack | Red indicator |
**Detection Factors:**
 
The system considers multiple factors for detection:
 
1. **Visibility**: Current hide state
2. **Sound**: Movement noise level
3. **Light**: Environmental lighting (placeholder)
4. **Distance**: Proximity to AI
5. **Cover**: Cover type and quality
 
### Sound System
 
Movement generates noise that affects detection:
 
**Sound Level Calculation:**
 
| Movement State | Sound Level | Detection Modifier |
|---------------|-------------|--------------------|
| Stationary | 0.0 | No detection bonus |
| Walking | 0.3 | +30% detection range |
| Running | 0.7 | +70% detection range |
| Sprinting | 1.0 | +100% detection range |
**Sound Decay:**
- Sound level decreases over time
- Decay rate: `sound_decay_rate` per second (default 0.2)
- Gradual reduction simulates sound dissipation
- AI may investigate last known sound location
 
**Sound-Based Detection:**
- High sound levels increase AI awareness
- AI may enter suspicious state from sound alone
- Combined with visibility for total detection risk
- Movement noise persists briefly after stopping
 
## Camera System
 
### Camera Controls
 
While hidden, the camera enters a specialized free-look mode:
 
**Free-Look Activation:**
- Triggered when entering hide state
- Camera detaches from normal character follow
- Mouse/controller input controls camera directly
- Character body remains stationary
 
**Camera Rotation:**
 
**Horizontal Rotation (Yaw):**
- Input: Mouse X-axis or right stick X
- Speed: `rotation_speed` multiplier
- Range: `range_angle_y` (min, max degrees)
- Smooth: `smooth_camera_rotation_speed`
 
**Vertical Rotation (Pitch):**
- Input: Mouse Y-axis or right stick Y
- Speed: `rotation_speed` multiplier
- Range: `range_angle_x` (min, max degrees)
- Smooth: `smooth_camera_rotation_speed`
 
**Rotation Implementation:**
```
Horizontal angle += mouse_x * rotation_speed
Vertical angle -= mouse_y * rotation_speed
Horizontal clamped to [range_angle_y.min, range_angle_y.max]
Vertical clamped to [range_angle_x.min, range_angle_x.max]
Camera rotation = Quat from Euler angles (vertical, 0, 0)
Pivot rotation = Quat from Euler angles (0, horizontal, 0)
```
 
**Camera Translation:**
 
**Horizontal Movement:**
- Input: WASD or left stick
- Speed: `move_camera_speed` multiplier
- Range: `move_camera_limits_x` (min, max units)
 
**Vertical Movement:**
- Input: WASD or left stick
- Speed: `move_camera_speed` multiplier
- Range: `move_camera_limits_y` (min, max units)
 
**Camera Reset:**
- Manual reset via input action
- Returns camera to neutral position
- Resets rotation and translation
- Immediate or smooth transition
 
### Peek Mechanics
 
Peeking allows limited exposure to gain visibility:
 
**Peek Activation:**
1. Must be in hide state
2. Press Peek action
3. State transitions to `HideState::Peek`
4. Camera behavior changes
 
**Peek Camera Behavior:**
- More restrictive rotation limits
- Reduced movement range
- Over-shoulder view perspective
- Character slightly emerges from cover
 
**Peek Detection Risk:**
- Higher visibility than full hide
- Reduced compared to being fully visible
- AI can detect if looking directly
- Brief peeks minimize risk
 
**Peek Duration:**
- Can peek indefinitely
- Detection risk accumulates over time
- Best used for quick glances
- Release to return to full hide
 
### Zoom System
 
Optional zoom functionality for enhanced visibility:
 
**Zoom Controls:**
- **Zoom In**: Decrease FOV
- **Zoom Out**: Increase FOV
- Input: Mouse wheel or shoulder buttons
 
**Zoom Parameters:**
- **Max Zoom**: Minimum FOV (default 10°)
- **Min Zoom**: Maximum FOV (default 90°)
- **Zoom Speed**: FOV change rate (default 10°/sec)
 
**Zoom Implementation:**
```
If increase_zoom:
    current_fov -= delta_time * zoom_speed
If decrease_zoom:
    current_fov += delta_time * zoom_speed
current_fov clamped to [max_zoom, min_zoom]
```
 
**Zoom Features:**
- Smooth transitions
- Configurable speed
- Input debouncing (0.1s)
- FOV override option for hide state
 
## Advanced Features
 
### Corner Leaning
 
Corner leaning provides tactical positioning at corners:
 
**Lean Activation:**
1. Must be in hide state
2. Must be at corner cover
3. Press Corner Lean action
4. State transitions to `HideState::CornerLean`
 
**Lean Mechanics:**
- Character shifts laterally
- Partial body exposure
- Maintains cover protection
- Camera adjusts to lean direction
 
**Lean Types:**
- **Left Lean**: Expose left side
- **Right Lean**: Expose right side
- **High Lean**: Peek over top
- **Low Lean**: Peek around bottom
 
**Tactical Advantages:**
- Minimal exposure
- Quick return to cover
- Precise positioning
- Fire from cover capability
 
### Spring Camera
 
Automatic camera return for intuitive controls:
 
**Spring Rotation:**
- After `spring_rotation_delay` seconds of no input
- Camera automatically returns to neutral rotation
- Smooth interpolation
- Can be disabled
 
**Spring Movement:**
- After `spring_movement_delay` seconds of no input
- Camera automatically returns to neutral position
- Smooth interpolation
- Can be disabled
 
**Spring Behavior:**
```
If no rotation input for > spring_rotation_delay:
    Interpolate camera rotation to identity
    Interpolate pivot rotation to identity
    
If no movement input for > spring_movement_delay:
    Interpolate camera position to zero offset
```
 
**Use Cases:**
- Casual exploration
- Reduces player fatigue
- Natural camera behavior
- Optional feature for hardcore mode
 
### Fixed Place Hide
 
Specialized hiding at designated locations:
 
**Concept:**
- Predefined hide spots in level
- Character enters specific animation/pose
- Fixed camera perspective
- Complete hiding functionality
 
**Implementation:**
- Hide spot entities marked with components
- Character snaps to hide position
- Custom camera settings per hide spot
- Exit triggers return to normal gameplay
 
**Examples:**
- Closets
- Under tables
- In vents
- Behind curtains
- In haystacks
 
**Benefits:**
- Guaranteed hiding
- Cinematic presentation
- Level design opportunities
- Clear player feedback
 
## AI Integration
 
The Stealth System tightly integrates with the AI System:
 
**AI Detection Range:**
- Queries AI `detection_range` field
- Only checks AI within range
- Distance-based detection probability
- Gradual awareness system
 
**AI Behavior Triggers:**
- Hidden state → AI ignores player
- Detected state → AI enters alert
- Sound events → AI investigates
- Line of sight breaks → AI searches
 
**Faction Integration:**
- Checks faction relations
- Only hostile factions trigger detection
- Neutral/friendly AI ignore stealth state
- Dynamic faction changes supported
 
**Alert System:**
- Detection alerts nearby AI
- Alert radius configurable
- Faction-wide awareness
- Coordinated search patterns
 
**AI State Changes on Detection:**
```
Player Detected:
    AI State: Idle/Patrol → Suspect → Chase
    AI Speed: patrol_speed → chase_speed
    AI Behavior: Passive → Aggressive
    AI Memory: Stores last known position
```
 
**Stealth Recovery:**
- After `time_delay_to_hide_again_if_discovered`
- Player can attempt hiding again
- AI may remain in search mode
- Requires breaking line of sight
 
## Setup and Usage
 
### Basic Stealth Setup
 
**Minimal Configuration:**
 
```rust
use bevy::prelude::*;
use bevy_allinone::prelude::*;
fn spawn_stealth_character(mut commands: Commands, position: Vec3) -> Entity {
    commands.spawn((
        Name::new("Stealth Character"),
        
        // Stealth components
        StealthController::default(),
        StealthState::default(),
        CoverDetection::default(),
        VisibilityMeter::default(),
        
        // Character controller
        CharacterController::default(),
        CharacterMovementState::default(),
        
        // Input
        InputState::default(),
        
        // Transform
        Transform::from_translation(position),
        GlobalTransform::default(),
        
        // Physics
        RigidBody::Dynamic,
        Collider::capsule(0.4, 1.0),
        LockedAxes::ROTATION_LOCKED,
    ))
    .id()
}
```
 
### Custom Stealth Configuration
 
**Advanced Setup:**
 
```rust
fn spawn_custom_stealth_character(mut commands: Commands) -> Entity {
    commands.spawn((
        Name::new("Custom Stealth Character"),
        
        StealthController {
            // Cover detection
            cover_detection_distance: 3.0,
            cover_detection_angle: 120.0,
            
            // Hide requirements
            character_need_to_crouch: true,
            character_cant_move: true,
            max_move_amount: 0.05,
            
            // Detection
            check_if_detected_while_hidden: true,
            time_delay_to_hide_again_if_discovered: 3.0,
            
            // Camera rotation
            camera_can_rotate: true,
            rotation_speed: 15.0,
            range_angle_x: Vec2::new(-60.0, 60.0),
            range_angle_y: Vec2::new(-90.0, 90.0),
            use_spring_rotation: true,
            spring_rotation_delay: 2.0,
            
            // Camera movement
            camera_can_move: true,
            move_camera_speed: 8.0,
            move_camera_limits_x: Vec2::new(-1.5, 1.5),
            move_camera_limits_y: Vec2::new(-1.5, 1.5),
            use_spring_movement: true,
            spring_movement_delay: 2.0,
            
            // Zoom
            zoom_enabled: true,
            zoom_speed: 20.0,
            max_zoom: 15.0,
            min_zoom: 75.0,
            
            // UI
            use_character_state_icon: true,
            use_message_when_unable_to_hide: true,
            unable_to_hide_message: "Cannot hide while being watched!".to_string(),
            
            ..default()
        },
        
        StealthState::default(),
        CoverDetection::default(),
        
        VisibilityMeter {
            visibility_decay_rate: 0.8,
            detection_increase_rate: 0.5,
            sound_decay_rate: 0.3,
            ..default()
        },
        
        // ... other components
    ))
    .id()
}
```
 
### Cover Object Setup
 
**Creating Cover Objects:**
 
```rust
fn spawn_cover_object(
    mut commands: Commands,
    position: Vec3,
    cover_type: CoverType,
) -> Entity {
    let height = match cover_type {
        CoverType::Low => 0.5,
        CoverType::Medium => 1.0,
        CoverType::High => 2.0,
        _ => 1.0,
    };
    
    commands.spawn((
        Name::new(format!("{:?} Cover", cover_type)),
        Transform::from_translation(position),
        GlobalTransform::default(),
        
        // Physics - make sure it's on the cover layer
        RigidBody::Static,
        Collider::cuboid(1.0, height, 1.0),
        CollisionLayers::new([Layer::Cover], [Layer::All]),
    ))
    .id()
}
```
 
### Input Actions
 
**Required Input Actions:**
 
The stealth system requires the following input actions to be mapped:
 
- `InputAction::Hide` - Toggle hiding
- `InputAction::Peek` - Toggle peeking
- `InputAction::CornerLean` - Toggle corner leaning
- `InputAction::ResetCamera` - Reset camera to neutral
- `InputAction::ZoomIn` - Zoom in (decrease FOV)
- `InputAction::ZoomOut` - Zoom out (increase FOV)
 
**Example Input Mapping:**
 
```rust
// This is typically configured in your input system
let input_mappings = vec![
    (InputAction::Hide, KeyCode::C),
    (InputAction::Peek, KeyCode::Q),
    (InputAction::CornerLean, KeyCode::E),
    (InputAction::ResetCamera, KeyCode::R),
    (InputAction::ZoomIn, MouseScrollUp),
    (InputAction::ZoomOut, MouseScrollDown),
];
```
 
## Performance Considerations
 
### Optimization Techniques
 
**Raycast Optimization:**
- Raycasts only fire when hidden
- Limited to `cover_detection_distance`
- Single raycast per character per frame
- Early exit on first hit
 
**Line of Sight Optimization:**
- Only checks when `check_if_detected_while_hidden` enabled
- Distance culling against AI detection range
- Early exit on first detecting AI
- Spatial partitioning for AI queries
 
**Update Frequency:**
- Cover detection runs in FixedUpdate (physics timestep)
- Camera updates run in Update (frame-rate)
- Visibility meter updates run in Update
- State validation runs in FixedUpdate
 
**Component Organization:**
- Data organized for cache efficiency
- Minimal state duplication
- Optional components for unused features
- Separate systems for independent features
 
### Performance Tips
 
1. **Disable Unused Features:**
   - Set `check_if_detected_while_hidden` to false if not needed
   - Disable zoom if not required
   - Disable spring camera if not desired
 
2. **Adjust Detection Frequency:**
   - Increase `check_if_character_can_be_hidden_again_rate`
   - Reduce update frequency for background characters
 
3. **Optimize Cover Layer:**
   - Use dedicated physics layer for cover
   - Minimize objects on cover layer
   - Use simple collision shapes
 
4. **Limit AI Count:**
   - Reduce active AI entities
   - Use frustum culling for distant AI
   - Implement AI LOD system
 
5. **Spatial Partitioning:**
   - Use Bevy's spatial queries efficiently
   - Implement chunking for large worlds
   - Cull checks outside relevant zones
 
## Customization Guide
 
### Creating Custom Hide States
 
**Extending HideState Enum:**
 
```rust
// Add new state to enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
pub enum HideState {
    // ... existing states ...
    Crawling,        // New: crawling hide state
    VentHiding,      // New: hiding in vents
    WaterHiding,     // New: hiding underwater
}
```
 
**Implementing Custom State Logic:**
 
```rust
fn update_custom_hide_states(
    mut query: Query
) {
    for (stealth, mut state, transform) in query.iter_mut() {
        match state.hide_state {
            HideState::Crawling => {
                // Custom crawling logic
                state.camera_is_free = false;
                // Implement crawling movement restrictions
            }
            HideState::VentHiding => {
                // Custom vent hiding logic
                state.camera_is_free = true;
                // Implement vent-specific mechanics
            }
            HideState::WaterHiding => {
                // Custom water hiding logic
                // Implement oxygen system, bubble detection, etc.
            }
            _ => {}
        }
    }
}
```
 
### Custom Cover Types
 
**Adding Specialized Cover:**
 
```rust
// Extend CoverType enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
pub enum CoverType {
    // ... existing types ...
    Foliage,         // Bushes, grass
    Water,           // Puddles, ponds
    Crowd,           // Blend into groups
    Shadow,          // Dark areas
    Dynamic,         // Moving cover (vehicles, etc.)
}
```
 
**Custom Cover Detection:**
 
```rust
fn detect_custom_cover_types(
    mut query: Query,
    foliage_query: Query>,
) {
    for (stealth, mut cover, transform) in query.iter_mut() {
        // Check proximity to foliage
        for foliage_transform in foliage_query.iter() {
            let distance = (foliage_transform.translation - transform.translation).length();
            
            if distance < 1.0 {
                cover.is_in_cover = true;
                cover.cover_type = CoverType::Foliage;
                cover.cover_height = foliage_transform.scale.y;
                break;
            }
        }
    }
}
```
 
### Custom Detection Factors
 
**Environmental Detection Modifiers:**
 
```rust
#[derive(Component)]
pub struct EnvironmentalDetection {
    pub weather_modifier: f32,      // Rain/fog reduces detection
    pub time_of_day_modifier: f32,  // Night reduces detection
    pub noise_level: f32,           // Ambient noise masks footsteps
}
fn update_environmental_detection(
    mut query: Query,
    environment: Res,
) {
    for (mut visibility, env_detection, transform) in query.iter_mut() {
        // Weather modifier
        let weather_factor = match environment.weather {
            Weather::Rain | Weather::Fog => 0.5,
            Weather::Clear => 1.0,
            _ => 0.8,
        };
        
        // Time of day modifier
        let time_factor = if environment.is_night() { 0.3 } else { 1.0 };
        
        // Calculate final detection modifier
        let total_modifier = weather_factor * time_factor * env_detection.noise_level;
        
        // Apply to visibility
        visibility.current_visibility *= total_modifier;
    }
}
```
 
### Custom Camera Behaviors
 
**Implementing New Camera Modes:**
 
```rust
#[derive(Component)]
pub struct CustomCameraMode {
    pub mode: CameraMode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CameraMode {
    OverShoulder,
    FirstPerson,
    TopDown,
    Isometric,
}
fn update_custom_camera_modes(
    mut query: Query,
    camera_query: Query, Without)>,
) {
    for (state, camera_mode, character_transform) in query.iter() {
        if !state.is_hidden {
            continue;
        }
        
        for mut camera_transform in camera_query.iter_mut() {
            match camera_mode.mode {
                CameraMode::OverShoulder => {
                    // Implement over-shoulder camera
                    let offset = Vec3::new(0.5, 0.5, -2.0);
                    camera_transform.translation = character_transform.translation + offset;
                }
                CameraMode::FirstPerson => {
                    // Implement first-person camera
                    let offset = Vec3::new(0.0, 1.7, 0.0);
                    camera_transform.translation = character_transform.translation + offset;
                }
                CameraMode::TopDown => {
                    // Implement top-down camera
                    let offset = Vec3::new(0.0, 10.0, 0.0);
                    camera_transform.translation = character_transform.translation + offset;
                    camera_transform.rotation = Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2);
                }
                _ => {}
            }
        }
    }
}
```
 
## Best Practices
 
### Design Guidelines
 
1. **Cover Placement:**
   - Place cover at strategic locations
   - Provide multiple cover options
   - Create cover-to-cover paths
   - Balance risk and reward
 
2. **Detection Tuning:**
   - Start with generous detection times
   - Test with real players
   - Balance challenge with fairness
   - Provide clear feedback
 
3. **Camera Feel:**
   - Smooth camera transitions
   - Comfortable rotation speeds
   - Reasonable angle limits
   - Optional spring camera for accessibility
 
4. **Level Design:**
   - Design with stealth in mind
   - Provide sightline breaks
   - Create hiding opportunities
   - Support multiple playstyles
 
### Common Patterns
 
**Stealth Sequences:**
```rust
// Patrol guards → Find cover → Wait → Move to next cover → Reach objective
```
 
**Detection Escalation:**
```rust
// Suspicious → Investigating → Alert → Combat → Search → Return to patrol
```
 
**Cover Progression:**
```rust
// Safe area → Open space with cover → Guarded area → Objective
```
 
### Debugging Tips
 
**Visualization:**
- Enable debug gizmos for cover detection
- Draw detection ranges
- Show line-of-sight rays
- Display state information
 
**Logging:**
- Log state transitions
- Track detection events
- Monitor visibility levels
- Record AI awareness
 
**Testing:**
- Test all hide states
- Verify cover detection
- Check AI integration
- Validate edge cases
 
## Troubleshooting
 
### Common Issues
 
**Character Can't Hide:**
- Check `can_be_hidden` flag
- Verify crouch requirement met
- Ensure not detected
- Check cooldown timer
 
**Cover Not Detected:**
- Verify cover object on correct physics layer
- Check `cover_detection_distance`
- Ensure cover object has collider
- Verify `cover_layer` mask
 
**Camera Not Moving:**
- Check `camera_can_rotate` flag
- Verify `camera_is_free` is true
- Ensure in hide state
- Check input mapping
 
**AI Detects Through Walls:**
- Verify wall colliders exist
- Check raycast layer masks
- Ensure obstacles on correct layers
- Test line-of-sight algorithm
 
**Spring Camera Not Working:**
- Enable `use_spring_rotation`/`use_spring_movement`
- Check delay timers
- Verify last input timestamps
- Test with longer delays
 
### Performance Issues
 
**High Raycast Cost:**
- Reduce cover detection distance
- Increase FixedUpdate timestep
- Limit active stealth characters
- Optimize cover object count
 
**AI Detection Lag:**
- Reduce AI query range
- Implement spatial partitioning
- Limit active AI count
- Use frustum culling
 
## Advanced Topics
 
### Multiplayer Stealth
 
Considerations for networked stealth gameplay:
 
**Client-Server Architecture:**
- Server authoritative detection
- Client prediction for hiding
- Reconciliation for state mismatches
- Bandwidth optimization
 
**Synchronization:**
- Replicate hide state
- Sync visibility meter
- Network camera transforms
- Handle latency compensation
 
### Animation Integration
 
Connecting stealth to animation system:
 
**Animation States:**
- Idle hidden animation
- Peek animation
- Corner lean animations
- Transition animations
 
**Animation Blending:**
- Blend between states
- Layer animations for peeking
- IK for wall contact
- Procedural lean adjustments
 
### AI Cooperation
 
Advanced AI behaviors for stealth:
 
**Alert Sharing:**
- Nearby AI alerted on detection
- Radio communication simulation
- Investigation patterns
- Search coordination
 
**Smart Searching:**
- Last known position tracking
- Predicted player position
- Cover checking
- Systematic area search
 
## Future Enhancements
 
Potential areas for expansion:
 
- **Light & Shadow System**: Dynamic lighting affecting detection
- **Noise Propagation**: Realistic sound travel through environment
- **Disguise System**: Blending in with NPCs
- **Environmental Kills**: Takedowns from hiding
- **Distraction Items**: Throwable objects to divert attention
- **Smell Detection**: AI tracking via scent
- **Crowd Blending**: Hiding in plain sight
- **Dynamic Cover**: Destructible and movable cover
- **Awareness Indicators**: Show AI suspicion levels
- **Stealth Rating**: Score player's stealth performance