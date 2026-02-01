# Map System - Comprehensive Documentation

## Table of Contents

1. [Overview](#overview)
2. [Core Components](#core-components)
   - [MapSettings](#mapsettings)
   - [MapMarker](#mapmarker)
   - [MapObjectInformation](#mapobjectinformation)
3. [Map Features](#map-features)
   - [Minimap](#minimap)
   - [Full Map](#full-map)
   - [Compass](#compass)
4. [Map Elements](#map-elements)
   - [Buildings & Floors](#buildings--floors)
   - [Quick Travel](#quick-travel)
   - [Glossary](#glossary)
   - [Objective Icons](#objective-icons)
5. [Setup and Usage](#setup-and-usage)

## Overview

The Map System provides a complete navigation solution including a Minimap, Full Map (World Map), and Compass. It handles marker tracking, 3D-to-2D projection, hierarchical positions (Buildings/Floors), and interactive elements like Quick Travel stations.

## Core Components

### MapSettings

Global configuration for the map system.

```rust
#[derive(Resource)]
pub struct MapSettings {
    pub show_minimap: bool,
    pub show_full_map: bool,
    pub minimap_zoom: f32, // Zoom level for minimap
    pub full_map_zoom: f32, // Zoom level for full map
    pub orientation: MapOrientation, // XY (Side), XZ (Top-down), YZ (Side)
}
```

### MapMarker

The logic component that marks an entity as trackable.

```rust
#[derive(Component)]
pub struct MapMarker {
    pub name: String,
    pub icon_type: MapIconType, // Npc, Quest, Enemy, etc.
    pub visible_in_minimap: bool,
    pub visible_in_full_map: bool,
}
```

### MapObjectInformation

Detailed metadata for complex objects. Checks building/floor context to determine visibility usage.

-   **`building_index`**: ID of the building the object belongs to (-1 = Global).
-   **`floor_index`**: ID of the floor.
-   **`icon_type`**: Definition for the UI icon.

## Map Features

### Minimap

A UI overlay (default: top-right) showing local entities.
-   **Projection**: Projects world coordinates to UI space based on `MapOrientation`.
-   **Filtering**: Only shows markers compatible with the player's current Building/Floor context.

### Full Map

A fullscreen overlay toggled with the **M** key.
-   Shows a broader view of the world.
-   Can display discovered zones.

### Compass

Rotates a UI element to match the player's facing direction.
-   Updates in real-time based on Player `Transform` rotation.

## Map Elements

### Buildings & Floors

Supports multi-level maps.

-   **`MapBuilding`**: Defines a building container.
-   **`MapFloor`**: Defines a specific floor layer.
-   **Visibility Logic**:
    -   If player is in Building A, Floor 2.
    -   Only markers with `building_index == A` AND `floor_index == 2` (or global markers) are clearly visible.

### Quick Travel

Teleporters allowing instant movement.

-   **`QuickTravelStation`**: Component marking a teleport spot.
-   **Interaction**: Press **E** when near an active station to travel.

### Glossary

Lore or information unlocked by map discovery.

-   **`MapGlossary`**: Unlocks content (`title`, `content`) when the associated marker is visited/interacted with.

### Objective Icons

Special icons for quests/missions.

-   **`ObjectiveIcon`**: Auto-generates a `MapMarker` when added.
-   **Off-screen Arrows**: (Planned) Indicator pointing to objectives outside the current view.

## Setup and Usage

### Adding a Marker

To show an entity on the radar:

```rust
commands.spawn((
    Transform::from_xyz(10.0, 0.0, 5.0),
    MapMarker {
        name: "Quest Giver".to_string(),
        icon_type: MapIconType::Quest,
        visible_in_minimap: true,
        ..default()
    }
));
```

### Configuring Map Settings

In your setup system:

```rust
fn configure_map(mut settings: ResMut<MapSettings>) {
    settings.minimap_zoom = 1.5;
    settings.orientation = MapOrientation::XZ; // Standard Top-Down
}
```

### Quick Travel Setup

```rust
commands.spawn((
    Transform::from_xyz(50.0, 0.0, 50.0),
    QuickTravelStation {
        destination: Vec3::new(0.0, 1.0, 0.0), // Base
        is_active: true,
        interact_message: "Travel to Base".into(),
    },
    // Add visual mesh...
));
```
