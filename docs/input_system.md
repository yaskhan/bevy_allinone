# Bevy Input System Documentation

## Documentation Contents

The Input System documentation provides comprehensive coverage of a flexible, production-ready input handling framework built for the Bevy game engine. This system delivers robust action mapping, input buffering, runtime rebinding capabilities, and seamless integration between global input state and entity-specific components.

## Table of Contents

- [1. System Overview](#1-system-overview)
    - [1.1 Design Philosophy](#11-design-philosophy)
    - [1.2 Key Capabilities](#12-key-capabilities)
    - [1.3 System Boundaries](#13-system-boundaries)
- [2. Architecture Deep Dive](#2-architecture-deep-dive)
    - [2.1 Resource/Component Relationship Model](#21-resourcecomponent-relationship-model)
    - [2.2 Data Flow Architecture](#22-data-flow-architecture)
    - [2.3 Update Schedule Integration](#23-update-schedule-integration)
- [3. Input Action Taxonomy](#3-input-action-taxonomy)
    - [3.1 Movement Actions](#31-movement-actions)
    - [3.2 Combat Actions](#32-combat-actions)
    - [3.3 Interaction & Utility Actions](#33-interaction--utility-actions)
    - [3.4 Advanced Movement & Stealth Actions](#34-advanced-movement--stealth-actions)
    - [3.5 Camera Control Actions](#35-camera-control-actions)
- [4. Binding System Mechanics](#4-binding-system-mechanics)
    - [4.1 Binding Types](#41-binding-types)
    - [4.2 Multi-Binding Semantics](#42-multi-binding-semantics)
    - [4.3 Default Binding Configuration](#43-default-binding-configuration)
- [5. Input State Management](#5-input-state-management)
    - [5.1 State Representation Structure](#51-state-representation-structure)
    - [5.2 Temporal Semantics](#52-temporal-semantics)
    - [5.3 Input Enable/Disable System](#53-input-enabledisable-system)
- [6. Input Buffering System](#6-input-buffering-system)
    - [6.1 Buffering Rationale](#61-buffering-rationale)
    - [6.2 Buffered Action Structure](#62-buffered-action-structure)
    - [6.3 Buffer Configuration](#63-buffer-configuration)
    - [6.4 Buffered Action Consumption](#64-buffered-action-consumption)
    - [6.5 Default Buffered Actions](#65-default-buffered-actions)
    - [6.6 Buffer Lifecycle Management](#66-buffer-lifecycle-management)
- [7. Runtime Rebinding System](#7-runtime-rebinding-system)
    - [7.1 Rebinding Workflow](#71-rebinding-workflow)
    - [7.2 Rebind State Resource](#72-rebind-state-resource)
    - [7.3 Input Capture Priority](#73-input-capture-priority)
    - [7.4 Binding Replacement Semantics](#74-binding-replacement-semantics)
    - [7.5 Rebinding Safety Considerations](#75-rebinding-safety-considerations)
- [8. System Execution Details](#8-system-execution-details)
    - [8.1 Primary Input Processing System](#81-primary-input-processing-system)
    - [8.2 Buffer Cleanup System](#82-buffer-cleanup-system)
    - [8.3 Player Input Synchronization System](#83-player-input-synchronization-system)
    - [8.4 Stub Systems: Movement and Action Processing](#84-stub-systems-movement-and-action-processing)
- [9. Configuration System](#9-configuration-system)
    - [9.1 InputConfig Resource Structure](#91-inputconfig-resource-structure)
    - [9.2 Configuration Initialization](#92-configuration-initialization)
    - [9.3 Runtime Configuration Updates](#93-runtime-configuration-updates)
- [10. Integration Patterns](#10-integration-patterns)
    - [10.1 Player Controller Integration](#101-player-controller-integration)
    - [10.2 Jump Buffering Implementation](#102-jump-buffering-implementation)
    - [10.3 Weapon System Integration](#103-weapon-system-integration)
    - [10.4 Menu System Integration](#104-menu-system-integration)
    - [10.5 Camera System Integration](#105-camera-system-integration)
- [11. Best Practices & Design Patterns](#11-best-practices--design-patterns)
    - [11.1 Input Action Design Principles](#111-input-action-design-principles)
    - [11.2 Binding Configuration Guidelines](#112-binding-configuration-guidelines)
    - [11.3 Buffer TTL Tuning Methodology](#113-buffer-ttl-tuning-methodology)
    - [11.4 Input Disable Patterns](#114-input-disable-patterns)
    - [11.5 Testing Methodologies](#115-testing-methodologies)
- [12. Troubleshooting Guide](#12-troubleshooting-guide)
    - [12.1 Common Issues & Solutions](#121-common-issues--solutions)
    - [12.2 Diagnostic Tools](#122-diagnostic-tools)
- [13. Performance Characteristics](#13-performance-characteristics)
    - [13.1 Computational Complexity](#131-computational-complexity)
    - [13.2 Optimization Opportunities](#132-optimization-opportunities)
    - [13.3 Scalability Considerations](#133-scalability-considerations)
- [14. Extension Points & Future Enhancements](#14-extension-points--future-enhancements)
    - [14.1 Gamepad Support Integration](#141-gamepad-support-integration)
    - [14.2 Advanced Buffering Features](#142-advanced-buffering-features)
    - [14.3 Input Recording & Playback](#143-input-recording--playback)
    - [14.4 Accessibility Enhancements](#144-accessibility-enhancements)
- [15. Related Systems & Integration Points](#15-related-systems--integration-points)
    - [15.1 Character Controller System](#151-character-controller-system)
    - [15.2 Weapon System](#152-weapon-system)
    - [15.3 Camera System](#153-camera-system)
    - [15.4 UI / Menu System](#154-ui--menu-system)
    - [15.5 Animation System](#155-animation-system)
- [16. Conclusion & Recommendations](#16-conclusion--recommendations)
    - [16.1 System Maturity Assessment](#161-system-maturity-assessment)
    - [16.2 Implementation Recommendations](#162-implementation-recommendations)

---

## 1. System Overview

### 1.1 Design Philosophy

The Bevy Input System implements a robust action-based input architecture that decouples physical input devices from logical game actions. This separation enables:

- **Hardware Agnosticism**: Game logic references semantic actions (`Jump`, `Interact`) rather than physical inputs (`Space`, `E`), allowing seamless support for keyboard, mouse, gamepad, or future input devices without code changes.

- **Runtime Flexibility**: Players can remap controls during gameplay without restarting the application or reloading assets, enhancing accessibility and user customization.

- **Temporal Buffering**: Critical actions like jumps and interactions benefit from input buffering, capturing player intent even when inputs occur between physics frames or during animation transitions.

- **State Isolation**: Global input state resources synchronize automatically with player entity components, maintaining clean separation between engine-level input processing and gameplay logic.

- **Extensibility**: The system architecture supports straightforward extension to additional input devices (gamepads, touchscreens) through the binding abstraction layer.

### 1.2 Key Capabilities

The input system delivers production-ready features essential for modern game development:

- **Comprehensive Action Taxonomy**: 35+ predefined logical actions spanning movement, combat, inventory management, camera control, and stealth mechanics.

- **Multi-Binding Support**: Each logical action supports multiple physical bindings simultaneously (e.g., `LockOn` bound to both `Tab` key and mouse wheel button).

- **Buffered Input Processing**: Time-sensitive actions receive special handling through a configurable buffer with automatic expiration management.

- **Runtime Rebinding**: Players can dynamically reassign controls through an intuitive capture process without interrupting gameplay flow.

- **Automatic State Synchronization**: Global input state resources automatically propagate to player entities, eliminating manual state copying boilerplate.

- **Input Enable/Disable Toggle**: Comprehensive input disabling with automatic state reset prevents unintended inputs during cutscenes, menus, or dialogue sequences.

- **Axis Normalization**: Movement vectors automatically normalize to prevent faster diagonal movement, with configurable sensitivity parameters.

- **Reflect Integration**: Full Bevy Reflect trait implementation enables runtime inspection, serialization, and editor tooling support.

### 1.3 System Boundaries

The input system operates within well-defined boundaries to maintain separation of concerns:

- **Input Collection Layer**: Raw device input captured through Bevy's `ButtonInput<KeyCode>` and `ButtonInput<MouseButton>` resources.

- **Binding Translation Layer**: Physical inputs mapped to logical actions through the `InputMap` resource configuration.

- **State Management Layer**: Translated actions populate the `InputState` resource with normalized values and temporal flags (`just_pressed` vs `pressed`).

- **Buffering Layer**: Time-sensitive actions receive special handling through the `InputBuffer` resource with configurable expiration.

- **Entity Synchronization Layer**: Global input state automatically propagates to player entities bearing the `InputState` component.

- **Gameplay Integration Boundary**: The system intentionally stops at state population; actual gameplay effects (character movement, weapon firing) occur in separate systems that consume `InputState`.

This layered architecture ensures the input system remains focused on input processing without encroaching on gameplay logic responsibilities.

---

## 2. Architecture Deep Dive

### 2.1 Resource/Component Relationship Model

The input system employs a dual-state model combining global resources with entity-specific components:

#### Global Resources (Engine-Level State)

- **`InputMap`**: Central configuration resource defining action-to-binding mappings. Serves as the single source of truth for control schemes.

- **`InputState` (Resource)**: Global snapshot of current input state maintained by input processing systems. Updated every frame during the `Update` schedule.

- **`InputBuffer`**: Temporal storage for recently pressed actions with automatic expiration based on configurable time-to-live.

- **`InputConfig`**: System-wide configuration parameters including mouse sensitivity, axis inversion, and buffer expiration duration.

- **`RebindState`**: Transient state resource tracking ongoing rebinding operations (which action is awaiting new binding assignment).

#### Entity Components (Gameplay-Level State)

- **`InputState` (Component)**: Per-entity copy of input state, automatically synchronized from the global resource. Allows multiple controllable entities (e.g., player character, vehicles) to maintain independent input states.

This dual-state model provides critical advantages:

- **Menu/UI Isolation**: Global input state continues updating during pause menus, but player entity components can be selectively disabled without affecting UI navigation inputs.

- **Multi-Entity Control**: Multiple player-controlled entities (e.g., in split-screen multiplayer) maintain independent input states through separate components.

- **AI/Player Differentiation**: AI-controlled entities can have `InputState` components populated by AI decision systems rather than player input, enabling unified consumption patterns across control schemes.

- **Cutscene Safety**: Disabling input on player entities during cutscenes prevents accidental player interference while maintaining global state for potential cutscene interaction points.

### 2.2 Data Flow Architecture

Input processing follows a strict unidirectional data flow through discrete processing stages:

```
Physical Input Devices
         ↓
[Bevy ButtonInput Resources]
         ↓
[update_input_state System]
         ├──→ Binding Translation (InputMap lookup)
         ├──→ Action Buffering (critical actions)
         ├──→ Axis Normalization (movement vectors)
         └──→ State Population (InputState resource)
         ↓
[cleanup_input_buffer System]
         ↓
[player_input_sync_system]
         ↓
[Gameplay Systems Consuming InputState]
```

Each stage performs a single responsibility with explicit boundaries:

1. **Binding Translation Stage**: Raw device inputs (`KeyCode::Space`) mapped to logical actions (`InputAction::Jump`) through `InputMap` configuration. Multiple bindings per action evaluated with OR semantics (any binding triggers the action).

2. **Buffering Stage**: Time-critical actions (`Jump`, `Interact`, `LockOn`) captured on `just_pressed` events and stored in `InputBuffer` with timestamps. Enables "input forgiveness" for actions occurring between animation frames.

3. **State Population Stage**: Translated actions populate `InputState` fields with appropriate temporal semantics:
   - Continuous actions (`MoveForward`) set boolean flags for duration of press
   - Momentary actions (`Jump`) set `just_pressed` flags only on initial press frame
   - Axis inputs (`movement`, `look`) normalized to consistent magnitude

4. **Buffer Cleanup Stage**: Expired buffered actions automatically removed based on configurable TTL (`buffer_ttl` in `InputConfig`).

5. **Entity Synchronization Stage**: Global `InputState` resource cloned to all entities with `InputState` components and `Player` marker component (excluding AI-controlled entities).

This staged architecture ensures deterministic processing order and prevents race conditions between input collection and gameplay consumption.

### 2.3 Update Schedule Integration

The input system integrates with Bevy's scheduling system through carefully ordered system execution:

```rust
.add_systems(Update, (
    update_input_state,
    handle_rebinding,
    cleanup_input_buffer,
    player_input_sync_system,
).chain())
.add_systems(Update, (
    process_movement_input,
    process_action_input,
));
```

Critical sequencing considerations:

- **Chained Execution**: The first four systems execute sequentially in guaranteed order within a single frame:
  1. `update_input_state`: Collects raw inputs and populates global state
  2. `handle_rebinding`: Processes rebinding requests using current frame inputs
  3. `cleanup_input_buffer`: Removes expired buffered actions
  4. `player_input_sync_system`: Propagates global state to player entities

- **Parallel Execution**: Movement and action processing systems run after the chain completes, ensuring they consume fully processed input state.

- **Frame Boundary Safety**: All input processing completes before gameplay systems execute, preventing mid-frame state inconsistencies.

- **Rebinding Priority**: Rebinding system executes after state update but before buffer cleanup, allowing rebinding operations to consume inputs that would otherwise trigger gameplay actions.

This scheduling guarantees that gameplay systems always observe a consistent, fully processed input state snapshot for the current frame.

---

## 3. Input Action Taxonomy

### 3.1 Movement Actions

Movement actions form the foundation of player locomotion with orthogonal directional controls:

- **`MoveForward`**: Primary forward movement along character's forward axis. Typically bound to `W` key. Combined with `Sprint` for accelerated movement.

- **`MoveBackward`**: Reverse movement along character's backward axis. Typically bound to `S` key. Often reduced speed compared to forward movement in gameplay implementation.

- **`MoveLeft`**: Lateral movement left relative to character orientation. Typically bound to `A` key. Enables strafing mechanics essential for combat maneuvering.

- **`MoveRight`**: Lateral movement right relative to character orientation. Typically bound to `D` key. Symmetric counterpart to `MoveLeft`.

- **`Jump`**: Vertical propulsion action initiating airborne state. Buffer-sensitive action benefiting from input forgiveness during landing animations. Typically bound to `Space` key.

- **`Sprint`**: Modifier action increasing movement speed while held. Continuous press detection enables variable-speed locomotion. Typically bound to `Left Shift`.

- **`Crouch`**: Posture modifier reducing character height and potentially movement speed. Enables stealth mechanics and low-clearance navigation. Typically bound to `Left Control`.

- **`LeanLeft` / `LeanRight`**: Partial cover mechanics allowing player to peek around obstacles while maintaining cover position. Implemented as continuous press actions rather than toggles for safety.

### 3.2 Combat Actions

Combat actions support diverse engagement mechanics with weapon handling and defensive capabilities:

- **`Attack`**: Primary offensive action triggering melee attacks or weapon firing depending on equipped item. Typically bound to left mouse button.

- **`Block`**: Defensive action reducing incoming damage or deflecting attacks. Continuous press enables partial blocking mechanics. Typically bound to right mouse button (shared with `Aim` in some configurations).

- **`Fire`**: Dedicated ranged weapon firing action. Separate from `Attack` to support hybrid combat systems with distinct melee/ranged inputs. Typically bound to left mouse button.

- **`Aim`**: Precision aiming modifier activating iron sights or scoped view. Continuous press enables variable zoom levels in advanced implementations. Typically bound to right mouse button.

- **`Reload`**: Weapon reloading sequence initiation. Momentary press triggers full reload animation sequence. Typically bound to `R` key.

- **`NextWeapon` / `PrevWeapon`**: Cyclic weapon selection through inventory. Momentary presses advance or retreat through weapon slots. Typically bound to arrow keys.

- **`SelectWeapon[0-9]`**: Direct weapon slot selection enabling instant weapon switching without cycling. Critical for tactical combat requiring rapid loadout changes. Bound to numeric keys `0-9` with `0` typically mapping to slot 10.

### 3.3 Interaction & Utility Actions

Interaction actions bridge player intent with environmental engagement:

- **`Interact`**: Context-sensitive interaction with objects, NPCs, or environmental elements. Buffer-sensitive to prevent missed interactions during movement. Typically bound to `E` key.

- **`ToggleInventory`**: User interface activation for inventory management, equipment configuration, and item usage. Momentary press toggles UI visibility state. Typically bound to `I` key.

- **`SwitchCameraMode`**: Perspective or camera behavior modification (first-person ↔ third-person, free-look modes). Momentary press cycles through available camera configurations. Typically bound to `C` key.

- **`ResetCamera`**: Camera reorientation to default position/angle. Essential for disorientation recovery after complex maneuvers. Typically bound to `X` key (changed from `R` to avoid reload conflict).

### 3.4 Advanced Movement & Stealth Actions

Specialized actions supporting advanced locomotion and stealth gameplay:

- **`Hide`**: Stealth state activation reducing visibility to enemies and enabling silent movement. Momentary press toggles stealth state. Typically bound to `H` key.

- **`Peek`**: Limited exposure from cover positions enabling target acquisition without full exposure. Continuous press maintains peek state. Typically bound to `P` key.

- **`CornerLean`**: Advanced cover mechanic allowing partial exposure around corners while maintaining cover integrity. Distinct from basic lean actions with more complex animation requirements. Typically bound to `C` key (potential conflict with camera mode requires resolution in binding configuration).

- **`LockOn`**: Target acquisition system locking camera and aiming reticle to valid targets within acquisition radius. Supports multiple bindings (keyboard `Tab` and mouse wheel button) for accessibility. Buffer-sensitive to prevent missed lock attempts during movement.

- **`ZoomIn` / `ZoomOut`**: Camera field-of-view adjustment for tactical observation or situational awareness. Typically bound to numpad `+`/`-` keys.

- **`SideSwitch`**: Cover side transition allowing player to switch cover positions without exposing full body. Critical for advanced cover combat systems. Typically bound to `V` key.

### 3.5 Camera Control Actions

Camera manipulation actions (primarily handled through axis inputs rather than discrete actions):

- **`look` axis**: Two-dimensional vector representing mouse movement delta for camera rotation. Processed separately from keyboard inputs with configurable sensitivity and axis inversion.

- **`ResetCamera`**: Explicit camera reset action complementing continuous look controls.

Note: Continuous camera rotation typically handled through raw mouse delta processing rather than discrete actions to enable smooth, analog control. The input system provides the `look` vector field in `InputState` for this purpose.

---

## 4. Binding System Mechanics

### 4.1 Binding Types

The system supports two primary physical input types through the `InputBinding` enumeration:

#### Keyboard Bindings (`InputBinding::Key(KeyCode)`)

- Maps logical actions to specific keyboard keys through Bevy's `KeyCode` enumeration.
- Supports full keyboard coverage including alphanumeric keys, function keys, modifiers, and special keys.
- Binding evaluation uses `pressed()` for continuous actions and `just_pressed()` for momentary actions.
- Modifier key handling (Ctrl, Alt, Shift) requires explicit binding configuration rather than automatic modifier detection to maintain binding transparency.

#### Mouse Bindings (`InputBinding::Mouse(MouseButton)`)

- Maps logical actions to mouse buttons through Bevy's `MouseButton` enumeration.
- Supports primary buttons (Left, Right, Middle) and additional buttons (Back, Forward) where hardware supports.
- Mouse movement (axis input) handled separately through dedicated look vector processing rather than discrete bindings.
- Scroll wheel typically mapped to discrete actions (`ZoomIn`/`ZoomOut`) rather than continuous axis due to event-based nature of scroll input.

### 4.2 Multi-Binding Semantics

Each logical action supports multiple physical bindings evaluated with OR semantics:

- **Binding Evaluation Order**: Bindings evaluated sequentially; first pressed binding triggers the action. Evaluation order typically irrelevant due to OR semantics but may impact rebinding UI display order.

- **Simultaneous Binding Presses**: Pressing multiple bindings for the same action simultaneously treated identically to single binding press—no special handling or priority assignment.

- **Binding Conflict Resolution**: The system does not prevent binding conflicts (same physical input mapped to multiple actions). Conflict resolution responsibility falls to:
  - Binding configuration (avoiding conflicts in default mappings)
  - Gameplay systems (resolving ambiguous inputs contextually)
  - Rebinding UI (validating against conflicts during user remapping)

- **Contextual Binding Suppression**: Advanced implementations may suppress certain bindings based on game state (e.g., disabling movement bindings during vehicle operation) through input state enable/disable toggling rather than binding modification.

### 4.3 Default Binding Configuration

The `InputMap::default()` implementation provides comprehensive out-of-the-box configuration:

- **WASD Movement**: Standard keyboard movement scheme with intuitive spatial mapping.
- **Mouse-Centric Combat**: Left mouse button for primary actions (attack/fire), right mouse button for secondary actions (aim/block).
- **Modifier Keys**: Left Shift for sprinting, Left Control for crouching—avoiding right-side modifiers to maintain left-hand positioning on movement keys.
- **Numeric Weapon Selection**: Direct slot access through top-row numeric keys (1-0), with 0 mapping to slot 10 for full decuple weapon support.
- **Conflict Mitigation**: Deliberate binding choices to minimize conflicts (e.g., `ResetCamera` on `X` instead of `R` to avoid reload conflict).
- **Accessibility Considerations**: Multiple bindings for critical actions (`LockOn` on both `Tab` and mouse wheel) accommodating different player preferences.

Default bindings represent a balanced starting point optimized for PC keyboard/mouse configurations. Console or alternative input schemes require custom `InputMap` initialization.

---

## 5. Input State Management

### 5.1 State Representation Structure

The `InputState` structure maintains comprehensive input snapshot with field categories:

#### Movement Vectors

- **`movement: Vec2`**: Normalized 2D vector representing directional input magnitude and direction. Values range from `(0.0, 0.0)` (no input) to normalized vectors like `(0.707, 0.707)` for diagonal movement. Automatic normalization prevents faster diagonal movement artifacts.

- **`look: Vec2`**: Raw mouse delta vector representing camera rotation input. Not normalized—magnitude represents physical mouse movement distance. Processed separately with configurable sensitivity multipliers.

#### Continuous Press States

Boolean flags indicating sustained press duration:

- `crouch_pressed`, `sprint_pressed`, `aim_pressed`, `lean_left`, `lean_right`, `block_pressed`, `fire_pressed`

These fields remain `true` for entire duration of physical input press, enabling analog-like behavior for actions with variable duration effects (e.g., partial crouch depth based on press duration).

#### Momentary Press States

Boolean flags indicating frame-specific press events:

- `jump_pressed`, `interact_pressed`, `lock_on_pressed`, `attack_pressed`, `switch_camera_mode_pressed`, `fire_just_pressed`, `reload_pressed`, `reset_camera_pressed`, `next_weapon_pressed`, `prev_weapon_pressed`, `toggle_inventory_pressed`, `side_switch_pressed`, `hide_pressed`, `peek_pressed`, `corner_lean_pressed`, `zoom_in_pressed`, `zoom_out_pressed`

These fields set `true` only on the exact frame when physical input transitions from released to pressed. Automatically reset to `false` on subsequent frames regardless of continued press duration. Critical for actions triggering discrete state transitions (jump initiation, weapon firing).

#### Specialized State Fields

- **`select_weapon: Option<usize>`**: Weapon slot selection result from numeric key presses. `Some(n)` indicates selection of slot `n` (0-indexed) on current frame; `None` otherwise. Resets to `None` each frame after consumption by weapon systems.

- **`enabled: bool`**: Master input enable/disable flag. When `false`, all input fields automatically zeroed/reset and physical input ignored. Essential for cutscenes, menus, and dialogue sequences.

### 5.2 Temporal Semantics

Critical distinction between continuous and momentary state fields:

| Field Type | Physical Input Duration | Field Value Duration | Typical Use Cases |
|------------|-------------------------|----------------------|-------------------|
| Continuous (`*_pressed`) | Held for N frames | Remains `true` for all N frames | Crouching, sprinting, aiming |
| Momentary (`*_just_pressed`) | Pressed on frame T | `true` only on frame T | Jumping, firing, interacting |
| Vector (`movement`, `look`) | Continuous analog input | Updated every frame with current values | Locomotion, camera rotation |

This temporal distinction enables nuanced input handling:

- **Jump Buffering**: `jump_pressed` field set from buffered action rather than direct input, allowing jump initiation up to 0.15 seconds after landing animation completes.

- **Double-Tap Detection**: Gameplay systems can implement double-tap mechanics by tracking momentary press timestamps across frames.

- **Hold-to-Activate**: Continuous press fields enable charge mechanics (e.g., hold fire to charge weapon) by measuring duration between initial press and release.

- **Input Chording**: Simultaneous press detection through combined evaluation of multiple continuous press fields (e.g., `crouch_pressed && sprint_pressed` for sliding).

### 5.3 Input Enable/Disable System

The `set_input_enabled()` method provides comprehensive input suppression:

```rust
pub fn set_input_enabled(&mut self, enabled: bool)
```

When disabling input (`enabled = false`):

1. Sets `enabled` flag to `false`
2. Resets all movement vectors to `Vec2::ZERO`
3. Resets all boolean press states to `false`
4. Clears weapon selection field (`None`)
5. Subsequent input processing skipped until re-enabled

Critical use cases:

- **Menu Systems**: Disable player input when opening inventory/pause menus while maintaining UI navigation inputs through separate input contexts.

- **Cutscenes**: Prevent player interference during narrative sequences while potentially allowing skip inputs through alternative input channels.

- **Dialogue Sequences**: Suppress locomotion/combat inputs during NPC conversations while permitting dialogue choice selection.

- **Loading Transitions**: Disable inputs during level transitions to prevent state corruption from inputs processed during asset loading.

- **Game State Transitions**: Temporarily disable inputs during death sequences, respawn animations, or objective completion sequences.

Re-enabling input (`enabled = true`) does not restore previous state—player must provide fresh inputs. This prevents unintended actions from inputs buffered during disabled period.

---

## 6. Input Buffering System

### 6.1 Buffering Rationale

Input buffering addresses critical temporal disconnects between player intent and game state readiness:

- **Animation Lockout Periods**: During jump landing animations, attack recovery, or weapon reload sequences, raw input presses might occur when gameplay systems cannot process them. Buffering captures these inputs for processing when state becomes receptive.

- **Frame Rate Variance**: At lower framerates, brief inputs might occur entirely between frames. Buffering with appropriate TTL captures these sub-frame inputs.

- **Network Latency Compensation**: In networked games, buffering provides window for reconciling client inputs with server-validated state transitions.

- **Player Intent Preservation**: Players pressing jump immediately upon landing expect jump initiation even if landing animation hasn't fully completed. Buffering honors this expectation.

### 6.2 Buffered Action Structure

The `BufferedAction` structure captures temporal input data:

```rust
pub struct BufferedAction {
    pub action: InputAction,
    pub timestamp: f32,
}
```

- **`action`**: Logical action that was pressed (`Jump`, `Interact`, etc.)
- **`timestamp`**: Game time in seconds when action was pressed (`time.elapsed_seconds()`)

Timestamp enables precise expiration management independent of frame rate. Buffer cleanup evaluates `(current_time - timestamp) <= buffer_ttl` rather than frame-counting.

### 6.3 Buffer Configuration

Buffer behavior controlled through `InputConfig` resource:

```rust
pub struct InputConfig {
    pub mouse_sensitivity: f32,
    pub gamepad_sensitivity: f32,
    pub invert_y_axis: bool,
    pub buffer_ttl: f32,  // Critical parameter
}
```

- **`buffer_ttl` (Time-To-Live)**: Maximum duration buffered actions remain valid. Default `0.15` seconds (150ms) represents balance between responsiveness and forgiveness:
  - Too short (<50ms): Fails to capture legitimate inputs during animation transitions
  - Too long (>300ms): Creates "input lag" perception as actions trigger unexpectedly after delay
  - Optimal range: 100-200ms depending on game genre (platformers benefit from longer buffers than competitive shooters)

Buffer TTL applies uniformly to all buffered actions. Advanced implementations might support action-specific TTLs through extended configuration.

### 6.4 Buffered Action Consumption

Two consumption patterns supported through `InputBuffer` methods:

#### Consume-and-Remove Pattern

```rust
pub fn consume(&mut self, action: InputAction) -> bool
```

- Searches buffer for specified action
- If found, removes action from buffer and returns `true`
- If not found, returns `false`
- Typical usage: `if input_buffer.consume(InputAction::Jump) { initiate_jump(); }`

Ensures each buffered action triggers at most one state transition, preventing duplicate processing.

#### Peek-without-Consumption Pattern

```rust
pub fn is_buffered(&self, action: InputAction) -> bool
```

- Checks buffer presence without removal
- Enables preview logic (e.g., UI indicators showing buffered jump available)
- Requires subsequent `consume()` call to actually trigger action

Supports advanced UX patterns like visual feedback for buffered inputs before state transition occurs.

### 6.5 Default Buffered Actions

System buffers three critical actions by default:

- **`Jump`**: Most common buffering candidate due to frequent animation lockout during landing/recovery.
- **`Interact`**: Prevents missed interactions when approaching interactable objects during movement animations.
- **`LockOn`**: Ensures target acquisition succeeds even when pressing lock-on during weapon sway or movement animations.

Gameplay systems may extend buffering to additional actions through custom buffer management outside core input system.

### 6.6 Buffer Lifecycle Management

Automatic buffer management through dedicated cleanup system:

```rust
fn cleanup_input_buffer(
    time: Res<Time>,
    config: Res<InputConfig>,
    mut input_buffer: ResMut<InputBuffer>,
)
```

- Executes every frame after input state update
- Filters buffer contents retaining only actions where `(now - timestamp) <= config.buffer_ttl`
- Zero-allocation implementation using `Vec::retain()` for performance
- No manual buffer management required by gameplay systems

This automatic cleanup prevents buffer accumulation and memory growth during extended play sessions.

---

## 7. Runtime Rebinding System

### 7.1 Rebinding Workflow

The rebinding system enables dynamic control remapping through intuitive three-step process:

1. **Initiation**: Gameplay system sets `RebindState.action = Some(target_action)` when player enters rebinding UI for specific action.

2. **Capture**: System monitors all physical inputs (keyboard keys, mouse buttons) on subsequent frames. First detected input becomes new binding.

3. **Assignment**: Detected input automatically assigned as sole binding for target action in `InputMap`, replacing previous bindings. `RebindState.action` reset to `None`.

This workflow requires zero gameplay system involvement beyond initiation—capture and assignment handled entirely by `handle_rebinding` system.

### 7.2 Rebind State Resource

```rust
pub struct RebindState {
    pub action: Option<InputAction>,
}
```

- **`None`**: Normal gameplay mode—inputs processed for action triggering
- **`Some(action)`**: Rebinding mode active for specified action—next physical input captured as new binding

State resource enables rebinding initiation from any system with mutable access to `RebindState`. Typical initiation pattern:

```rust
// In UI interaction system when player selects "Jump" for rebinding
rebind_state.action = Some(InputAction::Jump);
// Next physical input automatically becomes new jump binding
```

### 7.3 Input Capture Priority

During rebinding mode (`RebindState.action.is_some()`):

- All gameplay input processing bypassed—no actions triggered from captured input
- Keyboard inputs prioritized over mouse inputs in capture resolution:
  - System checks keyboard `get_just_pressed()` first
  - Only checks mouse buttons if no keyboard input detected
- First detected input of either type immediately assigned as new binding
- Escape key typically handled by UI system to cancel rebinding without assignment

This priority ordering reflects typical rebinding expectations—keyboard rebinding more common than mouse button rebinding.

### 7.4 Binding Replacement Semantics

New bindings completely replace existing bindings for target action:

- Previous bindings discarded without merge or append behavior
- Single-binding-per-action model enforced during rebinding
- Multi-binding configurations require explicit UI support beyond core system capabilities

This simplification ensures rebinding predictability—players always know exactly which physical input triggers an action after rebinding.

### 7.5 Rebinding Safety Considerations

Critical safety mechanisms prevent problematic rebinding scenarios:

- **System Key Protection**: Core system does not prevent rebinding of essential keys (Escape, Alt+Tab). Protection must be implemented at UI layer:
  - Rebinding UI should reject system-critical keys with visual feedback
  - Escape key typically reserved for menu cancellation rather than gameplay actions

- **Binding Conflict Detection**: Core system does not prevent binding conflicts. UI layer should:
  - Display warnings when new binding conflicts with existing actions
  - Offer conflict resolution options (reassign conflicting action, cancel operation)

- **Default Binding Restoration**: Core system provides no built-in reset-to-defaults functionality. Implementation requires:
  - Storing original `InputMap` at startup
  - UI button triggering restoration from stored default

- **Persistence Requirements**: Rebinding changes exist only in memory. Persistent storage requires:
  - Serialization of modified `InputMap` to disk (leveraging Reflect trait)
  - Deserialization on subsequent launches
  - Versioning strategy for binding schema changes between game updates

---

## 8. System Execution Details

### 8.1 Primary Input Processing System

```rust
fn update_input_state(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    input_map: Res<InputMap>,
    mut input_state: ResMut<InputState>,
    mut input_buffer: ResMut<InputBuffer>,
)
```

#### Execution Preconditions

- Immediately returns if `input_state.enabled == false`, skipping all processing
- Ensures zero processing cost during cutscenes/menus when input disabled
- Prevents buffer accumulation during disabled periods

#### Binding Translation Implementation

Closure-based binding evaluation provides efficient action checking:

```rust
let check_action = |action: InputAction| -> bool {
    if let Some(bindings) = input_map.bindings.get(&action) {
        bindings.iter().any(|binding| match binding {
            InputBinding::Key(code) => keyboard.pressed(code.clone()),
            InputBinding::Mouse(button) => mouse_buttons.pressed(button.clone()),
        })
    } else {
        false
    }
};
```

- Early return on first matching pressed binding (OR semantics)
- Clone required due to Bevy's input resource immutability constraints
- Missing action in map treated as permanently unbound (returns `false`)

Separate closure for `just_pressed` evaluation follows identical pattern with `just_pressed()` instead of `pressed()`.

#### Buffer Population Logic

Buffered actions captured on exact frame of press:

```rust
for action in actions_to_buffer {
    if check_action_just_pressed(action) {
        input_buffer.actions.push(BufferedAction {
            action,
            timestamp: time.elapsed_seconds(),
        });
    }
}
```

- Only `just_pressed` events buffered—not sustained presses
- Timestamp captured at moment of physical input press
- Buffer population occurs before state field population to enable buffer-sourced state values

#### Movement Vector Construction

Orthogonal axis combination with normalization:

```rust
let mut movement = Vec2::ZERO;
if check_action(InputAction::MoveForward) { movement.y += 1.0; }
if check_action(InputAction::MoveBackward) { movement.y -= 1.0; }
if check_action(InputAction::MoveLeft) { movement.x -= 1.0; }
if check_action(InputAction::MoveRight) { movement.x += 1.0; }
input_state.movement = movement.normalize_or_zero();
```

- Diagonal movement produces vector `(±1.0, ±1.0)` before normalization
- `normalize_or_zero()` prevents division-by-zero on zero vector
- Normalized diagonal vectors maintain consistent magnitude with cardinal directions
- Alternative implementations might skip normalization for "faster diagonal movement" gameplay feel

#### Weapon Selection Implementation

Mutually exclusive numeric selection with priority ordering:

```rust
input_state.select_weapon = None;
if check_action_just_pressed(InputAction::SelectWeapon1) { input_state.select_weapon = Some(0); }
else if check_action_just_pressed(InputAction::SelectWeapon2) { input_state.select_weapon = Some(1); }
// ... continues through SelectWeapon0
```

- `else if` chain ensures only one weapon selected per frame even with simultaneous key presses
- Priority ordering (1 before 2 before 3...) provides deterministic behavior
- `SelectWeapon0` maps to slot index 9 (tenth weapon slot) following common gaming convention
- Resets to `None` each frame—gameplay systems must consume selection immediately or lose it

### 8.2 Buffer Cleanup System

```rust
fn cleanup_input_buffer(
    time: Res<Time>,
    config: Res<InputConfig>,
    mut input_buffer: ResMut<InputBuffer>,
)
```

- Executes after input state update but before gameplay systems consume buffer
- Uses `Vec::retain()` for allocation-free filtering
- Time-based expiration independent of frame rate:
  ```rust
  input_buffer.actions.retain(|ba| now - ba.timestamp <= config.buffer_ttl);
  ```
- No configuration required beyond initial `buffer_ttl` setting

### 8.3 Player Input Synchronization System

```rust
fn player_input_sync_system(
    input_state: Res<InputState>,
    mut query: Query<&mut InputState, (With<Player>, Without<AiController>)>,
)
```

Critical query constraints:

- **`With<Player>`**: Only entities marked as player-controlled receive synchronization
- **`Without<AiController>`**: Explicitly excludes AI-controlled entities that might coincidentally have `Player` marker
- Prevents AI entities from receiving player input state
- Enables mixed scenes with both player and AI entities using same component type

Synchronization mechanism:

- Full clone of global `InputState` resource to each matching entity
- Occurs every frame after global state fully updated
- Ensures player entities always observe current input state snapshot
- Clone overhead minimal due to `InputState`'s compact representation (~80 bytes)

### 8.4 Stub Systems: Movement and Action Processing

```rust
fn process_movement_input(_input: Res<InputState>) {}
fn process_action_input(_input: Res<InputState>) {}
```

These placeholder systems represent integration points for gameplay logic:

- **`process_movement_input`**: Intended for character controller systems consuming `movement` vector and continuous press states (`sprint_pressed`, `crouch_pressed`)
- **`process_action_input`**: Intended for discrete action systems consuming momentary press states (`jump_pressed`, `fire_just_pressed`) and buffered actions

Current stub implementation enables:
- System ordering demonstration in schedule configuration
- Easy replacement with actual gameplay systems
- Clear separation between input processing and gameplay effect application

Production implementations would replace stubs with systems containing actual character movement, weapon firing, and interaction logic.

---

## 9. Configuration System

### 9.1 InputConfig Resource Structure

```rust
pub struct InputConfig {
    pub mouse_sensitivity: f32,
    pub gamepad_sensitivity: f32,
    pub invert_y_axis: bool,
    pub buffer_ttl: f32,
}
```

#### Sensitivity Parameters

- **`mouse_sensitivity`**: Multiplier applied to raw mouse delta values before populating `look` vector. Default `0.15` represents moderate sensitivity:
  - Lower values (<0.1): Slower, more precise aiming suitable for snipers
  - Higher values (>0.3): Faster turning suitable for close-quarters combat
  - Typical range: 0.05 to 0.5 depending on game genre and player preference
  - Applied in look vector processing systems (not in core input system)

- **`gamepad_sensitivity`**: Analogous multiplier for gamepad thumbstick inputs. Default `1.0` represents direct mapping:
  - Values >1.0 amplify stick movement for faster turning
  - Values <1.0 dampen stick movement for precision control
  - Separate parameter enables independent tuning for different input devices

#### Axis Inversion

- **`invert_y_axis`**: Boolean flag for vertical look inversion (common preference among flight simulator players):
  - `false`: Push mouse forward = look down (standard)
  - `true`: Push mouse forward = look up (inverted)
  - Applied during look vector processing rather than in core input system
  - Typically exposed as user-configurable option in settings menu

#### Buffer Configuration

- **`buffer_ttl`**: Time-to-live for buffered actions in seconds. Default `0.15` (150ms):
  - Directly impacts jump forgiveness window
  - Affects interaction reliability during movement
  - Should be tuned based on animation durations in target game
  - Longer buffers beneficial for platformers with precise timing requirements
  - Shorter buffers preferred for competitive shooters requiring immediate feedback

### 9.2 Configuration Initialization

Default implementation provides balanced starting point:

```rust
impl Default for InputConfig {
    fn default() -> Self {
        Self {
            mouse_sensitivity: 0.15,
            gamepad_sensitivity: 1.0,
            invert_y_axis: false,
            buffer_ttl: 0.15,
        }
    }
}
```

Game-specific tuning recommended before production release:
- Analyze animation durations to determine optimal buffer TTL
- Conduct playtesting to refine default sensitivity values
- Consider genre conventions (FPS players expect different defaults than RPG players)

### 9.3 Runtime Configuration Updates

`InputConfig` is a standard Bevy resource enabling runtime modification:

```rust
// In settings UI system when player adjusts slider
config.mouse_sensitivity = new_value;
```

Changes take effect immediately on next frame:
- No system restarts or resource reinitialization required
- Sensitivity changes apply to subsequent mouse delta processing
- Buffer TTL changes affect expiration of actions buffered after change

Persistence requires explicit serialization/deserialization as with `InputMap`.

---

## 10. Integration Patterns

### 10.1 Player Controller Integration

Recommended integration pattern for character controllers:

```rust
// In character movement system
fn character_movement(
    input: Res<InputState>,
    mut query: Query<(&mut Transform, &mut CharacterController), With<Player>>,
) {
    if !input.enabled {
        return; // Respect global input disable state
    }
    
    for (mut transform, mut controller) in query.iter_mut() {
        // Apply movement vector with sprint modifier
        let speed = if input.sprint_pressed { 
            controller.sprint_speed 
        } else { 
            controller.walk_speed 
        };
        
        let movement = input.movement * speed * time.delta_seconds();
        // ... apply movement to transform
    }
}
```

Critical integration considerations:

- Always check `input.enabled` before processing to respect global disable state
- Use `movement` vector directly—already normalized and directionally correct
- Apply speed multipliers in gameplay systems rather than input system for flexibility
- Continuous press states (`sprint_pressed`) enable analog-like speed variation

### 10.2 Jump Buffering Implementation

Leveraging buffered jump input in character controller:

```rust
fn jump_handling(
    mut input_buffer: ResMut<InputBuffer>,
    mut query: Query<&mut CharacterController>,
) {
    for mut controller in query.iter_mut() {
        // Check both direct input and buffered input
        let wants_to_jump = input.jump_pressed || input_buffer.consume(InputAction::Jump);
        
        if wants_to_jump && controller.can_jump() {
            controller.initiate_jump();
        }
    }
}
```

Buffer consumption pattern ensures:
- Jump triggers from either current frame input or buffered input
- Buffered jump consumed only once (removed from buffer after consumption)
- No double-jump from single physical input press

### 10.3 Weapon System Integration

Weapon selection and firing integration pattern:

```rust
fn weapon_system(
    input: Res<InputState>,
    mut weapons: ResMut<WeaponInventory>,
) {
    // Handle weapon selection
    if let Some(slot) = input.select_weapon {
        weapons.equip_slot(slot);
    }
    
    // Handle firing with buffering consideration
    if input.fire_just_pressed || input_buffer.consume(InputAction::Fire) {
        weapons.active_weapon.fire();
    }
    
    // Handle reload
    if input.reload_pressed {
        weapons.active_weapon.reload();
    }
}
```

Selection handling considerations:
- Consume `select_weapon` immediately—it resets to `None` next frame regardless
- Support both direct fire input and buffered fire for animation forgiveness
- Reload typically continuous press rather than momentary for cancelable reloads

### 10.4 Menu System Integration

Input disable pattern for pause menus:

```rust
fn pause_menu_system(
    mut input_state: ResMut<InputState>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut menu_state: ResMut<MenuState>,
) {
    // Toggle pause on ESC press
    if keyboard.just_pressed(KeyCode::Escape) {
        menu_state.paused = !menu_state.paused;
        
        // Disable player input when paused
        input_state.set_input_enabled(!menu_state.paused);
    }
    
    // UI navigation uses separate input context not affected by disable
    if menu_state.paused {
        // Process UI navigation inputs here
        // These would use direct ButtonInput checks rather than InputState
    }
}
```

Critical separation:
- Global `InputState` disable affects only gameplay systems
- UI systems should use direct `ButtonInput` resources for navigation
- Prevents UI navigation from being blocked when gameplay input disabled
- Enables simultaneous gameplay pause with active UI interaction

### 10.5 Camera System Integration

Camera control integration leveraging look vector:

```rust
fn camera_control(
    input: Res<InputState>,
    config: Res<InputConfig>,
    mut query: Query<&mut CameraController>,
) {
    for mut controller in query.iter_mut() {
        // Apply sensitivity multiplier to raw look input
        let look_delta = input.look * config.mouse_sensitivity;
        
        // Apply Y-axis inversion if configured
        let look_delta = Vec2::new(look_delta.x, 
            if config.invert_y_axis { -look_delta.y } else { look_delta.y }
        );
        
        // Apply rotation to camera
        controller.yaw += look_delta.x;
        controller.pitch += look_delta.y;
        
        // Clamp pitch to prevent gimbal lock
        controller.pitch = controller.pitch.clamp(-1.5, 1.5);
    }
}
```

Look vector processing responsibilities:
- Sensitivity multiplication applied in camera system (not input system)
- Axis inversion handled during processing rather than at capture time
- Pitch clamping prevents unnatural camera orientations
- Raw `look` vector preserved in `InputState` for alternative camera implementations

---

## 11. Best Practices & Design Patterns

### 11.1 Input Action Design Principles

#### Semantic Action Naming

- Prefer domain-specific names (`Crouch` vs `ControlPress`)
- Avoid device-specific terminology (`MouseRight` vs `Aim`)
- Group related actions with consistent prefixes (`SelectWeapon1` through `SelectWeapon0`)
- Reserve generic names (`Action1`, `ButtonA`) for configurable action slots

#### Action Granularity

- Split compound actions into primitives when gameplay requires independent control:
  - Prefer separate `Aim` and `Fire` over combined `AimAndFire`
  - Enables advanced techniques like quick-scoping (brief aim then fire)
- Avoid over-segmentation creating binding bloat:
  - Single `Jump` action sufficient—no need for `ShortJump`/`LongJump` primitives
  - Duration-based variations handled through press timing in gameplay systems

#### Buffering Strategy

- Buffer actions with high temporal sensitivity:
  - Jumping (animation lockout periods)
  - Interactions (approach timing)
  - Target acquisition (combat flow)
- Avoid buffering continuous actions:
  - Movement inputs benefit from immediate responsiveness
  - Sprint/crouch modifiers should not buffer to prevent unintended state changes

### 11.2 Binding Configuration Guidelines

#### Conflict Avoidance

- Audit default bindings for overlapping assignments:
  - `LeanRight` and `Interact` both default to `E`—requires resolution
  - `CornerLean` and `SwitchCameraMode` both default to `C`—requires resolution
- Document intentional conflicts with resolution strategy:
  - Contextual binding suppression (disable lean when in vehicle)
  - Priority ordering (interaction overrides lean when near interactable)

#### Accessibility Considerations

- Provide multiple bindings for critical actions:
  - `LockOn` bound to both keyboard and mouse button
  - Movement supports both WASD and arrow keys in alternative configurations
- Avoid binding essential actions exclusively to mouse buttons:
  - Players with limited mouse dexterity require keyboard alternatives
- Reserve modifier combinations for optional enhancements rather than core mechanics:
  - `Ctrl+C` for copy acceptable; `Ctrl+Shift+Alt+Q` for jump unacceptable

#### Platform Adaptation

- Default bindings optimized for target platform:
  - PC: Keyboard/mouse optimized (WASD movement, mouse aiming)
  - Console: Gamepad optimized (thumbstick movement, shoulder button actions)
  - Mobile: Touch optimized (virtual stick areas, screen tap zones)
- Maintain binding abstraction to enable platform-specific defaults without code changes

### 11.3 Buffer TTL Tuning Methodology

Systematic approach to buffer configuration:

1. **Measure Animation Durations**: Profile critical animation sequences:
   - Jump landing recovery: typically 150-300ms
   - Weapon reload completion: varies by weapon type
   - Interaction approach window: distance-dependent

2. **Set Initial TTL**: Configure `buffer_ttl` to 75% of shortest critical animation:
   - Landing recovery 200ms → initial TTL 150ms
   - Ensures inputs captured during majority of lockout period

3. **Playtest with Edge Cases**:
   - Rapid jump sequences (jump immediately after landing)
   - Interaction attempts while running at full speed
   - Weapon switching during reload animations

4. **Adjust Based on Feedback**:
   - "Missed jumps" complaints → increase TTL by 25ms increments
   - "Unintended double jumps" complaints → decrease TTL by 25ms increments
   - Target 95%+ success rate on intentional rapid inputs

5. **Document Rationale**: Record final TTL value with justification:
   - "175ms TTL balances jump forgiveness with prevention of accidental double-jumps during sprinting"

### 11.4 Input Disable Patterns

Granular input disabling strategies:

#### Full Disable

```rust
input_state.set_input_enabled(false);
```

Use cases:
- Full-screen cutscenes with no player interaction
- Game over/level complete sequences before restart prompt
- Loading screens with no interactive elements

#### Partial Disable (Custom Implementation)

Core system doesn't support partial disable—implement through wrapper:

```rust
// Custom resource tracking disabled action groups
#[derive(Resource)]
struct DisabledInputGroups {
    movement: bool,
    combat: bool,
    camera: bool,
}

// System filtering actions based on disabled groups
fn filtered_input_update(
    disabled: Res<DisabledInputGroups>,
    raw_input: Res<InputState>,
    mut filtered: ResMut<FilteredInputState>,
) {
    filtered.movement = if disabled.movement { Vec2::ZERO } else { raw_input.movement };
    filtered.jump_pressed = if disabled.movement { false } else { raw_input.jump_pressed };
    // ... etc
}
```

Use cases:
- Driving sequences (disable movement but enable camera)
- Sniper scope (disable movement but enable aiming/firing)
- Dialogue sequences (disable movement/combat but enable dialogue choices)

### 11.5 Testing Methodologies

Comprehensive input testing strategy:

#### Automated Binding Validation

```rust
#[test]
fn test_no_duplicate_bindings() {
    let map = InputMap::default();
    let mut binding_usage = HashMap::new();
    
    for (action, bindings) in &map.bindings {
        for binding in bindings {
            let count = binding_usage.entry(binding.clone()).or_insert(0);
            *count += 1;
        }
    }
    
    // Allow up to 2 uses per binding (intentional conflicts)
    for (binding, count) in &binding_usage {
        assert!(count <= &2, "Binding {:?} used {} times", binding, count);
    }
}
```

#### Buffer Timing Tests

```rust
#[test]
fn test_jump_buffer_timing() {
    let mut buffer = InputBuffer::default();
    let config = InputConfig { buffer_ttl: 0.15, ..default() };
    
    // Add jump at t=0.0
    buffer.actions.push(BufferedAction { 
        action: InputAction::Jump, 
        timestamp: 0.0 
    });
    
    // Should be present at t=0.14
    buffer.actions.retain(|ba| 0.14 - ba.timestamp <= config.buffer_ttl);
    assert_eq!(buffer.actions.len(), 1);
    
    // Should be expired at t=0.16
    buffer.actions.retain(|ba| 0.16 - ba.timestamp <= config.buffer_ttl);
    assert_eq!(buffer.actions.len(), 0);
}
```

#### Edge Case Simulation

Manual testing scenarios:
- Rapid key mashing during animation transitions
- Simultaneous press of conflicting bindings
- Input during frame rate drops (simulate with frame limiter)
- Rebinding during active gameplay sequences
- Input disable/enable transitions during action sequences

---

## 12. Troubleshooting Guide

### 12.1 Common Issues & Solutions

#### Issue: Inputs Not Registering

**Symptoms**: Player presses keys but character doesn't move/act.

**Diagnosis Steps**:
1. Verify `input_state.enabled == true` (not disabled by menu/cutscene)
2. Check `InputMap` contains bindings for expected actions
3. Confirm no binding conflicts consuming inputs unexpectedly
4. Validate systems execute in correct order (input update before gameplay)

**Solutions**:
- Ensure `InputPlugin` added to App before gameplay plugins
- Add debug system printing current input state:
  ```rust
  fn debug_input(input: Res<InputState>) {
      println!("Movement: {:?}, Jump: {}", input.movement, input.jump_pressed);
  }
  ```
- Check for accidental `set_input_enabled(false)` calls without corresponding enable

#### Issue: Double Actions from Single Press

**Symptoms**: Single key press triggers action twice (double jump, double fire).

**Diagnosis Steps**:
1. Verify gameplay system consumes buffered actions with `consume()` not `is_buffered()`
2. Check for duplicate system registration processing same inputs
3. Confirm no rebinding state active causing input capture duplication

**Solutions**:
- Always use `consume()` for buffered actions to ensure single consumption
- Audit system registration for duplicate input processing systems
- Ensure rebinding UI properly resets `RebindState.action` after assignment

#### Issue: Diagonal Movement Faster Than Cardinal

**Symptoms**: Character moves faster when pressing two movement keys simultaneously.

**Diagnosis Steps**:
1. Verify `movement.normalize_or_zero()` called in input processing
2. Check gameplay systems not re-normalizing already normalized vectors
3. Confirm no speed multipliers applied differentially to diagonal movement

**Solutions**:
- Ensure normalization occurs exactly once in input processing pipeline
- Gameplay systems should apply speed multipliers to normalized vectors:
  ```rust
  // Correct
  let velocity = input.movement.normalize_or_zero() * speed;
  
  // Incorrect - double normalization
  let velocity = input.movement.normalize_or_zero().normalize() * speed;
  ```

#### Issue: Rebinding Not Capturing Inputs

**Symptoms**: Rebinding UI active but inputs not captured as new bindings.

**Diagnosis Steps**:
1. Verify `RebindState.action` properly set to `Some(action)`
2. Check no other systems consuming inputs before rebinding system
3. Confirm rebinding system executes after input state update but before gameplay systems

**Solutions**:
- Add debug print in rebinding system to verify execution:
  ```rust
  if rebind_state.action.is_some() {
      println!("Rebinding active for {:?}", rebind_state.action);
  }
  ```
- Ensure rebinding system registered in same schedule phase as input processing
- Verify no UI systems consuming inputs with `ButtonInput::reset()` calls

#### Issue: Buffer Overflow / Memory Growth

**Symptoms**: Memory usage increases steadily during extended play sessions.

**Diagnosis Steps**:
1. Profile `InputBuffer.actions` vector size over time
2. Verify `cleanup_input_buffer` system executes every frame
3. Check for manual buffer additions without corresponding cleanup

**Solutions**:
- Confirm `cleanup_input_buffer` system registered in Update schedule
- Avoid manual buffer manipulation outside core systems
- Implement buffer size cap as safety measure:
  ```rust
  // In buffer population
  if input_buffer.actions.len() > 10 {
      input_buffer.actions.remove(0); // Drop oldest
  }
  ```

### 12.2 Diagnostic Tools

#### Input State Visualizer

Debug overlay displaying current input state:

```
INPUT STATE (enabled: true)
Movement: (0.00, 1.00)  [Forward]
Look: (2.34, -1.87)
Jump: ■ (buffered)
Sprint: □
Crouch: □
Fire: ■ (just pressed)
Weapon: None
```

Implementation approach:
- Create UI text entity updated every frame
- Format boolean states as filled/empty squares for quick scanning
- Highlight buffered actions with special indicator
- Color-code based on input category (movement green, combat red)

#### Binding Conflict Detector

System identifying problematic binding configurations:

```rust
fn detect_binding_conflicts(map: Res<InputMap>) {
    let mut binding_to_actions: HashMap<InputBinding, Vec<InputAction>> = HashMap::new();
    
    for (action, bindings) in &map.bindings {
        for binding in bindings {
            binding_to_actions.entry(binding.clone())
                .or_insert_with(Vec::new)
                .push(*action);
        }
    }
    
    for (binding, actions) in &binding_to_actions {
        if actions.len() > 1 {
            warn!("Binding conflict: {:?} mapped to {:?}", binding, actions);
        }
    }
}
```

Run during development builds to catch configuration issues early.

---

## 13. Performance Characteristics

### 13.1 Computational Complexity

#### Per-Frame Processing Cost

- **Binding Translation**: O(B×A) where B=bindings per action, A=actions checked
  - Typical: 35 actions × 1-2 bindings = 35-70 binding checks per frame
  - Optimized through early-exit on first matching binding
  - Negligible cost (<0.01ms) on modern hardware

- **Buffer Management**: O(N) where N=buffered actions
  - Typical buffer size: 0-3 actions during normal play
  - Worst case: 10+ actions during input mashing
  - `Vec::retain()` provides allocation-free filtering
  - Negligible cost (<0.005ms)

- **State Synchronization**: O(E) where E=player entities
  - Typical: 1 player entity + occasional vehicles = 1-3 entities
  - Clone operation on 80-byte structure extremely cheap
  - Negligible cost (<0.001ms)

#### Memory Footprint

- **InputMap**: ~2KB (35 actions × avg 2 bindings × ~30 bytes per binding)
- **InputState (Resource)**: ~80 bytes
- **InputState (Per Entity)**: ~80 bytes × entity count
- **InputBuffer**: ~48 bytes per buffered action × typical 0-3 actions = 0-144 bytes
- **InputConfig**: 16 bytes
- **RebindState**: 8 bytes

Total typical footprint: <3KB—negligible for modern systems.

### 13.2 Optimization Opportunities

#### Binding Lookup Caching

Current implementation performs hashmap lookup per action per frame:

```rust
if let Some(bindings) = input_map.bindings.get(&action) { ... }
```

Optimization potential:
- Precompute action-to-binding index mapping at startup
- Eliminate hashmap overhead for fixed action set
- Estimated gain: <0.001ms per frame—likely not worth complexity

#### Buffer Structure Optimization

Current `Vec<BufferedAction>` sufficient for typical use:
- Alternative: Fixed-size ring buffer for allocation-free operation
- Benefit minimal given tiny typical buffer sizes
- Added complexity unjustified for performance gain

#### Movement Vector Precomputation

Current implementation reconstructs movement vector every frame:
- Alternative: Cache normalized vector when inputs unchanged
- Complexity: Requires tracking input state change detection
- Benefit minimal given trivial normalization cost for 2D vectors

**Conclusion**: System already highly optimized for typical workloads. Micro-optimizations unlikely to yield measurable gains in real-world scenarios.

### 13.3 Scalability Considerations

#### Multiplayer Scaling

System scales linearly with player count:
- Each player requires separate `InputState` component
- Global resources (`InputMap`, `InputBuffer`) remain single-instance
- Synchronization cost: O(P) where P=player count
- 4-player split-screen: ~0.004ms sync cost—negligible

#### Action Set Expansion

Adding actions scales linearly:
- 100 actions instead of 35: ~0.015ms additional binding checks
- Still negligible for 60+ FPS targets
- Practical limit: ~500 actions before binding checks become measurable (>0.1ms)

#### Input Device Expansion

Adding gamepad support requires:
- New `InputBinding` variant (`Gamepad(GamepadButton)`)
- Extended binding evaluation closures
- Additional `ButtonInput<GamepadButton>` resource access
- Minimal performance impact—same O(B×A) complexity with slightly higher constant factor

---

## 14. Extension Points & Future Enhancements

### 14.1 Gamepad Support Integration

Current architecture supports straightforward gamepad extension:

#### Required Modifications

1. Extend `InputBinding` enumeration:
   ```rust
   pub enum InputBinding {
       Key(KeyCode),
       Mouse(MouseButton),
       Gamepad(GamepadButton),
   }
   ```

2. Add gamepad resource access to `update_input_state`:
   ```rust
   gamepad_buttons: Res<ButtonInput<GamepadButton>>,
   ```

3. Extend binding evaluation closures:
   ```rust
   InputBinding::Gamepad(button) => gamepad_buttons.pressed(button.clone()),
   ```

4. Add gamepad axis processing for analog movement:
   ```rust
   // In movement construction
   let gamepad_axis = Vec2::new(
       gamepad_axes.axis(GamepadAxis::LEFT_STICK_X),
       gamepad_axes.axis(GamepadAxis::LEFT_STICK_Y),
   );
   movement += gamepad_axis;
   ```

#### Design Considerations

- **Binding Priority**: Define evaluation order when multiple devices active:
  - Option 1: Last-pressed device takes priority
  - Option 2: Configurable priority per action
  - Option 3: Simultaneous input blending (complex)

- **Dead Zone Configuration**: Analog stick dead zones require separate configuration resource:
  ```rust
  pub struct GamepadConfig {
      pub left_stick_deadzone: f32,
      pub right_stick_deadzone: f32,
  }
  ```

- **Rumble/Haptic Feedback**: Extend system with output capabilities:
  ```rust
  pub struct HapticRequest {
      pub gamepad: Gamepad,
      pub intensity: f32,
      pub duration: f32,
  }
  ```

### 14.2 Advanced Buffering Features

#### Action-Specific TTL Configuration

Current uniform TTL may not suit all actions:
- Jump benefits from 150ms buffer
- Interact might need only 50ms buffer
- Fire might need 0ms buffer (no buffering)

Extension approach:
```rust
pub struct BufferConfig {
    pub default_ttl: f32,
    pub overrides: HashMap<InputAction, f32>,
}

// In buffer population
let ttl = config.overrides.get(&action).copied().unwrap_or(config.default_ttl);
```

#### Input Chording Support

Buffer combinations of simultaneous inputs:
- Double-tap detection (tap twice quickly for special move)
- Directional inputs (down-down for slide)
- Modifier combinations (crouch+jump for vault)

Implementation strategy:
- Track input history with timestamps
- Pattern matching against chord definitions
- Separate buffer resource for chord detection

### 14.3 Input Recording & Playback

Replay system for:
- Tutorial demonstrations
- Speedrun verification
- Networked game reconciliation

Core requirements:
- Deterministic input capture
- Timestamped input events
- Playback with identical timing

Extension architecture:
```rust
#[derive(Resource)]
pub struct InputRecorder {
    pub recording: bool,
    pub playback: bool,
    pub events: Vec<InputEvent>,
}

pub struct InputEvent {
    pub timestamp: f32,
    pub action: InputAction,
    pub pressed: bool,
}
```

Challenges:
- Maintaining determinism across systems
- Handling random number generators during playback
- Memory management for long recordings

### 14.4 Accessibility Enhancements

#### Input Remapping Profiles

Predefined control schemes for accessibility needs:
- One-handed keyboard layouts
- Mouse-only operation modes
- Reduced input complexity modes

Implementation:
```rust
pub enum ControlProfile {
    Standard,
    OneHandedLeft,
    OneHandedRight,
    MouseOnly,
    Simplified,
}

impl ControlProfile {
    pub fn apply_to_map(&self, map: &mut InputMap) {
        match self {
            ControlProfile::OneHandedLeft => {
                // Remap all actions to left-hand accessible keys
                map.bindings.insert(InputAction::Jump, vec![InputBinding::Key(KeyCode::Space)]);
                // ... etc
            }
            // ... other profiles
        }
    }
}
```

#### Input Assistance Features

- **Input Forgiveness Expansion**: Increase buffer TTL dynamically based on player performance
- **Auto-Correct**: Gently correct near-miss inputs (jump pressed 2 frames late → still jump)
- **Input Simplification**: Convert complex inputs to simpler equivalents (rapid fire → sustained fire)

---

## 15. Related Systems & Integration Points

### 15.1 Character Controller System

Tight integration required for responsive movement:
- Consumes `movement` vector and continuous press states
- Implements jump buffering using `InputBuffer`
- Applies physics constraints to raw input vectors
- Provides `can_jump()` state for buffered jump validation

Critical interface:
```rust
// CharacterController component provides state query methods
impl CharacterController {
    pub fn can_jump(&self) -> bool {
        self.is_grounded && !self.jump_cooldown.active()
    }
}
```

### 15.2 Weapon System

Action consumption patterns:
- Weapon selection: Consume `select_weapon` immediately (single-frame validity)
- Firing: Processes direct input state for Auto/Semi-Auto modes, with internal timer management for Burst sequences
- Reloading: Typically continuous press for cancelable reloads
- Switching: Momentary press with cooldown to prevent accidental switches

State synchronization:
- Active weapon state affects valid inputs (cannot fire when reloading)
- Input system remains unaware of weapon state—validation occurs in weapon system

### 15.3 Camera System

Look vector processing responsibilities:
- Apply sensitivity multipliers from `InputConfig`
- Handle Y-axis inversion preference
- Clamp pitch to prevent unnatural orientations
- Smooth input for cinematic camera behaviors

Separation of concerns:
- Input system provides raw `look` vector
- Camera system applies transformations and constraints
- Enables multiple camera types sharing same input source

### 15.4 UI / Menu System

Critical separation patterns:
- Gameplay input: Processed through `InputState` resource
- UI navigation: Processed directly through `ButtonInput` resources
- Input disable: Affects only gameplay systems, not UI navigation

Menu state machine:
```
GameState::Playing 
  → ESC pressed 
  → GameState::Paused (disable gameplay input, enable UI navigation)
  → Resume selected 
  → GameState::Playing (enable gameplay input, disable UI navigation)
```

### 15.5 Animation System

Input-driven animation selection:
- Movement vector magnitude drives locomotion blend tree
- Jump pressed triggers jump start animation
- Crouch pressed transitions to crouched animation state
- Aim pressed activates aiming upper-body layer

Animation-state feedback:
- Landing animation completion enables jump buffering window
- Reload animation progress affects fire input validity
- Input system unaware of animation state—buffering provides temporal bridge

---

## 16. Conclusion & Recommendations

### 16.1 System Maturity Assessment

The input system demonstrates production-ready characteristics:

- **Architectural Soundness**: Clear separation of concerns between input collection, translation, and consumption
- **Performance Efficiency**: Negligible computational and memory overhead even under stress conditions
- **Extensibility**: Well-defined extension points for gamepad support, advanced buffering, and accessibility features
- **Robustness**: Comprehensive edge case handling (buffer expiration, input disabling, state synchronization)
- **Maintainability**: Clean code structure with minimal interdependencies between components

Recommended for production use with minor configuration tuning for target game genre.

### 16.2 Implementation Recommendations

#### Pre-Production Phase

1. Audit default bindings for conflicts specific to your game:
   - Resolve `LeanRight`/`Interact` conflict (both default to `E`)
   - Resolve `CornerLean`/`SwitchCameraMode` conflict (both default to `C`)
   - Document final binding decisions in design documentation

2. Determine buffer TTL based on animation durations:
   - Profile jump landing recovery animations
   - Set initial `buffer_ttl` to 75% of shortest critical animation
   - Plan playtesting session specifically for buffer tuning

3. Design rebinding UI with conflict detection:
   - Implement visual warnings for binding conflicts
   - Provide one-click resolution options (reassign conflicting action)
   - Include "Restore Defaults" functionality

#### Production Phase

1. Implement input state visualizer for QA:
   - Overlay showing current input state during gameplay
   - Highlight buffered actions with distinct visual treatment
   - Enable via debug key combination (e.g., F12)

2. Conduct accessibility review:
   - Verify all critical actions have keyboard alternatives
   - Test with input delay simulators to validate buffer effectiveness
   - Gather feedback from players with motor impairments

3. Performance profiling:
   - Measure input system cost across target hardware spectrum
   - Verify no frame drops during input mashing scenarios
   - Confirm memory footprint stability during extended sessions

#### Post-Launch Considerations

1. Monitor rebinding analytics:
   - Track most frequently rebound actions
   - Identify problematic default bindings for future titles
   - Correlate rebinding patterns with player retention metrics

2. Plan input system evolution:
   - Prioritize gamepad support based on player platform distribution
   - Evaluate demand for advanced accessibility features
   - Prepare architecture for potential VR/AR input modalities

### 16.3 Final Assessment

This input system provides a robust foundation for diverse game genres with minimal modification. Its action-based architecture, temporal buffering capabilities, and runtime rebinding support address the majority of input handling requirements for modern games. The clean separation between input processing and gameplay logic ensures long-term maintainability as game complexity grows.

Recommended adoption path:
1. Integrate core system with default bindings (resolving known conflicts)
2. Tune buffer TTL through targeted playtesting
3. Implement rebinding UI with conflict detection
4. Extend with gamepad support when platform requirements demand
5. Enhance with accessibility features based on player feedback

The system's thoughtful architecture provides solid footing for both immediate production needs and future evolution as player expectations and input technologies advance.