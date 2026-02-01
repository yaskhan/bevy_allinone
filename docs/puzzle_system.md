# Puzzle System Documentation
 
## Table of Contents
 
1. [Overview](#overview)
2. [Architecture](#architecture)
3. [Core Components](#core-components)
4. [Puzzle Types](#puzzle-types)
5. [Event System](#event-system)
6. [Integration Guide](#integration-guide)
7. [Advanced Features](#advanced-features)
8. [Best Practices](#best-practices)
9. [Examples](#examples)
 
---
 
## Overview
 
The Puzzle System is a comprehensive framework for creating interactive environmental challenges in your Bevy game. It provides a rich set of components and systems that enable developers to build complex, multi-layered puzzles with minimal boilerplate code.
 
### Key Features
 
- **Multiple Puzzle Types**: Support for buttons, levers, pressure plates, locks, sequences, piano puzzles, and object placement challenges
- **State Management**: Robust state tracking for puzzle progress, completion, and failure states
- **Event-Driven Architecture**: Custom event queues for inter-system communication and puzzle state changes
- **Visual & Audio Feedback**: Built-in support for visual indicators, animations, and sound effects
- **Debug Tools**: Gizmo visualization and debug information for puzzle development
- **Integration Ready**: Seamlessly integrates with the Interaction, Inventory, and Audio systems
- **Flexible Configuration**: Extensive customization options for timing, behavior, and appearance
 
### System Philosophy
 
The Puzzle System follows a component-based design philosophy where each puzzle element is a self-contained entity with specific components. This modular approach allows for:
 
- Easy mixing and matching of puzzle elements
- Reusable puzzle components across different scenes
- Clear separation of concerns between puzzle logic and game systems
- Straightforward debugging and iteration during development
 
---
 
## Architecture
 
### Plugin Structure
 
The Puzzle System is implemented as a Bevy plugin (`PuzzlePlugin`) that registers all necessary components, resources, and systems. When added to your app, it automatically sets up the complete puzzle infrastructure.
 
```rust
app.add_plugins(PuzzlePlugin);
```
 
### System Execution Order
 
The plugin registers systems in specific execution orders to ensure proper functionality:
 
**Update Systems** (run every frame, in chain):
1. `update_puzzle_buttons` - Handles button press/release logic and cooldowns
2. `update_puzzle_levers` - Manages lever state transitions and rotations
3. `update_puzzle_pressure_plates` - Processes weight detection and plate activation
4. `update_puzzle_locks` - Checks key requirements and unlock conditions
5. `update_puzzle_sequences` - Validates sequence order and progress
6. `update_puzzle_pianos` - Handles piano key presses and auto-play
7. `update_puzzle_object_placements` - Manages object placement validation
8. `update_puzzle_draggables` - Processes object grabbing and movement
9. `update_puzzle_timers` - Updates time limits and triggers timeouts
10. `process_puzzle_events` - Dispatches and processes all puzzle events
11. `update_puzzle_ui` - Refreshes UI elements based on puzzle state
12. `debug_draw_puzzle_gizmos` - Renders debug visualization
 
**Startup Systems**:
- `setup_puzzle_ui` - Initializes UI elements for puzzle feedback
 
### Resource Management
 
The system uses custom resource-based event queues (instead of Bevy's built-in events) to work around Bevy 0.18 EventReader limitations:
 
- `PuzzleEventQueue` - Generic puzzle events
- `PuzzleSolvedEventQueue` - Puzzle completion events
- `PuzzleFailedEventQueue` - Puzzle failure events
- `PuzzleResetEventQueue` - Puzzle reset events
- `PuzzleDebugSettings` - Debug configuration
 
---
 
## Core Components
 
### PuzzleSystem Component
 
The main orchestrator component for drag-and-drop style puzzles. This component manages the overall puzzle state and coordinates multiple draggable elements.
 
**Key Fields:**
 
| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `draggable_elements` | `Vec<Entity>` | `[]` | List of entities that can be dragged |
| `respawn_draggable` | `bool` | `false` | Whether objects respawn when leaving puzzle area |
| `respawn_in_original_position` | `bool` | `true` | Use original position vs. respawn point |
| `respawn_position` | `Option<Vec3>` | `None` | Custom respawn location |
| `drag_velocity` | `f32` | `8.0` | Speed multiplier for dragging |
| `max_raycast_distance` | `f32` | `10.0` | Maximum detection range for objects |
| `freeze_rotation_when_grabbed` | `bool` | `true` | Lock rotation during grab |
| `can_rotate_objects` | `bool` | `false` | Allow manual rotation |
| `rotate_speed` | `f32` | `30.0` | Rotation speed (degrees/second) |
| `horizontal_rotation_enabled` | `bool` | `true` | Enable horizontal rotation |
| `vertical_rotation_enabled` | `bool` | `true` | Enable vertical rotation |
| `rotate_to_camera_fixed` | `bool` | `false` | Auto-orient to camera |
| `camera_rotation_speed` | `f32` | `5.0` | Speed of camera-based rotation |
| `change_hold_distance_speed` | `f32` | `2.0` | Speed of distance adjustment |
| `number_objects_to_place` | `u32` | `0` | Required objects for completion |
| `disable_action_after_resolve` | `bool` | `false` | Lock interaction after solving |
| `wait_delay_after_resolve` | `f32` | `1.0` | Delay before disabling actions |
| `solved` | `bool` | `false` | Puzzle completion state |
| `using_puzzle` | `bool` | `false` | Player currently interacting |
| `current_objects_placed` | `u32` | `0` | Progress counter |
**Usage Example:**
 
```rust
commands.spawn((
    PuzzleSystem {
        draggable_elements: vec![cube_entity, sphere_entity, cylinder_entity],
        number_objects_to_place: 3,
        respawn_draggable: true,
        can_rotate_objects: true,
        rotate_speed: 45.0,
        ..default()
    },
    Transform::from_xyz(0.0, 0.0, 0.0),
));
```
 
### PuzzleProgress Component
 
Tracks the overall progress and state of any puzzle entity.
 
**Key Fields:**
 
| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `state` | `PuzzleState` | `Unsolved` | Current puzzle state |
| `progress` | `f32` | `0.0` | Progress percentage (0.0 - 1.0) |
| `total_steps` | `u32` | `1` | Total steps to complete |
| `current_step` | `u32` | `0` | Current step index |
| `can_reset` | `bool` | `true` | Allow puzzle reset |
| `reset_count` | `u32` | `0` | Number of resets |
| `time_spent` | `f32` | `0.0` | Elapsed time in seconds |
**PuzzleState Enum:**
- `Unsolved` - Initial state, puzzle not started
- `InProgress` - Player actively working on puzzle
- `Solved` - Puzzle successfully completed
- `Failed` - Puzzle failed (if failure mechanics enabled)
 
---
 
## Puzzle Types
 
### 1. Button Puzzles
 
Buttons are simple on/off switches that can be pressed by the player. They support cooldowns, timed activation, and auto-reset functionality.
 
**Component: `PuzzleButton`**
 
**Configuration Options:**
 
| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | `""` | Button identifier |
| `is_pressed` | `bool` | `false` | Current press state |
| `can_press` | `bool` | `true` | Enable/disable pressing |
| `cooldown` | `f32` | `0.5` | Seconds between presses |
| `cooldown_timer` | `f32` | `0.0` | Internal timer |
| `press_duration` | `f32` | `0.0` | How long press lasts (0 = instant) |
| `press_timer` | `f32` | `0.0` | Internal duration timer |
| `auto_reset` | `bool` | `false` | Reset automatically |
| `reset_delay` | `f32` | `1.0` | Delay before reset |
| `reset_timer` | `f32` | `0.0` | Internal reset timer |
| `press_amount` | `f32` | `0.3` | Visual depression distance |
| `press_speed` | `f32` | `10.0` | Animation speed |
| `sound_effect` | `Option<Handle<AudioSource>>` | `None` | Press sound |
| `use_incorrect_sound` | `bool` | `false` | Play error sound on wrong press |
| `incorrect_sound` | `Option<Handle<AudioSource>>` | `None` | Error sound |
**Behavior:**
- When interacted with, the button transitions to pressed state
- Visual feedback shows button depression based on `press_amount`
- If `press_duration > 0`, button stays pressed for that duration
- If `auto_reset` is true, button resets after `reset_delay`
- Cooldown prevents rapid pressing
 
**Example Setup:**
 
```rust
commands.spawn((
    PuzzleButton {
        name: "door_button".to_string(),
        cooldown: 1.0,
        auto_reset: true,
        reset_delay: 3.0,
        press_amount: 0.5,
        sound_effect: Some(button_sound.clone()),
        ..default()
    },
    Interactable {
        interaction_type: InteractionType::PuzzleButton,
        prompt_text: "Press Button".to_string(),
        ..default()
    },
    Transform::from_xyz(5.0, 1.0, 0.0),
));
```
 
### 2. Lever Puzzles
 
Levers are multi-state switches that can be pulled in different directions (up, down, left, right, neutral).
 
**Component: `PuzzleLever`**
 
**Configuration Options:**
 
| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | `""` | Lever identifier |
| `state` | `LeverState` | `Neutral` | Current position |
| `target_state` | `LeverState` | `Up` | State when activated |
| `rotation_axis` | `Vec3` | `Vec3::Y` | Rotation pivot axis |
| `rotation_range` | `Vec2` | `(-45, 45)` | Min/max angles in degrees |
| `rotation_speed` | `f32` | `30.0` | Animation speed |
| `can_move` | `bool` | `true` | Enable/disable movement |
| `sound_effect` | `Option<Handle<AudioSource>>` | `None` | Movement sound |
| `current_angle` | `f32` | `0.0` | Current rotation angle |
**LeverState Enum:**
- `Neutral` - Center position
- `Up` - Pulled upward
- `Down` - Pushed downward
- `Left` - Pulled to the left
- `Right` - Pulled to the right
 
**Behavior:**
- Lever smoothly rotates between states using the specified rotation axis
- Visual rotation is interpolated at `rotation_speed`
- Sound plays when state changes
- Can be used for directional puzzles or binary switches
 
**Example Setup:**
 
```rust
commands.spawn((
    PuzzleLever {
        name: "power_switch".to_string(),
        rotation_axis: Vec3::X,
        rotation_range: Vec2::new(-60.0, 60.0),
        rotation_speed: 45.0,
        sound_effect: Some(lever_sound.clone()),
        ..default()
    },
    Interactable {
        interaction_type: InteractionType::PuzzleLever,
        prompt_text: "Pull Lever".to_string(),
        ..default()
    },
    Transform::from_xyz(2.0, 2.0, 0.0),
));
```
 
### 3. Pressure Plate Puzzles
 
Pressure plates activate when sufficient weight is placed on them. They use Avian3d's physics system to detect colliding objects and calculate total weight.
 
**Component: `PuzzlePressurePlate`**
 
**Configuration Options:**
 
| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | `""` | Plate identifier |
| `is_pressed` | `bool` | `false` | Current activation state |
| `required_weight` | `f32` | `1.0` | Minimum weight to activate |
| `current_weight` | `f32` | `0.0` | Current weight on plate |
| `stay_pressed` | `bool` | `false` | Remain pressed after activation |
| `press_duration` | `f32` | `0.0` | Time to stay pressed (0 = instant) |
| `press_timer` | `f32` | `0.0` | Internal timer |
| `reset_delay` | `f32` | `0.5` | Delay before reset |
| `reset_timer` | `f32` | `0.0` | Internal reset timer |
| `press_amount` | `f32` | `0.1` | Visual depression distance |
| `sound_effect` | `Option<Handle<AudioSource>>` | `None` | Press sound |
| `release_sound` | `Option<Handle<AudioSource>>` | `None` | Release sound |
**Behavior:**
- Continuously checks for colliding entities with mass/weight
- Activates when `current_weight >= required_weight`
- Visual feedback shows plate depression
- If `stay_pressed` is false, deactivates when weight is removed
- Can be used for weight-based puzzles or multi-object requirements
 
**Physics Integration:**
The system uses Avian3d's collision detection to identify objects on the plate. Objects need appropriate colliders and mass components to be detected.
 
**Example Setup:**
 
```rust
commands.spawn((
    PuzzlePressurePlate {
        name: "heavy_door_plate".to_string(),
        required_weight: 50.0,
        stay_pressed: false,
        press_amount: 0.2,
        sound_effect: Some(plate_press_sound.clone()),
        release_sound: Some(plate_release_sound.clone()),
        ..default()
    },
    Collider::cuboid(2.0, 0.1, 2.0),
    Sensor,
    Transform::from_xyz(0.0, 0.0, 0.0),
));
```
 
### 4. Lock and Key Puzzles
 
Lock and key systems require the player to possess specific key items to unlock doors, chests, or trigger events.
 
**Components: `PuzzleLock` and `PuzzleKey`**
 
**PuzzleLock Configuration:**
 
| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | `""` | Lock identifier |
| `is_unlocked` | `bool` | `false` | Current unlock state |
| `required_keys` | `Vec<String>` | `[]` | List of required key IDs |
| `required_key_count` | `u32` | `1` | Number of keys needed |
| `current_keys` | `Vec<String>` | `[]` | Keys already inserted |
| `multi_key` | `bool` | `false` | Require multiple keys |
| `unlock_sound` | `Option<Handle<AudioSource>>` | `None` | Success sound |
| `wrong_key_sound` | `Option<Handle<AudioSource>>` | `None` | Failure sound |
| `lock_state` | `LockState` | `Locked` | Visual state |
**LockState Enum:**
- `Locked` - Completely locked
- `PartiallyUnlocked` - Some keys inserted (multi-key locks)
- `Unlocked` - Fully unlocked
 
**PuzzleKey Configuration:**
 
| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | `""` | Key name |
| `key_id` | `String` | `""` | Unique identifier (matches lock's required_keys) |
| `consumed_on_use` | `bool` | `true` | Remove from inventory after use |
| `reusable` | `bool` | `false` | Can be used multiple times |
| `use_sound` | `Option<Handle<AudioSource>>` | `None` | Usage sound |
| `key_type` | `KeyType` | `Physical` | Key category |
**KeyType Enum:**
- `Physical` - Traditional physical key item
- `Digital` - Digital code or password
- `Token` - Special token or collectible
- `Card` - Keycard for electronic locks
 
**Behavior:**
- Keys must be in player's inventory (integrates with Inventory System)
- Lock checks if player has required key(s) when interacted
- Multi-key locks track partial progress
- Keys can be consumed or remain in inventory
- Different lock states provide visual feedback
 
**Example Setup:**
 
```rust
// Create a key
let key_entity = commands.spawn((
    PuzzleKey {
        name: "Red Key".to_string(),
        key_id: "red_key".to_string(),
        consumed_on_use: false,
        reusable: true,
        key_type: KeyType::Physical,
        ..default()
    },
    Transform::from_xyz(-3.0, 1.0, 0.0),
)).id();
// Create a lock that requires the key
commands.spawn((
    PuzzleLock {
        name: "Red Door".to_string(),
        required_keys: vec!["red_key".to_string()],
        required_key_count: 1,
        unlock_sound: Some(unlock_sound.clone()),
        wrong_key_sound: Some(wrong_key_sound.clone()),
        ..default()
    },
    Interactable {
        interaction_type: InteractionType::PuzzleLock,
        prompt_text: "Unlock Door".to_string(),
        ..default()
    },
    Transform::from_xyz(5.0, 2.0, 0.0),
));
```
 
### 5. Sequence Puzzles
 
Sequence puzzles require the player to press buttons or interact with objects in a specific order.
 
**Components: `PuzzleSequence` and `PuzzleSequenceItem`**
 
**PuzzleSequence Configuration:**
 
| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | `""` | Sequence identifier |
| `correct_index` | `u32` | `0` | Current expected item index |
| `complete` | `bool` | `false` | Sequence completion state |
| `reset_on_wrong` | `bool` | `true` | Reset progress on wrong input |
| `correct_sound` | `Option<Handle<AudioSource>>` | `None` | Correct step sound |
| `incorrect_sound` | `Option<Handle<AudioSource>>` | `None` | Wrong step sound |
| `progress` | `f32` | `0.0` | Visual progress (0.0-1.0) |
**PuzzleSequenceItem Configuration:**
 
| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | `""` | Item identifier |
| `order_index` | `u32` | `0` | Position in sequence |
| `pressed` | `bool` | `false` | Has been pressed this attempt |
| `press_amount` | `f32` | `0.3` | Visual feedback amount |
| `sound_effect` | `Option<Handle<AudioSource>>` | `None` | Individual press sound |
**Behavior:**
- Each sequence item has an `order_index` defining its position
- Sequence tracks `correct_index` to validate order
- When wrong item is pressed:
  - If `reset_on_wrong` is true: sequence resets to index 0
  - If false: sequence continues, allowing retry
- Progress is calculated as `correct_index / total_items`
- Completion triggers when all items pressed in correct order
 
**Example Setup:**
 
```rust
// Create the sequence manager
let sequence_entity = commands.spawn((
    PuzzleSequence {
        name: "code_sequence".to_string(),
        reset_on_wrong: true,
        correct_sound: Some(correct_beep.clone()),
        incorrect_sound: Some(error_beep.clone()),
        ..default()
    },
)).id();
// Create sequence items (order: 0, 1, 2)
for (index, position) in [(0, Vec3::new(-2.0, 1.0, 0.0)),
                           (1, Vec3::new(0.0, 1.0, 0.0)),
                           (2, Vec3::new(2.0, 1.0, 0.0))].iter() {
    commands.spawn((
        PuzzleSequenceItem {
            name: format!("button_{}", index),
            order_index: *index,
            sound_effect: Some(button_click.clone()),
            ..default()
        },
        Interactable {
            interaction_type: InteractionType::PuzzleSequenceItem,
            prompt_text: "Press".to_string(),
            ..default()
        },
        Transform::from_xyz(position.x, position.y, position.z),
    ));
}
```
 
### 6. Piano Puzzles
 
Piano puzzles create musical challenges where players must play specific note sequences or melodies.
 
**Components: `PuzzlePiano` and `PuzzlePianoKey`**
 
**PuzzlePiano Configuration:**
 
| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | `""` | Piano identifier |
| `using_piano` | `bool` | `false` | Player currently playing |
| `key_rotation_amount` | `f32` | `30.0` | Key press animation angle |
| `key_rotation_speed` | `f32` | `30.0` | Animation speed |
| `song_to_play` | `String` | `""` | Auto-play song sequence |
| `play_rate` | `f32` | `0.3` | Seconds between auto-play notes |
| `use_event_on_auto_play_end` | `bool` | `false` | Trigger event on completion |
| `song_line_length` | `u32` | `8` | Notes per line (formatting) |
| `song_line_delay` | `f32` | `0.5` | Delay between lines |
| `auto_play_coroutine` | `Option<Handle<AudioSource>>` | `None` | Auto-play handle |
**PuzzlePianoKey Configuration:**
 
| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | `""` | Key identifier (e.g., "C", "D#") |
| `sound` | `Option<Handle<AudioSource>>` | `None` | Note sound |
| `current_rotation` | `f32` | `0.0` | Current key angle |
| `target_rotation` | `f32` | `-30.0` | Pressed position |
| `rotation_speed` | `f32` | `30.0` | Animation speed |
| `is_pressed` | `bool` | `false` | Current press state |
**Behavior:**
- Each piano key produces a specific note sound
- Keys visually rotate when pressed, returning to rest position
- Piano can auto-play sequences from `song_to_play` string
- Song format: space-separated note names (e.g., "C D E F G A B C")
- Can be combined with sequence validation for melody puzzles
- Supports both manual play and automated demonstrations
 
**Example Setup:**
 
```rust
// Create piano
let piano_entity = commands.spawn((
    PuzzlePiano {
        name: "puzzle_piano".to_string(),
        key_rotation_amount: 15.0,
        song_to_play: "C E G C".to_string(),
        play_rate: 0.5,
        ..default()
    },
    Transform::from_xyz(0.0, 1.0, 0.0),
)).id();
// Create piano keys
let notes = ["C", "D", "E", "F", "G", "A", "B"];
for (i, note) in notes.iter().enumerate() {
    commands.spawn((
        PuzzlePianoKey {
            name: note.to_string(),
            sound: Some(piano_sounds.get(note).unwrap().clone()),
            rotation_speed: 45.0,
            ..default()
        },
        Transform::from_xyz(i as f32 * 0.2, 1.0, 0.0),
    ));
}
```
 
### 7. Object Placement Puzzles
 
Object placement puzzles require players to position specific objects in designated locations with correct orientation.
 
**Components: `PuzzleObjectPlacement` and `PuzzleDraggable`**
 
**PuzzleObjectPlacement Configuration:**
 
| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | `""` | Placement spot identifier |
| `object_to_place` | `Option<Entity>` | `None` | Expected object entity |
| `is_placed` | `bool` | `false` | Object correctly placed |
| `place_position_speed` | `f32` | `5.0` | Snap-to-position speed |
| `place_rotation_speed` | `f32` | `10.0` | Snap-to-rotation speed |
| `use_rotation_limit` | `bool` | `false` | Enforce rotation constraints |
| `max_up_rotation` | `f32` | `30.0` | Max vertical deviation |
| `max_forward_rotation` | `f32` | `30.0` | Max forward deviation |
| `use_position_limit` | `bool` | `false` | Enforce position constraints |
| `max_position_distance` | `f32` | `1.0` | Maximum deviation radius |
| `needs_other_objects_before` | `bool` | `false` | Require prerequisites |
| `number_objects_before` | `u32` | `0` | Required previous placements |
| `current_objects_placed` | `u32` | `0` | Prerequisite counter |
| `object_inside_trigger` | `bool` | `false` | Object in placement zone |
| `object_in_correct_position` | `bool` | `false` | Position validation |
| `object_in_correct_rotation` | `bool` | `false` | Rotation validation |
**PuzzleDraggable Configuration:**
 
| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | `""` | Object identifier |
| `is_grabbed` | `bool` | `false` | Currently being held |
| `can_grab` | `bool` | `true` | Allow interaction |
| `original_position` | `Vec3` | `Vec3::ZERO` | Spawn position |
| `original_rotation` | `Quat` | `Quat::IDENTITY` | Spawn rotation |
| `hold_distance` | `f32` | `5.0` | Distance from camera |
| `is_rotating` | `bool` | `false` | Player rotating object |
| `freeze_rotation` | `bool` | `true` | Lock rotation when grabbed |
**Behavior:**
- Players can grab draggable objects using raycasting
- Objects follow camera at specified `hold_distance`
- Objects can be rotated if `can_rotate_objects` is enabled
- Placement zones validate position and rotation
- Prerequisites can create sequential placement puzzles
- Objects snap to correct position when placed properly
- Visual feedback indicates valid/invalid placement
 
**Example Setup:**
 
```rust
// Create draggable object
let cube_entity = commands.spawn((
    PuzzleDraggable {
        name: "red_cube".to_string(),
        original_position: Vec3::new(-5.0, 1.0, 0.0),
        hold_distance: 3.0,
        freeze_rotation: false,
        ..default()
    },
    Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
    MeshMaterial3d(materials.add(Color::srgb(1.0, 0.0, 0.0))),
    Collider::cuboid(1.0, 1.0, 1.0),
    RigidBody::Dynamic,
    Transform::from_xyz(-5.0, 1.0, 0.0),
)).id();
// Create placement zone
commands.spawn((
    PuzzleObjectPlacement {
        name: "cube_slot".to_string(),
        object_to_place: Some(cube_entity),
        place_position_speed: 8.0,
        use_rotation_limit: true,
        max_up_rotation: 15.0,
        max_forward_rotation: 15.0,
        ..default()
    },
    Collider::cuboid(1.1, 1.1, 1.1),
    Sensor,
    Transform::from_xyz(5.0, 1.0, 0.0),
));
```
 
---
 
## Event System
 
The Puzzle System uses a custom event queue architecture to communicate puzzle state changes throughout your game.
 
### Event Types
 
#### PuzzleSolvedEvent
 
Triggered when a puzzle is successfully completed.
 
```rust
pub struct PuzzleSolvedEvent {
    pub puzzle_entity: Entity,
    pub puzzle_name: String,
    pub time_spent: f32,
    pub reset_count: u32,
}
```
 
**Usage:**
```rust
fn handle_puzzle_completion(
    mut solved_events: ResMut,
) {
    for event in solved_events.0.drain(..) {
        info!("Puzzle '{}' solved in {:.1}s with {} resets!", 
              event.puzzle_name, event.time_spent, event.reset_count);
        // Unlock door, give reward, etc.
    }
}
```
 
#### PuzzleFailedEvent
 
Triggered when a puzzle fails (if failure conditions are implemented).
 
```rust
pub struct PuzzleFailedEvent {
    pub puzzle_entity: Entity,
    pub puzzle_name: String,
    pub reason: String,
}
```
 
#### PuzzleResetEvent
 
Triggered when a puzzle is reset to its initial state.
 
```rust
pub struct PuzzleResetEvent {
    pub puzzle_entity: Entity,
    pub puzzle_name: String,
}
```
 
#### Component-Specific Events
 
- **PuzzleButtonPressedEvent** - Button activation
- **PuzzleLeverMovedEvent** - Lever state change
- **PuzzlePressurePlatePressedEvent** - Pressure plate activation
- **PuzzleKeyUsedEvent** - Key usage attempt
- **PuzzleSequenceItemPressedEvent** - Sequence button press
- **PuzzlePianoKeyPressedEvent** - Piano note played
- **PuzzleObjectPlacedEvent** - Object placed correctly
- **PuzzleTimerTimeoutEvent** - Time limit exceeded
- **PuzzleHintShownEvent** - Hint displayed to player
 
### Accessing Events
 
Events are stored in resource queues and can be accessed in your systems:
 
```rust
fn my_puzzle_handler(
    mut event_queue: ResMut,
    mut solved_queue: ResMut,
) {
    // Process general puzzle events
    for event in event_queue.0.drain(..) {
        match event {
            PuzzleEvent::ButtonPressed(e) => {
                info!("Button {} pressed", e.button_name);
            },
            PuzzleEvent::Solved(e) => {
                info!("Puzzle {} completed!", e.puzzle_name);
            },
            _ => {}
        }
    }
    
    // Process solved events
    for event in solved_queue.0.drain(..) {
        // Award experience, open doors, etc.
    }
}
```
 
**Important Note:** Events are cleared after processing, similar to Bevy's event system. Make sure to drain and process events in the same frame.
 
---
 
## Advanced Features
 
### Puzzle Timer System
 
The timer system adds time pressure to puzzles, creating urgency and challenge.
 
**Component: `PuzzleTimer`**
 
```rust
commands.spawn((
    PuzzleTimer {
        active: true,
        time_limit: 60.0,  // 60 second limit
        use_event_on_timeout: true,
        pause_on_solve: true,
        ..default()
    },
    // ... other puzzle components
));
```
 
**Behavior:**
- Timer counts up from 0 to `time_limit`
- When limit reached, triggers `PuzzleTimerTimeoutEvent`
- Automatically pauses when puzzle is solved (if enabled)
- Can be displayed in UI for player feedback
 
### Hint System
 
Provides progressive hints to help players who get stuck.
 
**Component: `PuzzleHint`**
 
```rust
commands.spawn((
    PuzzleHint {
        text: "Try pressing the buttons in the order shown on the wall painting.".to_string(),
        level: 1,
        max_level: 3,
        time_before_hint: 45.0,  // Show after 45 seconds
        auto_show: true,
        ..default()
    },
    // ... other puzzle components
));
```
 
**Hint Levels:**
- Level 0: No hint
- Level 1: Vague hint
- Level 2: Direct hint
- Level 3: Full solution
 
**Behavior:**
- Timer increments while puzzle is unsolved
- When `timer >= time_before_hint`, hint becomes available
- If `auto_show` is true, hint displays automatically
- Otherwise, player must request hint (via UI or input)
- Can have multiple hint levels that progressively reveal more
 
### Sound Feedback
 
The system provides comprehensive audio feedback for all puzzle interactions.
 
**Component: `PuzzleSound`**
 
```rust
commands.spawn((
    PuzzleSound {
        solved_sound: Some(victory_sound.clone()),
        failed_sound: Some(failure_sound.clone()),
        correct_sound: Some(correct_beep.clone()),
        incorrect_sound: Some(wrong_beep.clone()),
        hint_sound: Some(hint_chime.clone()),
        volume: 0.8,
        use_spatial: true,
        ..default()
    },
    // ... other puzzle components
));
```
 
**Sound Types:**
- `solved_sound` - Played on puzzle completion
- `failed_sound` - Played on puzzle failure
- `reset_sound` - Played when puzzle resets
- `correct_sound` - Positive feedback for correct actions
- `incorrect_sound` - Negative feedback for wrong actions
- `hint_sound` - Notification when hint appears
 
**Spatial Audio:**
When `use_spatial` is true, sounds are positioned at the puzzle entity's location, creating 3D audio that helps players locate puzzle elements in the game world.
 
### Debug Visualization
 
The system includes built-in debug tools for puzzle development.
 
**Component: `PuzzleGizmo`**
 
```rust
commands.spawn((
    PuzzleGizmo {
        show: true,
        color: Color::srgb(0.0, 1.0, 0.0),  // Green
        radius: 0.5,
        arrow_length: 2.0,
        arrow_color: Color::srgb(1.0, 1.0, 0.0),  // Yellow
        ..default()
    },
    // ... other puzzle components
));
```
 
**Visualization Features:**
- Sphere gizmo at puzzle location
- Directional arrows showing orientation
- Color-coded state indicators
- Adjustable size and appearance
 
**Component: `PuzzleDebug`**
 
```rust
commands.spawn((
    PuzzleDebug {
        name: "Sequence Puzzle 1".to_string(),
        info: "4-button sequence".to_string(),
        show_debug: true,
    },
    // ... other puzzle components
));
```
 
**Debug Resource: `PuzzleDebugSettings`**
 
```rust
// Enable global debug mode
commands.insert_resource(PuzzleDebugSettings {
    enabled: true,
    show_gizmos: true,
    show_text: true,
    log_events: true,
});
```
 
---
 
## Integration Guide
 
### Integration with Interaction System
 
The Puzzle System is designed to work seamlessly with the Interaction System.
 
**Setup Pattern:**
 
```rust
use bevy_allinone::prelude::*;
// Spawn an interactable puzzle button
commands.spawn((
    // Puzzle component
    PuzzleButton {
        name: "activation_button".to_string(),
        cooldown: 2.0,
        sound_effect: Some(button_sound.clone()),
        ..default()
    },
    
    // Interaction component
    Interactable {
        interaction_type: InteractionType::PuzzleButton,
        prompt_text: "Press Button".to_string(),
        interaction_distance: 3.0,
        enabled: true,
        ..default()
    },
    
    // Visual components
    Mesh3d(meshes.add(Cuboid::new(0.5, 0.3, 0.5))),
    MeshMaterial3d(materials.add(Color::srgb(0.8, 0.2, 0.2))),
    
    Transform::from_xyz(0.0, 1.0, 0.0),
));
```
 
**How It Works:**
1. Player approaches puzzle element
2. Interaction System detects proximity and shows prompt
3. Player presses interaction key
4. Interaction System triggers `InteractionEvent`
5. Puzzle System receives event and updates puzzle state
6. Puzzle System generates `PuzzleEvent` if needed
7. Your game logic responds to puzzle events
 
### Integration with Inventory System
 
Key-based puzzles integrate with the Inventory System to check for required items.
 
**Example:**
 
```rust
fn check_puzzle_key_requirements(
    interaction_events: Res,
    mut puzzle_locks: Query,
    player_query: Query>,
    keys: Query,
) {
    for event in interaction_events.0.iter() {
        if let Ok(mut lock) = puzzle_locks.get_mut(event.target) {
            if let Ok(inventory) = player_query.get(event.source) {
                // Check if player has required keys in inventory
                for item_entity in &inventory.items {
                    if let Ok(key) = keys.get(*item_entity) {
                        if lock.required_keys.contains(&key.key_id) {
                            // Player has the key!
                            lock.current_keys.push(key.key_id.clone());
                            
                            if lock.current_keys.len() >= lock.required_key_count as usize {
                                lock.is_unlocked = true;
                                lock.lock_state = LockState::Unlocked;
                            }
                        }
                    }
                }
            }
        }
    }
}
```
 
### Integration with Quest System
 
Puzzles can trigger quest objectives or be required by quests.
 
**Example:**
 
```rust
fn update_quest_on_puzzle_solve(
    mut solved_events: ResMut,
    mut quest_logs: Query,
    mut quest_events: ResMut,
) {
    for event in solved_events.0.drain(..) {
        // Find quest logs that have the puzzle quest
        for mut log in quest_logs.iter_mut() {
            for quest in log.active_quests.iter_mut() {
                // Check if this puzzle is a quest objective
                if quest.name == "Solve the Temple Puzzle" {
                    for objective in quest.objectives.iter_mut() {
                        if objective.name.contains(&event.puzzle_name) {
                            objective.status = QuestStatus::Completed;
                            
                            // Trigger quest event
                            quest_events.0.push(QuestEvent::ObjectiveCompleted(
                                quest.id, 
                                0
                            ));
                        }
                    }
                }
            }
        }
    }
}
```
 
### Integration with Dialog System
 
Puzzles can unlock dialog options or be hinted at through conversations.
 
**Example:**
 
```rust
// Add dialog that hints at puzzle solution
Dialog {
    id: "old_sage_hint".to_string(),
    text: "The ancient mechanism requires you to press the stones in the order of the seasons: Spring, Summer, Fall, Winter.".to_string(),
    speaker: "Old Sage".to_string(),
    options: vec![
        DialogOption {
            text: "Thank you for the hint.".to_string(),
            next_dialog_id: None,
            ..default()
        }
    ],
    ..default()
}
```
 
---
 
## Best Practices
 
### 1. Puzzle Complexity
 
**Start Simple:** Begin with basic single-element puzzles and gradually increase complexity.
 
```rust
// Beginner: Single button
PuzzleButton::default()
// Intermediate: Sequence of 3 buttons
PuzzleSequence with 3 PuzzleSequenceItems
// Advanced: Multi-stage puzzle with timers and hints
PuzzleSystem + PuzzleTimer + PuzzleHint + Multiple object types
```
 
**Combine Systems:** The real power comes from combining multiple puzzle types.
 
```rust
// Example: Pressure plate unlocks a door after sequence is complete
// 1. Solve button sequence
// 2. Place heavy object on pressure plate
// 3. Door opens
```
 
### 2. Feedback is Critical
 
**Always provide clear feedback:**
- **Visual:** Buttons depress, levers rotate, lights change color
- **Audio:** Clicks, clunks, chimes for success/failure
- **UI:** Progress bars, hint text, remaining time
- **Haptic:** (If supported) Controller vibration
 
```rust
PuzzleButton {
    press_amount: 0.3,        // Visual depression
    press_speed: 10.0,        // Fast animation
    sound_effect: Some(...),  // Audio feedback
    ..default()
}
```
 
### 3. Reset and Recovery
 
**Allow players to recover from mistakes:**
 
```rust
PuzzleSequence {
    reset_on_wrong: true,  // Reset on error, allowing retry
    ..default()
}
PuzzleProgress {
    can_reset: true,       // Allow manual reset
    ..default()
}
```
 
**Consider auto-reset on failure:**
- Timeout resets puzzle
- Wrong action resets progress
- Option to manually reset
 
### 4. Accessibility
 
**Make puzzles accessible:**
- Provide hints for players who get stuck
- Allow difficulty adjustments
- Ensure colorblind-friendly visual feedback
- Include audio cues for visual elements
- Consider alternative solutions
 
```rust
PuzzleHint {
    auto_show: true,
    time_before_hint: 60.0,  // Generous time before hint
    max_level: 3,            // Multiple hint levels
    ..default()
}
```
 
### 5. Testing and Iteration
 
**Playtest extensively:**
- Watch new players attempt puzzles
- Track completion rates and times
- Identify frustration points
- Adjust difficulty based on feedback
 
**Use debug tools during development:**
 
```rust
#[cfg(debug_assertions)]
commands.insert_resource(PuzzleDebugSettings {
    enabled: true,
    show_gizmos: true,
    log_events: true,
});
```
 
### 6. Performance Considerations
 
**Optimize puzzle systems:**
- Use spatial queries efficiently (don't raycast every frame)
- Pool audio sources for frequently played sounds
- Disable puzzle systems when player is far away
- Use LOD for complex puzzle meshes
 
```rust
// Only update puzzles near player
fn update_active_puzzles(
    player: Query>,
    mut puzzles: Query>,
) {
    let player_pos = player.single().translation;
    
    for (puzzle_transform, mut button) in puzzles.iter_mut() {
        let distance = player_pos.distance(puzzle_transform.translation);
        
        if distance < 20.0 {  // Only update nearby puzzles
            // Update button logic
        }
    }
}
```
 
### 7. Save System Integration
 
**Ensure puzzle state persists:**
 
```rust
// Mark puzzles as saveable
commands.spawn((
    PuzzleButton { /* ... */ },
    SaveableEntity,  // Custom component for save system
));
// Save puzzle state
fn save_puzzle_progress(
    puzzles: Query,
    mut save_data: ResMut,
) {
    for (progress, button) in puzzles.iter() {
        save_data.puzzles.push(PuzzleState {
            state: progress.state,
            button_pressed: button.is_pressed,
            // ... other state
        });
    }
}
```
 
---
 
## Examples
 
### Example 1: Simple Door Button
 
A basic button that opens a door when pressed.
 
```rust
use bevy::prelude::*;
use bevy_allinone::prelude::*;
fn spawn_door_puzzle(
    mut commands: Commands,
    mut meshes: ResMut<Assets>,
    mut materials: ResMut<Assets>,
    asset_server: Res,
) {
    // Spawn door
    let door = commands.spawn((
        Name::new("Door"),
        Mesh3d(meshes.add(Cuboid::new(2.0, 3.0, 0.2))),
        MeshMaterial3d(materials.add(Color::srgb(0.5, 0.3, 0.1))),
        Transform::from_xyz(0.0, 1.5, 5.0),
    )).id();
    
    // Spawn button
    commands.spawn((
        Name::new("Door Button"),
        PuzzleButton {
            name: "door_button".to_string(),
            auto_reset: false,  // Button stays pressed
            press_amount: 0.2,
            sound_effect: Some(asset_server.load("sounds/button_press.ogg")),
            ..default()
        },
        Interactable {
            interaction_type: InteractionType::PuzzleButton,
            prompt_text: "Press Button".to_string(),
            interaction_distance: 2.0,
            enabled: true,
            ..default()
        },
        Mesh3d(meshes.add(Cuboid::new(0.5, 0.2, 0.5))),
        MeshMaterial3d(materials.add(Color::srgb(1.0, 0.0, 0.0))),
        Transform::from_xyz(-2.0, 1.0, 3.0),
    ));
    
    // System to open door when button pressed
    commands.add_systems(Update, open_door_on_button);
}
fn open_door_on_button(
    buttons: Query,
    mut doors: Query>,
    time: Res,
) {
    for button in buttons.iter() {
        if button.is_pressed {
            for mut transform in doors.iter_mut() {
                // Slide door up
                transform.translation.y += time.delta_secs() * 2.0;
                transform.translation.y = transform.translation.y.min(6.0);
            }
        }
    }
}
```
 
### Example 2: Four-Button Sequence
 
A sequence puzzle requiring buttons to be pressed in order: 1, 2, 3, 4.
 
```rust
fn spawn_sequence_puzzle(
    mut commands: Commands,
    mut meshes: ResMut<Assets>,
    mut materials: ResMut<Assets>,
    asset_server: Res,
) {
    // Create sequence manager
    let sequence = commands.spawn((
        Name::new("Sequence Puzzle"),
        PuzzleSequence {
            name: "vault_code".to_string(),
            reset_on_wrong: true,
            correct_sound: Some(asset_server.load("sounds/correct_beep.ogg")),
            incorrect_sound: Some(asset_server.load("sounds/error_buzz.ogg")),
            ..default()
        },
        PuzzleProgress::default(),
        Transform::from_xyz(0.0, 0.0, 0.0),
    )).id();
    
    // Create 4 buttons in a square pattern
    let positions = [
        Vec3::new(-1.0, 1.0, 0.0),  // Button 1 (top-left)
        Vec3::new(1.0, 1.0, 0.0),   // Button 2 (top-right)
        Vec3::new(-1.0, -1.0, 0.0), // Button 3 (bottom-left)
        Vec3::new(1.0, -1.0, 0.0),  // Button 4 (bottom-right)
    ];
    
    for (index, pos) in positions.iter().enumerate() {
        commands.spawn((
            Name::new(format!("Sequence Button {}", index + 1)),
            PuzzleSequenceItem {
                name: format!("button_{}", index + 1),
                order_index: index as u32,
                sound_effect: Some(asset_server.load("sounds/button_click.ogg")),
                ..default()
            },
            Interactable {
                interaction_type: InteractionType::PuzzleSequenceItem,
                prompt_text: format!("Press Button {}", index + 1),
                interaction_distance: 2.0,
                enabled: true,
                ..default()
            },
            Mesh3d(meshes.add(Sphere::new(0.3))),
            MeshMaterial3d(materials.add(Color::srgb(0.2, 0.8, 1.0))),
            Transform::from_translation(*pos),
        ));
    }
}
```
 
### Example 3: Weight Puzzle with Multiple Objects
 
A puzzle requiring multiple objects to be placed on pressure plates.
 
```rust
fn spawn_weight_puzzle(
    mut commands: Commands,
    mut meshes: ResMut<Assets>,
    mut materials: ResMut<Assets>,
    asset_server: Res,
) {
    // Create two pressure plates
    let plate_positions = [Vec3::new(-3.0, 0.0, 0.0), Vec3::new(3.0, 0.0, 0.0)];
    
    for pos in plate_positions {
        commands.spawn((
            Name::new("Pressure Plate"),
            PuzzlePressurePlate {
                name: "weight_plate".to_string(),
                required_weight: 10.0,
                stay_pressed: true,
                press_amount: 0.1,
                sound_effect: Some(asset_server.load("sounds/plate_press.ogg")),
                ..default()
            },
            Mesh3d(meshes.add(Cuboid::new(2.0, 0.1, 2.0))),
            MeshMaterial3d(materials.add(Color::srgb(0.3, 0.3, 0.3))),
            Collider::cuboid(2.0, 0.1, 2.0),
            Sensor,
            Transform::from_translation(pos),
        ));
    }
    
    // Create heavy boxes
    let box_positions = [
        Vec3::new(-6.0, 1.0, 0.0),
        Vec3::new(6.0, 1.0, 0.0),
        Vec3::new(0.0, 1.0, -3.0),
    ];
    
    for pos in box_positions {
        commands.spawn((
            Name::new("Heavy Box"),
            PuzzleDraggable {
                name: "box".to_string(),
                can_grab: true,
                hold_distance: 3.0,
                ..default()
            },
            Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
            MeshMaterial3d(materials.add(Color::srgb(0.6, 0.4, 0.2))),
            Collider::cuboid(1.0, 1.0, 1.0),
            RigidBody::Dynamic,
            Mass(15.0),  // Heavy enough to activate plates
            Transform::from_translation(pos),
        ));
    }
}
```
 
### Example 4: Piano Melody Puzzle
 
A puzzle where the player must play a specific melody on a piano.
 
```rust
fn spawn_piano_puzzle(
    mut commands: Commands,
    mut meshes: ResMut<Assets>,
    mut materials: ResMut<Assets>,
    asset_server: Res,
) {
    // Load piano note sounds
    let note_sounds = [
        asset_server.load("sounds/piano_c.ogg"),
        asset_server.load("sounds/piano_d.ogg"),
        asset_server.load("sounds/piano_e.ogg"),
        asset_server.load("sounds/piano_f.ogg"),
        asset_server.load("sounds/piano_g.ogg"),
        asset_server.load("sounds/piano_a.ogg"),
        asset_server.load("sounds/piano_b.ogg"),
    ];
    
    let notes = ["C", "D", "E", "F", "G", "A", "B"];
    
    // Create piano
    let piano = commands.spawn((
        Name::new("Puzzle Piano"),
        PuzzlePiano {
            name: "secret_melody".to_string(),
            key_rotation_amount: 15.0,
            key_rotation_speed: 50.0,
            song_to_play: "C E G E C".to_string(),  // Secret melody
            ..default()
        },
        PuzzleProgress::default(),
        Transform::from_xyz(0.0, 1.0, 0.0),
    )).id();
    
    // Create piano keys
    for (i, note) in notes.iter().enumerate() {
        let key_entity = commands.spawn((
            Name::new(format!("Piano Key {}", note)),
            PuzzlePianoKey {
                name: note.to_string(),
                sound: Some(note_sounds[i].clone()),
                rotation_speed: 50.0,
                ..default()
            },
            Interactable {
                interaction_type: InteractionType::PuzzlePianoKey,
                prompt_text: format!("Play {}", note),
                interaction_distance: 2.0,
                enabled: true,
                ..default()
            },
            Mesh3d(meshes.add(Cuboid::new(0.15, 0.05, 0.8))),
            MeshMaterial3d(materials.add(Color::srgb(1.0, 1.0, 1.0))),
            Transform::from_xyz(i as f32 * 0.2 - 0.6, 1.0, 0.0),
        )).id();
    }
    
    // Create sequence validator
    commands.spawn((
        Name::new("Melody Validator"),
        PuzzleSequence {
            name: "melody_sequence".to_string(),
            reset_on_wrong: true,
            correct_sound: Some(asset_server.load("sounds/success_chord.ogg")),
            incorrect_sound: Some(asset_server.load("sounds/wrong_note.ogg")),
            ..default()
        },
    ));
}
```
 
### Example 5: Complex Multi-Stage Puzzle
 
A comprehensive puzzle combining multiple mechanics.
 
```rust
fn spawn_complex_puzzle(
    mut commands: Commands,
    mut meshes: ResMut<Assets>,
    mut materials: ResMut<Assets>,
    asset_server: Res,
) {
    // Stage 1: Find and use a key to unlock a door
    let key = commands.spawn((
        Name::new("Ancient Key"),
        PuzzleKey {
            name: "Ancient Key".to_string(),
            key_id: "ancient_key".to_string(),
            consumed_on_use: false,
            key_type: KeyType::Physical,
            ..default()
        },
        Mesh3d(meshes.add(Capsule3d::new(0.05, 0.3))),
        MeshMaterial3d(materials.add(Color::srgb(1.0, 0.84, 0.0))),
        Transform::from_xyz(-10.0, 1.0, 0.0),
    )).id();
    
    let locked_door = commands.spawn((
        Name::new("Ancient Door"),
        PuzzleLock {
            name: "ancient_door".to_string(),
            required_keys: vec!["ancient_key".to_string()],
            unlock_sound: Some(asset_server.load("sounds/door_unlock.ogg")),
            ..default()
        },
        Interactable {
            interaction_type: InteractionType::PuzzleLock,
            prompt_text: "Use Key".to_string(),
            interaction_distance: 2.0,
            enabled: true,
            ..default()
        },
        Mesh3d(meshes.add(Cuboid::new(2.0, 3.0, 0.2))),
        MeshMaterial3d(materials.add(Color::srgb(0.4, 0.3, 0.2))),
        Transform::from_xyz(0.0, 1.5, 0.0),
    )).id();
    
    // Stage 2: Solve a sequence puzzle
    let sequence = commands.spawn((
        Name::new("Rune Sequence"),
        PuzzleSequence {
            name: "rune_activation".to_string(),
            reset_on_wrong: true,
            ..default()
        },
        PuzzleProgress::default(),
    )).id();
    
    // Create sequence runes
    let rune_order = [2, 0, 3, 1];  // Correct order
    for (display_index, actual_order) in rune_order.iter().enumerate() {
        commands.spawn((
            Name::new(format!("Rune {}", display_index)),
            PuzzleSequenceItem {
                name: format!("rune_{}", display_index),
                order_index: *actual_order,
                ..default()
            },
            Interactable {
                interaction_type: InteractionType::PuzzleSequenceItem,
                prompt_text: "Touch Rune".to_string(),
                interaction_distance: 2.0,
                enabled: true,
                ..default()
            },
            Transform::from_xyz(display_index as f32 * 1.5, 1.5, 5.0),
        ));
    }
    
    // Stage 3: Place crystal on pedestal
    let crystal = commands.spawn((
        Name::new("Power Crystal"),
        PuzzleDraggable {
            name: "crystal".to_string(),
            hold_distance: 2.5,
            ..default()
        },
        Mesh3d(meshes.add(Sphere::new(0.3))),
        MeshMaterial3d(materials.add(Color::srgb(0.5, 0.8, 1.0))),
        Collider::sphere(0.3),
        RigidBody::Dynamic,
        Transform::from_xyz(5.0, 1.0, 5.0),
    )).id();
    
    commands.spawn((
        Name::new("Crystal Pedestal"),
        PuzzleObjectPlacement {
            name: "pedestal".to_string(),
            object_to_place: Some(crystal),
            place_position_speed: 5.0,
            use_rotation_limit: false,
            ..default()
        },
        Collider::cylinder(0.3, 1.0),
        Sensor,
        Transform::from_xyz(0.0, 0.5, 10.0),
    ));
    
    // Add timer and hints
    commands.spawn((
        Name::new("Puzzle Controller"),
        PuzzleSystem {
            number_objects_to_place: 1,
            ..default()
        },
        PuzzleTimer {
            active: true,
            time_limit: 300.0,  // 5 minutes
            use_event_on_timeout: true,
            ..default()
        },
        PuzzleHint {
            text: "The runes must be activated in the order shown by the star constellation above.".to_string(),
            time_before_hint: 60.0,
            auto_show: true,
            max_level: 2,
            ..default()
        },
        PuzzleSound {
            solved_sound: Some(asset_server.load("sounds/puzzle_complete.ogg")),
            volume: 1.0,
            ..default()
        },
    ));
}
```
 
---
 
## Conclusion
 
The Puzzle System provides a robust, flexible framework for creating engaging environmental challenges in your Bevy game. By combining different puzzle types, integrating with other game systems, and following best practices, you can create memorable puzzle experiences that challenge and delight your players.
 
### Key Takeaways
 
1. **Component-Based Design**: Each puzzle element is a self-contained entity with specific components
2. **Event-Driven Communication**: Use event queues to respond to puzzle state changes
3. **Extensive Customization**: Every aspect of puzzles can be tuned and configured
4. **Integration Ready**: Works seamlessly with Interaction, Inventory, Quest, and other systems
5. **Player-Friendly**: Built-in support for hints, timers, reset mechanics, and accessibility
 
### Further Resources
 
- **Source Code**: `/src/puzzle.rs`
- **Examples**: `/examples/puzzle_demo.rs`
- **Related Systems**:
  - [Interaction System Documentation](./interaction-system.md)
  - [Inventory System Documentation](./inventory-system.md)
  - [Quest System Documentation](./quest-system.md)
 
### Community and Support
 
For questions, bug reports, or feature requests related to the Puzzle System, please visit the project's GitHub repository or join the community Discord server.
 
---
 
*Last Updated: January 2026*
*Bevy All-in-One Controller v0.1.0*