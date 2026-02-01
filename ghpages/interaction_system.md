# Interaction System

## Table of Contents

1. [Overview](#overview)
2. [Core Architecture](#core-architecture)
   - [InteractionDetector](#interactiondetector)
   - [Interactable](#interactable)
   - [UsingDevicesSystem](#usingdevicessystem)
   - [InteractionData](#interactiondata)
3. [Detection Workflow](#detection-workflow)
   - [Raycast Detection](#raycast-detection)
   - [Proximity Device Detection](#proximity-device-detection)
   - [Priority Logic](#priority-logic)
4. [Interaction Types](#interaction-types)
5. [Events & Feedback](#events--feedback)
   - [InteractionEvent](#interactionevent)
   - [Visual Debugging](#visual-debugging)
6. [UI System](#ui-system)
7. [Advanced Usage Examples](#advanced-usage-examples)

## Overview

The Interaction System is a robust framework designed to handle all player-world interactions. It supports two primary modes of detection:
1.  **Precision Raycasting**: For picking up small items or interacting with specific parts of an object (e.g., looking directly at a keypad).
2.  **Proximity Detection**: For "use" actions that should be forgiving, like opening a large door or activating a localized device without perfect aim.

## Core Architecture

### InteractionDetector

This component is the "eyes" of the interaction system. It should be attached to the entity representing the player's view (e.g., the Camera or the Player entity with an offset).

```rust
#[derive(Component)]
pub struct InteractionDetector {
    /// Maximum distance the ray will travel (default: 3.0 units)
    pub max_distance: f32,
    /// Offset from the entity's transform origin (e.g., eye level)
    pub ray_offset: Vec3,
    /// Optimization: Raycast frequency in seconds (default: 0.1s / 10Hz)
    pub update_interval: f32,
    /// Collision layers to check against (default: All)
    pub interaction_layers: u32,
}
```

### Interactable

Attached to world objects to make them interactive.

```rust
#[derive(Component)]
pub struct Interactable {
    /// Text displayed in the UI (e.g., "Open Chest")
    pub interaction_text: String,
    /// Distance required to interact (can be different from detection distance)
    pub interaction_distance: f32,
    /// Master switch for interaction
    pub can_interact: bool,
    /// The category of interaction (Pickup, Open, Use, etc.)
    pub interaction_type: InteractionType,
}
```

### UsingDevicesSystem

A player-side component that manages a "list of nearby devices". This is crucial for gameplay feels where the player can walk up to a console and blindly press 'E' to use it.

```rust
#[derive(Component)]
pub struct UsingDevicesSystem {
    pub can_use_devices: bool,
    pub raycast_distance: f32, // Distance to scan for devices
    pub device_list: Vec<DeviceInfo>, // Auto-populated list of nearby valid devices
    pub current_device_index: i32, // The currently selected device from the list
    // ...
}
```

### InteractionData

Optional component for advanced constraints.

```rust
#[derive(Component)]
pub struct InteractionData {
    pub cooldown: f32,       // Time in seconds before it can be used again
    pub cooldown_timer: f32, // Internal timer
    pub auto_trigger: bool,  // If true, activates immediately when in range (e.g., pressure plate)
    pub data: String,        // Generic data payload (e.g., Key ID required)
}
```

## Detection Workflow

The system runs a chain of logic every frame (or update interval) to determine what the player is looking at or standing near.

### 1. Raycast Detection
The `detect_interactables` system casts a ray from the `InteractionDetector` source.
-   If it hits an entity with an `Interactable` component:
    -   It populates the `CurrentInteractable` resource.
    -   Calculates precise hit distance and point.
    -   Checks if `hit_distance <= interactable.interaction_distance`.

### 2. Proximity Device Detection
The `detect_devices_in_proximity` system scans all entities with `DeviceStringAction` components around the player.
-   Adds them to `UsingDevicesSystem.device_list` if within `raycast_distance`.
-   Removes them if they move out of range.
-   The `select_closest_device` system then picks the best candidate from this list based on distance and angle settings.

### 3. Priority Logic
When the player presses the Interact key (Input), the `process_interactions` system decides which target to use:
1.  **Priority**: `UsingDevicesSystem` target (Context-sensitive device use).
2.  **Fallback**: `CurrentInteractable` (Raycast hit).

This ensures that if you are standing next to a button (Device), you press it, but if you look at a specific item on a table (Raycast), you pick that up instead.

## Interaction Types

-   `Pickup`: For inventory items.
-   `Use`: Generic interaction.
-   `Talk`: For conversational NPCs.
-   `Open`: Doors, crates, lockers.
-   `Activate`: Switches, computers.
-   `Examine`: Rotatable 3D inspection.
-   `Toggle`: On/Off states.
-   `Grab`: Physics object manipulation.
-   `Device`: Complex machine interaction.

## Events & Feedback

### InteractionEvent
The primary output of the system. Listen to `InteractionEventQueue` to add game logic.

```rust
pub struct InteractionEvent {
    pub source: Entity, // Player
    pub target: Entity, // Object
    pub interaction_type: InteractionType,
}
```

### Visual Debugging
Enable `InteractionDebugSettings` to see raycasts in the editor.
-   **Green Line**: Raycast ray.
-   **Orange Sphere**: Hit point on an interactable.
-   **Grey Line**: Miss/No hit.

## UI System
The system includes a built-in UI visualizer `update_interaction_ui`.
-   It reads the current "best" target (from Device system or Raycast).
-   Updates the prompt text: `Press {Key} to {Action} {Object Name}`.
-   Handles color coding (White = In Range, Red = Too Far).

## Advanced Usage Examples

### 1. Creating a Cooldown Switch
A switch that can only be engaged once every 5 seconds.

```rust
commands.spawn((
    // Basics
    Transform::from_xyz(0.0, 1.0, 0.0),
    Interactable {
        interaction_text: "Heavy Lever".into(),
        interaction_type: InteractionType::Activate,
        ..default()
    },
    // Logic
    InteractionData {
        cooldown: 5.0,
        ..default()
    },
    // State
    UsableDevice {
        active_text: "Pull Back".into(),
        inactive_text: "Pull Forward".into(),
        ..default()
    }
));
```

### 2. Handling the Event
System to play a sound when the lever is pulled.

```rust
fn handle_lever_sound(
    mut events: ResMut<InteractionEventQueue>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
) {
    for event in events.0.drain(..) {
        if event.interaction_type == InteractionType::Activate {
             audio.play(asset_server.load("sounds/lever_clunk.ogg"));
             info!("Clunk!");
        }
    }
}
```
