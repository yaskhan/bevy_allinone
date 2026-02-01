# Ladder System

The **Ladder System** provides structured vertical movement mechanics, allowing players to mount, climb, and dismount defined ladder objects. Unlike the free-form [Climb System](./climb_system.md), ladders offer constrained axis movement and specific interaction states.

## Overview

The system operates on a "Constraint" basis. When a player interacts with a ladder, their physics control is overridden. Gravity is disabled, and input vectors are remapped to slide the character along the ladder's local Up/Right vectors.

### Key Logic Flow
1.  **Detection**: Player enters a trigger volume or raycasts onto a ladder.
2.  **Mounting**: A smooth lerp/animation moves the player to the ladder's center line.
3.  **Climbing**: Input 'Forward/Backward' is translated to 'Up/Down' ladder movement.
4.  **Dismounting**: Reaching the top/bottom triggers an exit animation and restores physics.

## Core Concepts

### 1. Control Relativity
The system intelligently handles camera-relative inputs.
- **Looking Up + Press W**: Climbs Up.
- **Looking Down + Press W**: Climbs Down.

This is managed by comparing the camera's forward vector with the ladder's up vector (`ladder_angle`). If the angle exceeds `min_angle_to_inverse_direction` (typically 100 degrees), the input sign is flipped.

### 2. Ladder States (`LadderMovementState`)
- `None`: Normal gameplay.
- `Approaching`: Moving to engagement distance.
- `Mounting`: Locked animation entering the ladder.
- `ClimbingUp`/`Down`: Active user control.
- `Dismounting`: Locked animation exiting the ladder.

## Components Reference

### `LadderSystem` (The Object)
Attached to the ladder entity in the world.

| Field | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `tag_to_check` | `String` | `"Player"` | Only entities with this Tag can use this ladder. |
| `use_ladder_horizontal_movement` | `bool` | `true` | Allows A/D shimmying. |
| `move_in_ladder_center` | `bool` | `false` | Forces player X-axis to lock to center. |
| `gizmo_color` | `Color` | `Red` | Debug line visualization. |

### `PlayerLadderSystem` (The Player)
Attached to the character controller.

| Field | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `ladder_movement_speed` | `f32` | `5.0` | Base climb velocity. |
| `min_angle_to_inverse_direction` | `f32` | `100.0` | Angle threshold for reversing Up/Down inputs. |
| `ladder_found` | `bool` | `false` | Runtime flag (ReadOnly). |

### `LadderMovement`
Tracks real-time inputs and velocity application.
- `vertical_movement_amount`: Multiplier for Y-axis speed (0.3).
- `horizontal_movement_amount`: Multiplier for X-axis speed (0.1).

## Mechanics Details

### Mounting
Mounting is a critical phase where control is taken away to align the character.
1.  **State**: Sets to `LadderMovementState::Mounting`.
2.  **Gravity**: Disables `gravity_scale` or sets `zero_gravity_mode = true`.
3.  **Alignment**: Lerps `transform.position` to `ladder.position + offset`.
4.  **Animation**: Plays the "Mount" clip.

### Dismounting
Occurs when:
1.  **Top/Bottom End**: `LadderExitDetection` raycasts hit nothing (top) or floor (bottom).
2.  **Jump**: Player presses Spacebar to bail.

The system calculates a `dismount_target_position` (e.g., 0.5 units behind the player) and lerps them there before re-enabling gravity.

### Horizontal Shimmy
If `use_ladder_horizontal_movement` is enabled, 'A' and 'D' keys move the player along the ladder's local Right vector.
- **Usage**: Wide industrial ladders or mesh climbing.

## Integration Guide

### 1. Creating a Ladder
1.  Create a mesh/primitive (Cube).
2.  Add `LadderSystem` component.
3.  Add `LadderDirection` component (defines which way is "Up").
4.  Add `Collider` (Sensor) for detection.

```rust
commands.spawn((
    Transform::from_xyz(0.0, 5.0, 0.0),
    LadderSystem {
        use_ladder_horizontal_movement: false,
        ..default()
    },
    LadderDirection::default(),
    Collider::cuboid(0.5, 5.0, 0.5), // Needs to be a Trigger/Sensor
    Sensor,
));
```

### 2. Configuring the Player
Must have `PlayerLadderSystem` and `LadderExitDetection`.

```rust
commands.spawn((
    // ... CharacterController ...
    PlayerLadderSystem {
        ladder_movement_speed: 4.0,
        ..default()
    },
    LadderMovement::default(),
    LadderMovementTracker::default(),
    LadderAnimation::default(),
    LadderExitDetection::default(), // Crucial for not climbing forever
));
```

## Troubleshooting

### "Player climbs upside down"
**Cause**: The `LadderDirection` vector on the ladder entity is inverted.
**Fix**: Rotate the ladder entity 180 degrees or manually set `LadderDirection.direction = Vec3::NEG_Y` (rare).

### "Player walks through the ladder"
**Cause**: No collision trigger or `tag_to_check` mismatch.
**Fix**: Ensure the ladder has a `Sensor` collider and the Player has a `Name` or Tag matching `ladder.tag_to_check`.

### "Controls feel reversed"
**Cause**: `min_angle_to_inverse_direction` might be too sensitive or camera forward vector is misaligned.
**Fix**: Increase the angle to 120.0 or debug draw the `ladder_angle` to see what the system thinks the view alignment is.
