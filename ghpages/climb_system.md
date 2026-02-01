# Climb & Parkour System

The **Climb System** provides a complete solution for vertical traversal, ledge hanging, wall running, and parkour mechanics. It allows players to detect climbable surfaces via raycasting, transition seamlessly between movement states, and interact with the environment in a fluid, "Assassin's Creed"-like manner.

## Documentation Contents

### Part 1: Core Architecture
- **Overview** - System capabilities and design philosophy
- **The State Machine** - Understanding `ClimbState`
- **Component Reference** - Deep dive into configuration components
    - `ClimbLedgeSystem`
    - `ClimbStateTracker`
    - `LedgeZone`

### Part 2: Ledge Detection & Physics
- **Detection Logic** - The Dual-Ray approach
- **Auto-Hang** - Dropping onto ledges safely
- **Wall Movement** - Physics calculations for shimmying and vaulting
- **Surface Types** - Material-based gameplay modifiers

### Part 3: Animation & Advanced Topics
- **Animation Integration** - Target Matching and Masks
- **Stamina System** - Fatigue mechanics
- **Troubleshooting** - Solving common setup issues

---

## Overview

The Climb System separates "movement" from "climbing". When a climbable edge is detected, the standard character controller logic (gravity, friction, walking speed) is suspended, and the `ClimbState` takes over.

### Key Capabilities
-   **Procedural Edge Detection**: Uses raycasts to find ledges on *any* geometry without manual tagging (though manual `LedgeZone`s are supported).
-   **Fluid State Transitions**: Seamlessly move from *Running* -> *Vaulting* -> *Falling* -> *Hanging*.
-   **Stamina Management**: Climbing consumes stamina; running out causes the player to fall.
-   **Vehicle Integration**: Can be configured to allow/disallow climbing while on vehicles.

---

## The State Machine

The heart of the system is the `ClimbState` enum. This finite state machine determines allowed inputs and movement vectors.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Reflect)]
pub enum ClimbState {
    None,           // Standard character movement
    Approaching,    // Moving towards a detected ledge (auto-adjust)
    Hanging,        // Idle on the wall
    ClimbingUp,     // Ascending
    ClimbingDown,   // Descending
    ClimbingLeft,   // Shimmying left
    ClimbingRight,  // Shimmying right
    Vaulting,       // Quick hop over low obstacles
    Falling,        // Special fall state (detached from wall)
}
```

### State Flow
1.  **Detection**: System scans for a ledge while `ClimbState::None`.
2.  **Transition**: If detected + Input (e.g., Jump/Forward), switch to `Approaching`.
3.  **Hanging**: Once aligned, switch to `Hanging`. Gravity is disabled.
4.  **Movement**: Player input (WASD) switches state to `ClimbingUp/Down/Left/Right`.
5.  **Exit**:
    -   **Jump**: Launches player away/up (`ForceMode::Impulse`).
    -   **Drop**: Pressing Crouch/Drop key switches to `Falling` or `None`.
    -   **Top Out**: Moving up past the edge switches to standard "Grounded" state.

---

## Component Reference

### 1. `ClimbLedgeSystem`
This is the "Brain" of the climb system. It contains typically static configuration settings for how climbing *feels*. It is attached to the Player entity.

#### Main Settings
| Field | Type | Description |
| :--- | :--- | :--- |
| `climb_ledge_active` | `bool` | Master switch for the system. |
| `climb_ledge_ray_forward_distance` | `f32` | How far forward to check for walls (default: `1.0`). |
| `climb_ledge_ray_down_distance` | `f32` | How far down to check for the lip of the ledge (default: `1.0`). |
| `layer_mask_to_check` | `u32` | Physics layer mask for climbable geometry. |

#### Movement & Positioning
| Field | Type | Description |
| :--- | :--- | :--- |
| `adjust_to_hold_on_ledge_position_speed` | `f32` | Lerp speed when snapping to the wall (default: `3.0`). |
| `adjust_to_hold_on_ledge_rotation_speed` | `f32` | Slerp speed for aligning rotation to wall normal (default: `10.0`). |
| `hand_offset` | `f32` | Vertical offset to align visual hands with the ledge edge (default: `0.2`). |
| `hold_on_ledge_offset` | `Vec3` | Fine-tuning offset for the hang position. |
| `climb_ledge_speed_first_person` | `f32` | climbing speed when in FP mode. |

#### Input & Logic
| Field | Type | Description |
| :--- | :--- | :--- |
| `only_grab_ledge_if_moving_forward` | `bool` | Prevents accidental grabs when backing up. |
| `can_jump_when_hold_ledge` | `bool` | Allows wall jumping. |
| `jump_force_when_hold_ledge` | `f32` | Ejection force for wall jumps. |
| `auto_climb_in_third_person` | `bool` | If true, automatically mounts ledges without jump button. |

#### Debug State (Read-Only)
These fields are useful for debugging in the inspector but shouldn't be set manually:
-   `ledge_zone_found`: Is the raycast hitting a valid `LedgeZone`?
-   `climbing_ledge`: Are we currently attached?
-   `surface_to_hang_on_ground_found`: Did the low-raycast find a drop-down ledge?

### 2. `ClimbStateTracker`
Tracks dynamic runtime data.

```rust
#[derive(Component, Debug, Reflect)]
pub struct ClimbStateTracker {
    pub current_state: ClimbState,
    pub previous_state: ClimbState,
    pub state_timer: f32,       // Time in current state (useful for anims)
    pub stamina: f32,           // Current energy (0-100)
    pub max_stamina: f32,
    pub stamina_drain_rate: f32,
    pub is_stamina_depleted: bool,
}
```

### 3. `LedgeZone`
A marker component for `Trigger` volumes or specific collider entities that overrides climb behavior.

-   **Usage**: Add to a transparent cube at the top of a wall to define a specific climbable area.
-   **Override**: settings in `LedgeZone` override the global raycast settings while the player is inside the zone.

```rust
#[derive(Component, Debug, Reflect)]
pub struct LedgeZone {
    pub ledge_zone_active: bool,
    pub ledge_zone_can_be_climbed: bool, // Set false to make "unclimbable" paint
    pub custom_climb_speed: Option<f32>, // (Future Feature)
}
```

### 4. `LedgeDetection`
A transient component updated every frame by the raycasting system.

```rust
#[derive(Component, Debug, Reflect)]
pub struct LedgeDetection {
    pub ledge_found: bool,
    pub ledge_position: Vec3, // World space point of the edge
    pub ledge_normal: Vec3,   // Normal of the wall face
    pub surface_type: SurfaceType, // Material (Wood, Stone, Metal)
}
```

---

## Configuration Guide

### Setting up a Player
To enable climbing on a character:

1.  Add `ClimbLedgeSystem::default()`.
2.  Add `ClimbStateTracker::default()`.
3.  Add `LedgeDetection::default()` (required for the system to write data).
4.  Ensure `CharacterController` is present (the systems read `character.is_dead` etc).

### Tuning Raycasts
If your character isn't grabbing ledges:
1.  **Check Forward Ray**: Increase `climb_ledge_ray_forward_distance`. The character's collider radius might keep them too far from the wall.
2.  **Check Down Ray**: `climb_ledge_ray_down_distance` must be long enough to reach from "Forward Ray Hit" down to the actual ledge lip.
3.  **Offsets**: Use `hand_offset` to fix "floating hands" or "hands inside wall" visual issues.

### First Person vs Third Person
The system supports both modes via flags in `ClimbLedgeSystem`:
-   `auto_climb_in_first_person`: specialized logic for FPS feels (often faster, less animation-heavy).
-   `has_to_look_at_ledge_position_on_first_person`: Requires the player to aim at the ledge to grab it, adding skill/realism.

---

## Ledge Detection Logic

The detection system runs in the `FixedUpdate` loop to ensure physics consistency. It primarily uses the `detect_ledge` system.

### The Dual-Ray Algorithm

To robustly detect a climbable edge (a "cliff" or "wall top") without relying on manual triggers, the system uses two raycasts:

1.  **Forward Wall Check**:
    -   **Origin**: Player's chest/head height.
    -   **Direction**: `PlayerTransform.forward`.
    -   **Distance**: `climb_ledge_ray_forward_distance`.
    -   **Purpose**: Finds the face of the wall. If nothing is hit, there is no wall to climb.

2.  **Downward Ledge Check**:
    -   **Origin**: A point slightly *forward* of the wall hit (offset by a small epsilon) and *above* the player.
    -   **Direction**: `Vec3::DOWN`.
    -   **Distance**: `climb_ledge_ray_down_distance`.
    -   **Purpose**: Finds the *top* of the wall.

**Success Condition**:
-   Ray 1 hits a wall (Normal is roughly horizontal).
-   Ray 2 hits a floor/top (Normal is roughly vertical/up).
-   The distance between Ray 2's hit point and the player's hands is within a valid range.

If successful, `LedgeDetection.ledge_position` is set to the hit point of Ray 2, which represents the exact edge coordinates for the hands to grab.

### Auto-Hang (`detect_ledge_below`)

This secondary system (`detect_ledge_below`) allows players to walk off a roof and automatically turn around to grab the ledge, or "drop down" onto a ledge below them.

-   **Trigger**: Player is airborne or near an edge.
-   **Logic**: Raycasts `DOWN` and `BACK`.
-   **Result**: If a valid ledge is found below, it triggers the `AutoHang` component, which smoothly interpolates the player's position to the ledge, rotating them 180 degrees to face the wall.

---

## Wall Movement & Physics

When in `ClimbState::ClimbingUp/Left/Right`, standard physics forces are modified.

### Shimmying (Horizontal)
-   **Movement**: The player is moved along the cross product of `LedgeNormal` and `Vec3::UP`.
-   **Check**: Before moving, a "probe" raycast checks if the new position also has a `Ledge`. If the wall ends, shimmying stops to prevent moving into thin air.

### Vaulting
If the player climbs *up* and there is no wall above (it's a floor), the state transitions to `Vaulting`.
-   **Vault**: A calculated force pushes the player up and forward.
-   **Completion**: Once the feet clear the edge, state returns to `None` (Grounded) via the character controller's ground detection.

### Jumping Off
When jumping from a ledge:
-   **ForceMode**: `Impulse` (instant velocity change).
-   **Direction**:
    -   **Input=None**: Jump `UP` + `BACK` (away from wall).
    -   **Input=Up**: Jump `UP` (reach higher).
    -   **Input=Left/Right**: Jump sideways (wall hop).
-   **Force Calculation**: `base_force * stamina_multiplier * surface_multiplier`.

---

## Surface Types

Not all walls are equal. The `SurfaceType` enum allows different gameplay properties based on the material detected by the raycast (e.g., via Collider material or tag).

```rust
#[derive(Debug, Clone, Copy, PartialEq, Reflect, Default)]
pub enum SurfaceType {
    #[default] Default,
    Stone,   // Default speed
    Wood,    // 0.9x speed (rough)
    Metal,   // 0.8x speed (slippery)
    Ice,     // 1.2x speed (fast slide) / High stamina drain
    Rope,    // 0.7x speed
    Custom(f32),
}
```

### Impact on Gameplay
1.  **Climb Speed**: `calculate_climb_speed` applies the multiplier to movement.
2.  **Stamina Cost**: `calculate_stamina_cost` scales effort (e.g., Ice drains stamina faster).
3.  **Jump Force**: Some surfaces might provide less push-off force.
4.  **Sound**: Triggers different foley sounds (Wood creak vs Stone scrape).

---

## Animation Integration

The system includes helper components for syncing movement with animations, particularly useful for **Target Matching** (aligning the hand bone exactly with the ledge).

### `ClimbAnimation` Component
Used to provide data to your Animation Graph (e.g., `bevy_animation_graph` or standard Bevy animator).

-   **`match_start_value` / `match_end_value`**: Normalized time ranges (0.0 to 1.0) within an animation clip where physical position correction should occur.
-   **`match_mask_value`**: Vector mask defining which axes to match (e.g., `Vec3::Y` for vertical alignment).

### Implementation Pattern
1.  **State Change**: When `ClimbState` switches to `Vaulting`, trigger the "Vault" animation.
2.  **Target Set**: Set `LedgeDetection.ledge_position` as the Target Match point.
3.  **Update Loop**: During the animation frame, interpolate the Character's root transform so that at `match_end_value`, the hand/foot is exactly at the `ledge_position`.

This prevents "clipping" where the character's hands go inside the mesh or hover above it.

---

## Stamina System

Climbing is physically demanding. The `ClimbStateTracker` manages a stamina pool to prevent infinite climbing (unless configured otherwise).

```rust
pub struct ClimbStateTracker {
    pub stamina: f32,           // Current: 0.0 - 100.0
    pub max_stamina: f32,       // Cap: 100.0
    pub stamina_drain_rate: f32,// Per second cost
    pub stamina_regen_rate: f32,// Per second recovery (Grounded only)
}
```

### Mechanics
-   **Drain**: Occurs when `ClimbState` is *not* `None`.
-   **Fast Drain**: Fast movements (Vault, Jump) cost flat amounts of stamina.
-   **Regen**: Occurs only when `ClimbState == None` (Grounded).
-   **Exhaustion**:
    -   When `stamina <= 0.0`: `is_stamina_depleted` becomes true.
    -   Effect: The `update_climb_state` system forces a transition to `ClimbState::Falling`, causing the player to let go of the wall.
    -   Recovery: Player cannot grab a wall again until stamina recovers past specific threshold (e.g., 10%).

---

## Troubleshooting

### 1. "Player walks into wall instead of climbing"
-   **Cause**: Forward raycast is too short.
-   **Fix**: Increase `climb_ledge_ray_forward_distance`. It must be > `Collider Radius + Skin Width`.

### 2. "Player grabs ledge but hangs inside the geometry"
-   **Cause**: `ClimbLedgeSystem.hold_on_ledge_offset` is zero or incorrect.
-   **Fix**: Adjust the Z-offset in `hold_on_ledge_offset` to push the model back from the wall.

### 3. "Player grabs the air above the wall"
-   **Cause**: `ClimbLedgeSystem.hand_offset` incorrect.
-   **Fix**: Lower the `hand_offset` value. This value represents the distance from the "Ray Hit (Ledge Lip)" to the "Hand Bone".

### 4. "Cannot climb specific walls"
-   **Cause**: `layer_mask_to_check` mismatch.
-   **Fix**: Ensure your scene walls are on the Physics Layer defined in the config. By default, it might only check Layer 1 (Static Environment).

### 5. "Character jitters while hanging"
-   **Cause**: Conflict with Standard Controller or Physics Solver.
-   **Fix**: Ensure the system correctly sets `RigidBody::Kinematic` or disables Gravity during `ClimbState::Hanging`. The `update_climb_movement` system should take full control of the Transform/Velocity.

## Future Roadmap
- [ ] **Curved Ledges**: Better handling of non-linear surfaces (currently optimized for straight box-like geometry).
- [ ] **Corner Rounding**: Automatically transitioning from a North face to an East face of a building.
- [ ] **Dynmaic Objects**: Climbing on moving platforms (requires parent-local coordinate space updates).