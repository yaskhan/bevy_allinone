# Devices System - Comprehensive Documentation

## Table of Contents

1. [Overview](#overview)
2. [Core Architecture](#core-architecture)
   - [ElectronicDevice](#electronicdevice)
   - [UsingDevicesSystem](#usingdevicessystem)
   - [Device String Actions](#device-string-actions)
3. [Device Types](#device-types)
   - [Door System](#door-system)
   - [Simple Switch](#simple-switch)
   - [Examine Object](#examine-object)
   - [Recharger Station](#recharger-station)
4. [Interaction Flow](#interaction-flow)
5. [Events System](#events-system)
6. [Setup and Usage](#setup-and-usage)

## Overview

The Devices System provides a unified framework for interactive objects in the game world. It handles everything from simple buttons and doors to complex inspectable items and recharging stations. The system is designed to be highly modular, supporting custom actions, animations, and event-driven logic.

## Core Architecture

### ElectronicDevice

The base component for any interactive entity. It manages the state of usage, player detection, and activation logic.

```rust
#[derive(Component)]
pub struct ElectronicDevice {
    pub device_can_be_used: bool,
    pub using_device: bool,
    pub player_inside: bool, // Is player in trigger?
    pub use_move_camera_to_device: bool, // Should camera focus on device?
    pub use_free_interaction: bool, // Allows moving while using?
}
```

### UsingDevicesSystem

The player-side component that manages detection and interaction input.

-   **Raycast Detection**: Finds devices in front of the player.
-   **Icon Management**: Show "Use" prompts on screen.
-   **Input Handling**: Processes the "Interact" key (default: E).

### Device String Actions

Defines the text and behavior for UI prompts.

```rust
#[derive(Component)]
pub struct DeviceStringAction {
    pub device_name: String,   // e.g., "Safe"
    pub device_action: String, // e.g., "Open"
    pub secondary_device_action: String, // e.g., "Close"
    pub icon_enabled: bool,
}
```

## Device Types

### Door System

A comprehensive system for managing doors, gates, and hatches.

**Features:**
-   **Movement Types**: Translate (Slide), Rotate (Swing), Animation.
-   **States**: Open, Closed, Locked.
-   **Auto-Close**: Close after `time_to_close` seconds.
-   **Locking**: keycard/tag requirement logic (`tag_list_to_open`).

```rust
pub struct DoorSystem {
    pub movement_type: DoorMovementType,
    pub locked: bool,
    pub open_speed: f32,
    pub auto_close: bool,
    // ...
}
```

### Simple Switch

Handles buttons, levers, and toggles.

**Modes:**
-   **Momentary**: Plays animation, triggers event (e.g., Call Elevator).
-   **Toggle**: Swaps between On/Off states (e.g., Light Switch).

**Integration:**
-   **Events**: Can trigger `SimpleSwitchEvent` to notify other systems.
-   **Animation**: Plays `switch_animation_name` on use.

### Examine Object

Allows the player to pick up and inspect an object in 3D space.

**Capabilities:**
-   **Rotation**: Mouse movement rotates the object.
-   **Zoom**: Scroll wheel zooms in/out.
-   **Interactive Places**: Clickable zones on the object (e.g., "Read text" on a letter).

### Recharger Station

Refills player attributes (Health or Energy) when used.

## Interaction Flow

1.  **Detection**: `UsingDevicesSystem` casts a ray or checks for `Collider` triggers.
2.  **Validation**: Checks distance (`raycast_distance`) and obstacles.
3.  **Prompt**: Displays `DeviceStringAction` UI (e.g., "Press E to Open").
4.  **Input**: Player presses Interaction Key.
5.  **Activation**:
    -   `ElectronicDevice` state is updated.
    -   Specific logic (Door open, detailed view) runs.
    -   Camera may transition to focus on the device.

## Events System

The system relies heavily on Bevy Events for decoupling.

-   `ElectronicDeviceActivationEvent`: Fired when a device is successfully used.
-   `DoorOpenCloseEvent`: Notifies when a door changes state.
-   `SimpleSwitchEvent`: Generic event for button presses.
-   `ExamineObjectEvent`: Handles start/stop of inspection.

## Setup and Usage

### creating a Simple Door

```rust
commands.spawn((
    // 1. Core Device
    ElectronicDevice::default(),
    DeviceStringAction {
        device_name: "Blast Door".into(),
        device_action: "Open".into(),
        secondary_device_action: "Close".into(),
        ..default()
    },
    // 2. Door Logic
    DoorSystem {
        movement_type: DoorMovementType::Translate,
        open_speed: 2.0,
        doors_info: vec![
            SingleDoorInfo {
                current_target_position: Vec3::new(0.0, 3.0, 0.0), // Slide up
                ..default()
            }
        ],
        ..default()
    },
    // 3. Visuals & Physics
    PbrBundle { ... },
    Collider::cuboid(1.0, 2.0, 0.1),
));
```
