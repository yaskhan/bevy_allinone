# Climb System

The **Climb System** provides advanced vertical traversal capabilities, enabling players to grab ledges, climb walls, vault obstacles, and move horizontally across surfaces. Inspired by parkour mechanics in games like *Assassin's Creed* or *Mirror's Edge*, it transforms the environment into a navigable playground.

## Overview

The system is Raycast-driven, constantly probing the environment around the player to detect valid handholds and climbable geometry. It manages a state machine that transitions the character from standard movement into a "climbing mode" where gravity is defied and inputs translate to surface-relative movement.

### Key Logic Flow
1.  **Detection**: Is there a wall in front? Is there a ledge at hand-height?
2.  **Validation**: Is the surface angle correct? Is there space to hang?
3.  **Engagement**: Snap player position to the ledge (Inverse Kinematics targeting).
4.  **Traversal**: Process input to move along the ledge or jump to neighbor ledges.

## Core Concepts

### 1. Dual-Ray Detection
To ensure a valid ledge exists, the system uses a "Forward-then-Down" raycast strategy:
- **Ray A (Forward)**: Checks for a wall in front of the player (Chest height).
- **Ray B (Down)**: If Ray A hits, Ray B casts *downward* from a point slightly above the hit position.
- **Result**: If Ray B hits something, it means there is a top surface (a ledge). The difference between the player's height and Ray B's hit point determines if it's reachable.

### 2. Climb States
The system is built around the `ClimbState` enum:
- `None`: Normal walking/running.
- `Approaching`: Moving towards a valid ledge but not yet attached.
- `Hanging`: Static idle on a ledge.
- `ClimbingUp`/`Down`: Vertical movement.
- `ClimbingLeft`/`Right`: Shinmying sideways.
- `Vaulting`: A quick "up-and-over" action for low obstacles.

### 3. Surface Types
Not all walls are equal. The `SurfaceType` enum allows different materials to have different gameplay properties, primarily modifying climb speed using a multiplier:
- **Stone**: 1.0x (Standard)
- **Wood**: 0.9x (Slightly slower)
- **Metal**: 0.8x (Slippery/Hard to grip)
- **Ice**: 1.2x (Fast/Sliding?)
- **Rope**: 0.7x (Precise movement)

## Components Reference

### `ClimbLedgeSystem`
The massive configuration component attached to the Player. It controls every aspect of the feel and detection logic.

| Field | Type | Default | Description |
| :--- | :--- | :--- | :--- |
| `climb_ledge_active` | `bool` | `true` | Master switch for the system. |
| `climb_ledge_ray_forward_distance` | `f32` | `1.0` | How far forward to search for walls. |
| `climb_ledge_ray_down_distance` | `f32` | `1.0` | How deep to check for the ledge top. |
| `only_grab_ledge_if_moving_forward` | `bool` | `false` | If true, player must press 'W' to attach. |
| `auto_climb_in_third_person` | `bool` | `false` | If true, automatically climbs without jump button. |
| `can_jump_when_hold_ledge` | `bool` | `false` | Allows wall-jumps / back-ejects. |
| `jump_force_when_hold_ledge` | `f32` | `10.0` | power of the wall jump. |

### `ClimbStateTracker`
Manages the runtime state, specifically **Stamina**.
- Climbing drains stamina (`stamina_drain_rate`).
- Running out of stamina forces a release (`ClimbState::Falling`).
- Stamina regenerates when on the ground.

### `LedgeZone`
A component for level design. You can attach this to specific triggers or volumes to override climbing behavior in that area.
- `ledge_zone_can_be_climbed`: forcing a wall to be climable or unclimbable.
- `tag_to_check`: Only allows specific entities to climb here.

## Mechanics Details

### Auto-Hang
The system can be configured to automatically "catch" a ledge if the player falls past it.
- **Criteria**: Player must be close to the wall and falling.
- **Component**: `AutoHang` manages the transition, smoothly lerping the player toward the `target_ledge_position`.

### Vaulting
If a ledge is detected *below* a certain height threshold (e.g., waist height), the system triggers a "Vault" instead of a "Climb".
- Vaulting is a non-interruptible animation that moves the player to the other side of the obstacle immediately.

### Ledge Jumping
While hanging, the player can perform actions:
- **Jump Up**: Reach for a higher ledge.
- **Jump Back**: Eject away from the wall.
- **Drop**: Simply let go.

The `LedgeJump` component handles the force application (`ForceMode::Impulse`).

## Visuals & Animation

### Visual Debugging
The system relies heavily on raycasts, which are invisible.
- `LedgeDetection` stores the `raycast_hit_point` and `ledge_normal`.
- Use Gizmos to visualize these points when debugging why a wall isn't climbable.

### Animation Matching
The `ClimbAnimation` component is designed to sync with a motion-matching or traditional state machine animator.
- `match_start_value` / `match_end_value`: Normalized time for the climbing animation.
- `match_mask_value`: Mask for aligning body parts (Hands) to the ledge geometry.

## Integration Guide

### 1. Setting Up the Player
Add the necessary components to your CharacterController entity:

```rust
commands.spawn((
    // ... CharacterController components ...
    ClimbLedgeSystem::default(),
    ClimbStateTracker::default(),
    LedgeDetection::default(),
    AutoHang::default(),
    ClimbMovement::default(),
    ClimbAnimation::default(),
));
```

### 2. Marking the Environment
Most static geometry is climbable by detection, but use `LedgeZone` for manual overrides.

```rust
// Create a "Non-Climbable" zone (e.g., slippery slime wall)
commands.spawn((
    Transform::from_xyz(10.0, 5.0, 0.0),
    LedgeZone {
        ledge_zone_can_be_climbed: false,
        ..default()
    }
));
```

### 3. Handling Events
Listen to `LedgeGrabbedEvent` or `LedgeClimbedEvent` to play sound effects or trigger gameplay scripts.

```rust
fn play_climb_sfx(
    mut events: EventReader<LedgeGrabbedEvent>,
    asset_server: Res<AssetServer>,
    mut commands: Commands
) {
    for event in events.read() {
        // Play "Grab" sound at event.ledge_position
        commands.spawn(AudioBundle {
            source: asset_server.load("sounds/grab.ogg"),
            settings: PlaybackSettings::ONCE,
        });
    }
}
```

## Troubleshooting

### "Player won't grab the ledge"
1.  **Raycast Distance**: Increase `climb_ledge_ray_forward_distance`. The player collisions might be keeping them too far from the wall.
2.  **Layer Mask**: Ensure `layer_mask_to_check` matches the collision layer of your walls.
3.  **Active Flag**: Check `climb_ledge_active` is true.

### "Player grabs air"
**Cause**: `climb_ledge_ray_down_distance` is too long, hitting the floor behind the wall or a lower element.
**Fix**: Reduce the down-ray distance so it only catches the top surface of the immediate wall.

### "Stamina drains instantly"
**Cause**: `stamina_drain_rate` is too high relative to `max_stamina`.
**Fix**: Adjust values in `ClimbStateTracker`. Default drain is 10/sec, default max is 100 (10 seconds).

## Future Roadmap
- [ ] **Curved Ledges**: Better handling of non-linear surfaces (currently optimized for straight box-like geometry).
- [ ] **Corner Rounding**: Automatically transitioning from a North face to an East face of a building.
- [ ] **Dynmaic Objects**: Climbing on moving platforms (requires parent-local coordinate space updates).
