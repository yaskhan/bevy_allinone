# Interaction System

This document provides a complete, production-ready description of the Interaction System.
It is written for designers, gameplay programmers, and technical artists who need a deep understanding of how interactions are detected, validated, presented, and dispatched across the game.
All descriptions are system-focused and remain implementation-agnostic, while still reflecting the real data and behavior defined by the codebase.

## Documentation Contents

- Overview
- Design Goals
- Terminology
- High-Level Architecture
- Interaction Lifecycle
- Component Reference
- Type Reference
- Resource Reference
- Event and Queue Reference
- System Reference
- Device Management Model
- UI and Prompt Behavior
- Input Integration
- Cross-System Integration
- Performance and Scaling
- Debugging and Visualization
- Save and Serialization Notes
- Multiplayer and Networking Considerations
- Design Patterns Catalog
- Edge Case Catalog
- Best Practices
- Testing and QA Checklist
- Troubleshooting
- Future Extensions

## Overview

The Interaction System orchestrates how players and AI-controlled entities discover, understand, and activate interactive objects.
It merges short-range raycast detection, proximity-driven device lists, and context-aware UI prompts into a single operational flow.
The system is intentionally modular, enabling game teams to use only lightweight components for simple interactions, or full device lists for complex in-world machinery.
It also plays a bridging role between other subsystems, such as puzzles, inventory, dialog, and devices.
By design, it is neutral to narrative content; it focuses on selection and dispatch, leaving actual effects to downstream systems.

Key characteristics include:
- Deterministic selection of the most relevant target.
- Support for both instantaneous and cooldown-limited interactions.
- A predictable UI prompt flow that mirrors selection logic.
- Dedicated queues for interaction events to avoid hard coupling.
- A device list architecture that mirrors the original project structure.
- Debug-friendly visualization of raycasts and detected hit points.

## Design Goals

- Provide a player-friendly interaction model that always explains why an interaction is or is not available.
- Keep interaction detection deterministic even with multiple potential targets.
- Support a wide variety of interaction verbs without per-verb logic duplication.
- Decouple detection from execution so game logic can respond to interactions asynchronously.
- Offer both raycast and proximity detection to serve different device types.
- Make UI and world logic consistent through shared interaction state.
- Expose all parameters as data-driven fields on components to reduce hardcoded behavior.
- Provide clear extension points for new interaction types or filters.

## Terminology

- Interactable: Any entity carrying interaction metadata and eligible for player actions.
- Interaction Detector: The sensor component that performs raycasts and tracks when to update.
- Device: A special category of interactable that is managed by the device list system.
- Device List: A per-player list of nearby devices discovered by proximity scanning.
- Interaction Prompt: UI element that communicates the current action to the player.
- Interaction Event: The payload sent to other systems when an interaction succeeds.
- Interaction Type: The verb or category defining how the interaction should be interpreted.
- Cooldown: A lockout window preventing repeated interactions within a short time.

## High-Level Architecture

The Interaction System is composed of a plugin, components, resources, events, and systems.
A single plugin wires the update order so detection feeds selection, selection feeds validation, and validation feeds processing.
The architecture favors a data-driven flow where each stage consumes state produced by the previous stage.

Core architectural pillars:
- Detection: Raycast-driven discovery of potential interactables.
- Device Management: Proximity-driven inclusion of devices into a selectable list.
- Selection: Choosing the closest usable device or the current raycast hit.
- Validation: Cooldown and state checks that gate whether an interaction is allowed.
- Execution: Dispatching Interaction Events and toggling device state when required.
- Presentation: Updating the prompt text, color, and visibility based on availability.
- Debugging: Optional gizmo-based visualization for raycasts and hits.

The system also includes explicit hooks for specific downstream use cases:
- Electronic device activation notifications.
- Grab interactions for physics-based grabbing.
- Custom data payloads for puzzles, quests, or dialog triggers.

## Interaction Lifecycle

The lifecycle below describes the canonical flow for a player-initiated interaction.
Each step is deterministic and repeatable, ensuring that UI and gameplay logic remain aligned.

1. Sensor Update
- The Interaction Detector updates on a configurable interval.
- A raycast is emitted from the detector origin using the configured offset.
- The closest hit is stored as the candidate interactable.

2. Device Proximity Scan
- The device list system evaluates nearby devices based on distance thresholds.
- Devices are added or removed from the list using queue-based events.

3. Selection
- If the player has a device list, the closest valid device is selected.
- If no device is selected, the raycast candidate becomes the active interactable.

4. Range and State Validation
- Interaction distance and cooldowns are evaluated.
- Interactables can opt out dynamically via the can_interact flag.

5. UI Update
- The prompt displays the chosen target and interaction verb.
- The prompt color reflects whether the target is in range.
- When no target exists, the prompt is hidden.

6. Input Consumption
- Interaction input is detected directly or via buffered input.
- On success, the buffered input is consumed to prevent duplicate activation.

7. Interaction Execution
- The interaction event is queued with source, target, and interaction type.
- Device state toggles if the target is a usable device.
- Secondary system events such as electronic device activation are queued.

8. Cooldown Enforcement
- The interactable cooldown timer begins if configured.
- The interactable is temporarily locked until the cooldown expires.

## Component Reference

This section documents all components in the Interaction System.
Each component includes a purpose statement, configuration details, and practical use notes.

### InteractionDetector
Purpose:
- Provides a raycast-based sensor to detect interactables in front of an entity.
- Controls update cadence to balance responsiveness with performance.
Use cases:
- Player camera raycast for crosshair-based interactions.
- AI interaction sensors for scripted interaction behaviors.
Field reference:
- max_distance: Maximum length of the raycast used for detection.
- ray_offset: Positional offset applied to the ray origin, typically aligned with the eyes or camera.
- update_interval: Frequency of updates in seconds; zero means every frame.
- time_since_update: Accumulator used internally to track interval timing.
- interaction_layers: Layer mask used to filter raycast hits.
Behavioral notes:
- Update intervals above zero reduce load but may introduce slight detection latency.
- Raycast hits are stored regardless of interaction distance; range is validated separately.
- The detector is stateless about the target, relying on CurrentInteractable for shared state.

### Interactable
Purpose:
- Marks an entity as eligible for interaction and defines the primary verb and text.
Use cases:
- Doors, switches, loot items, NPCs, or context-sensitive devices.
Field reference:
- interaction_text: Display name or short label shown in the UI prompt.
- interaction_distance: Maximum distance required for successful interaction.
- can_interact: Dynamic flag used to block or allow interaction.
- interaction_type: High-level verb defining the interaction category.
Behavioral notes:
- The prompt text reflects the interaction_text combined with the interaction_type verb.
- can_interact is automatically updated by cooldown logic but can be overridden externally.
- interaction_distance should align with world scale and collision size.

### UsingDevicesSystem
Purpose:
- Maintains a per-player list of nearby devices and handles selection logic.
Use cases:
- Complex machinery with multiple nearby buttons or panels.
- Vehicle entry points or seat selectors.
Field reference:
- can_use_devices: Enables or disables device list behavior.
- device_list: Collection of device metadata derived from DeviceStringAction.
- current_device_index: Index of the currently selected device, or -1 when none.
- use_device_action_name: Localized label for device activation actions.
- raycast_distance: Range for detecting devices in proximity scanning.
- layer_mask: Mask for device-related raycasts when enabled.
- searching_devices_with_raycast: Enables raycast-based device discovery.
- show_use_device_icon_enabled: Toggles device icon rendering in the UI.
- use_min_distance_to_use_devices: Applies minimum distance checks for selection.
- min_distance_to_use_devices: Distance threshold for device inclusion.
- use_only_device_if_visible_on_camera: Requires device visibility to be selected.
- driving: Flag used by vehicle systems to prevent certain interactions.
Behavioral notes:
- device_list is updated by queue-based add and remove events.
- current_device_index is set based on closest distance among listed devices.
- When current_device_index is valid, it overrides raycast-based selection.

### DeviceStringAction
Purpose:
- Stores device-specific metadata used when a device is added to a player's list.
Use cases:
- Buttons, levers, or terminals with custom distances, labels, or directional restrictions.
Field reference:
- device_name: Display name used in UI prompts and device lists.
- device_action: Primary action label for the device.
- secondary_device_action: Optional additional verb label for special UI.
- show_icon: Whether to show a device icon when in range.
- action_offset: Distance offset for context positioning or UI alignment.
- use_local_offset: Interprets action_offset in local space when true.
- use_custom_min_distance: Enables custom distance checks for device selection.
- custom_min_distance: The custom distance threshold if enabled.
- use_custom_min_angle: Enables custom angle checks for device selection.
- custom_min_angle: The minimum facing angle allowed if enabled.
- use_relative_direction: Adjusts facing checks relative to the device orientation.
- ignore_use_only_if_visible: Overrides the player requirement to see the device.
- check_if_obstacle: Enables obstacle checks when selecting devices.
- icon_enabled: Determines whether UI icons for this device are enabled.
Behavioral notes:
- These values are copied into DeviceInfo when added to a device list.
- The component provides per-device overrides that are preserved even if the player settings differ.

### InteractionPrompt
Purpose:
- Marks the UI node that displays the interaction prompt text.
Use cases:
- Main HUD prompt for primary interaction actions.
Behavioral notes:
- The prompt is hidden when no interactable or device is selected.
- The prompt text is dynamically rebuilt every update based on current target.

### InteractionData
Purpose:
- Encapsulates timing and contextual data for a specific interactable.
Use cases:
- Interactions that require cooldowns, delays, or custom data payloads.
Field reference:
- duration: Time required to complete the interaction, or zero for instant interactions.
- cooldown: Minimum delay before the interaction can occur again.
- cooldown_timer: Current remaining cooldown time.
- auto_trigger: Enables automatic interactions when in range.
- data: Opaque string for custom logic such as item IDs or quest markers.
Behavioral notes:
- duration can be used to drive progress bars in external UI systems.
- auto_trigger is intentionally passive; external systems should check it to decide auto-activation.
- data is a free-form field intended for designers or content pipelines.

### UsableDevice
Purpose:
- Adds stateful toggle behavior and key requirements to an interactable.
Use cases:
- Switches, doors, valves, and powered devices that can be turned on or off.
Field reference:
- is_active: Current on/off state of the device.
- requires_key: Whether a key or unlock condition is required.
- key_id: Identifier for the required key item.
- active_text: Interaction text shown when the device is currently active.
- inactive_text: Interaction text shown when the device is currently inactive.
Behavioral notes:
- The interaction system toggles is_active on successful use.
- active_text and inactive_text replace the interactable interaction_text after toggling.
- Key gating is expected to be enforced by downstream systems using InteractionEvent data.

## Type Reference

### InteractionType
Purpose:
- Encodes the verb used when describing and dispatching interactions.
Supported values and interpretation:
- Pickup: Used for item collection, loot, or inventory transfers.
- Use: General-purpose action without a specialized semantic meaning.
- Talk: Triggers dialog or NPC interaction flows.
- Open: Explicitly communicates a door or container opening action.
- Activate: Typically used for power systems, terminals, or activation panels.
- Examine: Intended for inspection or lore-based interactions.
- Toggle: For binary state changes similar to switches.
- Grab: For physics-based or attachment-based grabbing systems.
- Device: A generic category for specialized device interactions.
Behavioral notes:
- The UI maps each type to a human-readable verb.
- Downstream systems should use interaction_type to route events appropriately.

### DeviceInfo
Purpose:
- Represents a device stored inside a player’s device list.
- Mirrors fields from DeviceStringAction while storing a reference to the device entity.
Field reference:
- name: Display name for the device in prompts and lists.
- entity: Entity identifier of the device.
- action_offset: Distance or offset used for UI placement or activation positioning.
- use_local_offset: Whether to interpret action_offset in local space.
- use_custom_min_distance: Enables custom distance checks.
- custom_min_distance: Custom distance threshold if enabled.
- use_custom_min_angle: Enables custom angle checks.
- custom_min_angle: Minimum angle threshold if enabled.
- use_relative_direction: Uses device orientation when evaluating angle checks.
- ignore_use_only_if_visible: Overrides player visibility requirement.
- check_if_obstacle: Enables obstruction testing between player and device.
Behavioral notes:
- Values are immutable copies; changes to DeviceStringAction require re-adding the device to update.
- DeviceInfo exists to decouple device selection from direct component queries.

## Resource Reference

### CurrentInteractable
Purpose:
- Stores the currently detected interactable from the raycast detector.
Field reference:
- entity: Optional entity identifier for the detected interactable.
- distance: Distance from the detector to the hit point.
- interaction_point: World-space position of the raycast hit.
- is_in_range: Whether the interactable is within interaction distance and is usable.
Behavioral notes:
- This resource is cleared at the beginning of detection each update.
- It acts as the fallback selection when no device list target exists.

### InteractionDebugSettings
Purpose:
- Provides configuration for debug visualization of interaction rays.
Field reference:
- enabled: Global switch for debug drawing.
- ray_color: Base color used for ray rendering.
- hit_color: Color used when the ray hits an interactable.
- miss_color: Color used when the ray hits nothing.
Behavioral notes:
- The system uses the hit_color if CurrentInteractable has an entity.
- Colors are meant for in-editor debugging and can be configured per project.

### InteractionUIState
Purpose:
- Tracks UI prompt visibility and current text state.
Field reference:
- is_visible: Whether the interaction UI is currently shown.
- current_text: The full text currently displayed.
Behavioral notes:
- The UI state resource is optional, but useful for integrations that need to know prompt status.
- UI state also supports analytics or tutorial systems that depend on prompt visibility.

## Event and Queue Reference

### AddDeviceEvent
Purpose:
- Requests that a device be added to a player’s device list.
Fields:
- player: The entity that owns the device list.
- device: The device entity to add.
Usage notes:
- The event is produced by proximity detection or custom scripts.
- Processing is deferred via AddDeviceQueue for deterministic updates.

### RemoveDeviceEvent
Purpose:
- Requests that a device be removed from a player’s device list.
Fields:
- player: The entity that owns the device list.
- device: The device entity to remove.
Usage notes:
- Removal is triggered by leaving the proximity range or custom logic.
- Device removal is safe even if the device is no longer valid.

### InteractionEvent
Purpose:
- Delivers the result of a successful interaction to downstream systems.
Fields:
- source: The interacting entity, usually the player.
- target: The entity being interacted with.
- interaction_type: The verb defining what action took place.
Usage notes:
- Downstream systems should treat events as authoritative.
- Events are queued for deterministic ordering and possible batch processing.

### InteractionEventQueue
Purpose:
- Stores a vector of InteractionEvent values generated this frame.
Usage notes:
- Consumers should drain the queue each update to avoid repeated processing.

### AddDeviceQueue and RemoveDeviceQueue
Purpose:
- Resource queues for device list updates.
Usage notes:
- Queues decouple detection from list mutation to avoid mutable aliasing issues.
- Both queues are cleared after processing in update_device_list.

## System Reference

This section describes each system in the Interaction plugin, its responsibilities, and its practical implications.

### setup_interaction_ui
Responsibilities:
- Builds a UI node for the interaction prompt.
- Positions the node near the lower center of the screen.
- Initializes prompt visibility as hidden.
Usage notes:
- Projects with custom UI can replace or extend this system.
- The prompt node is marked with InteractionPrompt for easy querying.

### update_interaction_ui
Responsibilities:
- Determines the current interaction target and updates UI visibility.
- Prioritizes UsingDevicesSystem target over raycast target.
- Builds descriptive prompt text using interaction type verbs.
- Applies color cues to indicate when the target is out of range.
Usage notes:
- If multiple players exist, the system currently uses the first available device list.
- The key prompt is currently represented as a placeholder and should be localized.

### validate_interactions
Responsibilities:
- Updates cooldown timers on interactables.
- Sets can_interact false while the cooldown is active.
Usage notes:
- Cooldown logic only applies to interactables with InteractionData.
- External systems can override can_interact to enforce quest or state gating.

### detect_interactables
Responsibilities:
- Performs raycast detection for each InteractionDetector.
- Updates CurrentInteractable with hit data.
- Resets CurrentInteractable at the start of each update cycle.
Usage notes:
- Raycasts ignore origin penetration to prevent self-hits.
- The detection update interval can be tuned for performance.

### process_interactions
Responsibilities:
- Detects player interaction input and resolves the selected target.
- Ensures the target is in range and can interact.
- Applies cooldowns and toggles device state if applicable.
- Pushes InteractionEvent into the queue.
- Triggers device activation and grab event queues.
Usage notes:
- The system prefers device list targets to ensure consistent device interactions.
- Input buffering allows interactions even if the key press occurs just before target selection.

### debug_draw_interaction_rays
Responsibilities:
- Draws raycasts and hit points using gizmos.
- Visualizes hit versus miss states using configurable colors.
Usage notes:
- This system should be disabled for production builds.
- It relies on CurrentInteractable for hit status.

### update_device_list
Responsibilities:
- Applies AddDeviceEvent and RemoveDeviceEvent to player device lists.
- Copies DeviceStringAction data into DeviceInfo entries.
Usage notes:
- Devices already in the list are not duplicated.
- Removal ignores unknown devices to avoid errors.

### select_closest_device
Responsibilities:
- Chooses the nearest device in a player’s device list.
- Updates current_device_index to reflect the chosen device.
Usage notes:
- Selection respects custom distance overrides per device.
- The closest device in range is always chosen, providing deterministic behavior.

### detect_devices_in_proximity
Responsibilities:
- Adds or removes devices from a player’s list based on distance.
- Uses the player’s raycast_distance setting as a threshold.
Usage notes:
- This system is intentionally simple and can be extended with visibility checks.
- It treats all DeviceStringAction entities as potential devices.

## Device Management Model

The device model exists to solve scenarios where multiple interactive devices are close together.
Instead of letting the raycast select an arbitrary target, devices are tracked in a list and selected deterministically.
The list uses proximity scanning and explicit add/remove events to ensure predictable selection.

Key behaviors:
- Device discovery is distance-based by default.
- Devices can implement custom distance and angle constraints.
- Device list selection overrides raycast target selection.
- Device-specific metadata is copied at the time of addition for stability.

Device selection flow:
- Discover devices within raycast_distance and enqueue add events.
- When devices leave the radius, enqueue remove events.
- Apply add/remove events to the device list in update_device_list.
- Select the closest valid device in select_closest_device.
- Use the selected device to build prompt text and determine interaction target.

Design implications:
- Devices with custom_min_distance can remain selectable even when the player is farther than default.
- Angle-based constraints make it possible to require players to face a device before interaction.
- check_if_obstacle enables future line-of-sight checks, preventing interactions through walls.

## UI and Prompt Behavior

The interaction prompt is the primary communication tool for interactions.
Its state is derived directly from selection logic to maintain player trust.

Prompt composition rules:
- If a selected device exists, the prompt is built from that device’s interactable data.
- Otherwise, the prompt is built from CurrentInteractable if present.
- The verb is derived from InteractionType and mapped to a localized phrase.
- The action key is inserted into the string as a placeholder label.
- When out of range, a suffix is appended to describe the restriction.

Visual feedback rules:
- In-range targets use a neutral bright color.
- Out-of-range targets switch to a warning color.
- Hidden state is used when no target is available.

Usability notes:
- Prompts should be localized and may include platform-specific key icons.
- Accessibility can be improved by offering higher-contrast color schemes.
- The prompt system is intentionally minimal to allow project-specific UI overlays.

## Input Integration

The Interaction System integrates with the shared input subsystem.
It supports both immediate input and buffered input to ensure responsive interactions.

Input handling characteristics:
- The system checks direct input state for interaction presses.
- It also checks the input buffer for queued interaction actions.
- If input is used successfully, it is consumed to prevent re-triggering.

Design guidance:
- Input buffering should be short enough to avoid unintended interactions.
- The interaction action should be mapped consistently with prompts.
- For accessibility, consider supporting multiple input mappings or held interactions.

## Cross-System Integration

The Interaction System provides a central handshake between player input and other systems.
It emits interaction events and exposes state that other subsystems can interpret.

Common integration patterns:
- Inventory System: Pickup interactions trigger item acquisition or loot windows.
- Dialog System: Talk interactions open dialog nodes and lock player movement.
- Quest System: Interaction data can advance quest objectives or start scripts.
- Puzzle System: Device interactions manipulate puzzle objects, sequences, or triggers.
- Tutorial System: Prompts can be monitored to decide when to show guidance.
- Vehicles System: Devices can represent entry points, seats, or control panels.
- Devices System: Electronic device activation events allow custom device logic.
- Grab System: Grab interaction types forward events to the grab subsystem.
- Map System: Interaction events can trigger fast travel or map reveal logic.
- Save System: Interaction state, such as device toggles, can be serialized.

Integration guidelines:
- Treat InteractionEvent as the authoritative signal for action completion.
- Use InteractionData data field to route specific outcomes without hardcoded entity checks.
- Keep UI prompt logic independent to avoid coupling with quest or inventory UI.

## Performance and Scaling

The Interaction System is lightweight but must be tuned for large scenes.
Performance considerations:
- Raycasts are the most expensive step and should be limited with update_interval.
- Device proximity scanning can be optimized with spatial partitioning if needed.
- UI updates are cheap but should only update when state changes in high-load scenarios.
- Gizmo drawing is meant for debug only and should be disabled in shipping builds.

Scaling recommendations:
- Use multiple InteractionDetector components only when necessary.
- Consider layered collision masks to reduce raycast hits.
- Batch device list scans by grouping players in time-sliced updates.
- Avoid excessive InteractionData on static props that do not need cooldowns.

## Debugging and Visualization

Debug visualization is essential for diagnosing interaction issues.
The debug system draws rays and hit points for each detector.

Recommended debug workflow:
- Enable InteractionDebugSettings when tuning interaction distances.
- Verify the raycast origin matches the camera or eyes.
- Inspect the hit point to confirm the correct entity is selected.
- Temporarily increase ray length to identify layering or obstruction problems.

Debug data to log:
- CurrentInteractable entity and distance.
- InteractionType and interaction_text used for prompts.
- Device list contents and current_device_index.

## Save and Serialization Notes

The Interaction System itself does not automatically persist state.
However, several components have data worth saving for continuity.

Common persistence targets:
- UsableDevice is_active state for doors and switches.
- InteractionData cooldowns when resuming mid-action.
- Custom data strings for unique device identifiers.

Serialization guidance:
- Store only designer-authored values; runtime-only values can be reinitialized.
- Rebuild device lists when loading, rather than serializing device_list directly.
- Ensure key_id references are stable across sessions.

## Multiplayer and Networking Considerations

Multiplayer implementations should treat interaction selection as local and interaction execution as authoritative.
Key considerations:
- Clients can predict prompt and selection but should await server confirmation for actions.
- InteractionEvent should be replicated or validated by the server.
- Cooldown timers should be authoritative to prevent exploitative rapid interaction.
- Device state toggles must be synchronized across clients.
- Device list logic should include ownership to avoid cross-player contamination.

## Design Patterns Catalog

The following catalog lists common interaction design patterns.
Each pattern is described with its intent and implementation guidance.

### Pattern 001: Simple Pickup
Intent:
- Provide a simple pickup interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 002: Contextual Use
Intent:
- Provide a contextual use interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 003: Talk Trigger
Intent:
- Provide a talk trigger interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 004: Door Interaction
Intent:
- Provide a door interaction interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 005: Terminal Activation
Intent:
- Provide a terminal activation interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 006: Puzzle Lever
Intent:
- Provide a puzzle lever interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 007: Timed Switch
Intent:
- Provide a timed switch interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 008: Multi-Step Device
Intent:
- Provide a multi-step device interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 009: Key-Locked Door
Intent:
- Provide a key-locked door interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 010: Quest Gated Use
Intent:
- Provide a quest gated use interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 011: Inspection Point
Intent:
- Provide a inspection point interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 012: Loot Chest
Intent:
- Provide a loot chest interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 013: Vehicle Entry
Intent:
- Provide a vehicle entry interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 014: Seat Selection
Intent:
- Provide a seat selection interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 015: Lift Control
Intent:
- Provide a lift control interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 016: Power Relay
Intent:
- Provide a power relay interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 017: Emergency Button
Intent:
- Provide a emergency button interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 018: Alarm Toggle
Intent:
- Provide a alarm toggle interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 019: Hidden Panel
Intent:
- Provide a hidden panel interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 020: Crafting Station
Intent:
- Provide a crafting station interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 021: Simple Pickup
Intent:
- Provide a simple pickup interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 022: Contextual Use
Intent:
- Provide a contextual use interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 023: Talk Trigger
Intent:
- Provide a talk trigger interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 024: Door Interaction
Intent:
- Provide a door interaction interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 025: Terminal Activation
Intent:
- Provide a terminal activation interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 026: Puzzle Lever
Intent:
- Provide a puzzle lever interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 027: Timed Switch
Intent:
- Provide a timed switch interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 028: Multi-Step Device
Intent:
- Provide a multi-step device interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 029: Key-Locked Door
Intent:
- Provide a key-locked door interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 030: Quest Gated Use
Intent:
- Provide a quest gated use interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 031: Inspection Point
Intent:
- Provide a inspection point interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 032: Loot Chest
Intent:
- Provide a loot chest interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 033: Vehicle Entry
Intent:
- Provide a vehicle entry interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 034: Seat Selection
Intent:
- Provide a seat selection interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 035: Lift Control
Intent:
- Provide a lift control interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 036: Power Relay
Intent:
- Provide a power relay interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 037: Emergency Button
Intent:
- Provide a emergency button interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 038: Alarm Toggle
Intent:
- Provide a alarm toggle interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 039: Hidden Panel
Intent:
- Provide a hidden panel interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 040: Crafting Station
Intent:
- Provide a crafting station interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 041: Simple Pickup
Intent:
- Provide a simple pickup interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 042: Contextual Use
Intent:
- Provide a contextual use interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 043: Talk Trigger
Intent:
- Provide a talk trigger interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 044: Door Interaction
Intent:
- Provide a door interaction interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 045: Terminal Activation
Intent:
- Provide a terminal activation interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 046: Puzzle Lever
Intent:
- Provide a puzzle lever interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 047: Timed Switch
Intent:
- Provide a timed switch interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 048: Multi-Step Device
Intent:
- Provide a multi-step device interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 049: Key-Locked Door
Intent:
- Provide a key-locked door interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 050: Quest Gated Use
Intent:
- Provide a quest gated use interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 051: Inspection Point
Intent:
- Provide a inspection point interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 052: Loot Chest
Intent:
- Provide a loot chest interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 053: Vehicle Entry
Intent:
- Provide a vehicle entry interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 054: Seat Selection
Intent:
- Provide a seat selection interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 055: Lift Control
Intent:
- Provide a lift control interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 056: Power Relay
Intent:
- Provide a power relay interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 057: Emergency Button
Intent:
- Provide a emergency button interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 058: Alarm Toggle
Intent:
- Provide a alarm toggle interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 059: Hidden Panel
Intent:
- Provide a hidden panel interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 060: Crafting Station
Intent:
- Provide a crafting station interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 061: Simple Pickup
Intent:
- Provide a simple pickup interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 062: Contextual Use
Intent:
- Provide a contextual use interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 063: Talk Trigger
Intent:
- Provide a talk trigger interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 064: Door Interaction
Intent:
- Provide a door interaction interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 065: Terminal Activation
Intent:
- Provide a terminal activation interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 066: Puzzle Lever
Intent:
- Provide a puzzle lever interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 067: Timed Switch
Intent:
- Provide a timed switch interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 068: Multi-Step Device
Intent:
- Provide a multi-step device interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 069: Key-Locked Door
Intent:
- Provide a key-locked door interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 070: Quest Gated Use
Intent:
- Provide a quest gated use interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 071: Inspection Point
Intent:
- Provide a inspection point interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 072: Loot Chest
Intent:
- Provide a loot chest interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 073: Vehicle Entry
Intent:
- Provide a vehicle entry interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 074: Seat Selection
Intent:
- Provide a seat selection interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 075: Lift Control
Intent:
- Provide a lift control interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 076: Power Relay
Intent:
- Provide a power relay interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 077: Emergency Button
Intent:
- Provide a emergency button interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 078: Alarm Toggle
Intent:
- Provide a alarm toggle interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 079: Hidden Panel
Intent:
- Provide a hidden panel interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 080: Crafting Station
Intent:
- Provide a crafting station interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 081: Simple Pickup
Intent:
- Provide a simple pickup interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 082: Contextual Use
Intent:
- Provide a contextual use interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 083: Talk Trigger
Intent:
- Provide a talk trigger interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 084: Door Interaction
Intent:
- Provide a door interaction interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 085: Terminal Activation
Intent:
- Provide a terminal activation interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 086: Puzzle Lever
Intent:
- Provide a puzzle lever interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 087: Timed Switch
Intent:
- Provide a timed switch interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 088: Multi-Step Device
Intent:
- Provide a multi-step device interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 089: Key-Locked Door
Intent:
- Provide a key-locked door interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 090: Quest Gated Use
Intent:
- Provide a quest gated use interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 091: Inspection Point
Intent:
- Provide a inspection point interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 092: Loot Chest
Intent:
- Provide a loot chest interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 093: Vehicle Entry
Intent:
- Provide a vehicle entry interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 094: Seat Selection
Intent:
- Provide a seat selection interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 095: Lift Control
Intent:
- Provide a lift control interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 096: Power Relay
Intent:
- Provide a power relay interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 097: Emergency Button
Intent:
- Provide a emergency button interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 098: Alarm Toggle
Intent:
- Provide a alarm toggle interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 099: Hidden Panel
Intent:
- Provide a hidden panel interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 100: Crafting Station
Intent:
- Provide a crafting station interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 101: Simple Pickup
Intent:
- Provide a simple pickup interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 102: Contextual Use
Intent:
- Provide a contextual use interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 103: Talk Trigger
Intent:
- Provide a talk trigger interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 104: Door Interaction
Intent:
- Provide a door interaction interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 105: Terminal Activation
Intent:
- Provide a terminal activation interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 106: Puzzle Lever
Intent:
- Provide a puzzle lever interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 107: Timed Switch
Intent:
- Provide a timed switch interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 108: Multi-Step Device
Intent:
- Provide a multi-step device interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 109: Key-Locked Door
Intent:
- Provide a key-locked door interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 110: Quest Gated Use
Intent:
- Provide a quest gated use interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 111: Inspection Point
Intent:
- Provide a inspection point interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 112: Loot Chest
Intent:
- Provide a loot chest interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 113: Vehicle Entry
Intent:
- Provide a vehicle entry interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 114: Seat Selection
Intent:
- Provide a seat selection interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 115: Lift Control
Intent:
- Provide a lift control interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 116: Power Relay
Intent:
- Provide a power relay interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 117: Emergency Button
Intent:
- Provide a emergency button interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 118: Alarm Toggle
Intent:
- Provide a alarm toggle interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 119: Hidden Panel
Intent:
- Provide a hidden panel interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

### Pattern 120: Crafting Station
Intent:
- Provide a crafting station interaction that is predictable and easy to discover.
Recommended components:
- Interactable with appropriate interaction_type and interaction_text.
- InteractionData when a cooldown or auto-trigger is needed.
- UsableDevice when state toggling is required.
Flow notes:
- Ensure the interaction is within a comfortable distance for the player camera.
- Provide clear prompt text that matches the player’s expectation.
- Use InteractionEvent to inform downstream systems of the chosen action.
Common pitfalls:
- Forgetting to reset can_interact after cooldown leads to permanent lockout.
- Misaligned interaction_distance causes prompts to appear but interactions to fail.
Testing guidance:
- Verify prompt visibility when in range and hidden when out of range.
- Confirm that InteractionEvent fires exactly once per input.

## Edge Case Catalog

This catalog enumerates interaction edge cases and suggested mitigations.
Each entry is written as a scenario with a resolution approach.

### Edge Case 001
Scenario:
- Multiple interactables overlapping in front of the camera.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 002
Scenario:
- Interactable is behind transparent geometry.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 003
Scenario:
- Device list contains destroyed entities.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 004
Scenario:
- Cooldown timer ends while player is out of range.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 005
Scenario:
- Player switches input device mid-interaction.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 006
Scenario:
- Interaction prompt flickers due to update interval.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 007
Scenario:
- Raycast hits a collider without Interactable.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 008
Scenario:
- Device requires key but key is missing.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 009
Scenario:
- InteractionData duration is non-zero but no progress UI.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 010
Scenario:
- Auto-trigger interactions in crowded scenes.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 011
Scenario:
- InteractionDetector has a large offset.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 012
Scenario:
- UsingDevicesSystem is disabled while a device is selected.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 013
Scenario:
- Multiple players share the same device.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 014
Scenario:
- InteractionEvent queue not drained.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 015
Scenario:
- UsableDevice toggles during scripted cutscene.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 016
Scenario:
- Interaction text changes during cooldown.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 017
Scenario:
- Device list is empty but UI prompt remains visible.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 018
Scenario:
- InteractionType is Device but no DeviceStringAction.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 019
Scenario:
- Interaction prompt does not localize key label.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 020
Scenario:
- Raycast hits object with custom collision layers.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 021
Scenario:
- Multiple interactables overlapping in front of the camera.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 022
Scenario:
- Interactable is behind transparent geometry.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 023
Scenario:
- Device list contains destroyed entities.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 024
Scenario:
- Cooldown timer ends while player is out of range.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 025
Scenario:
- Player switches input device mid-interaction.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 026
Scenario:
- Interaction prompt flickers due to update interval.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 027
Scenario:
- Raycast hits a collider without Interactable.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 028
Scenario:
- Device requires key but key is missing.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 029
Scenario:
- InteractionData duration is non-zero but no progress UI.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 030
Scenario:
- Auto-trigger interactions in crowded scenes.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 031
Scenario:
- InteractionDetector has a large offset.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 032
Scenario:
- UsingDevicesSystem is disabled while a device is selected.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 033
Scenario:
- Multiple players share the same device.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 034
Scenario:
- InteractionEvent queue not drained.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 035
Scenario:
- UsableDevice toggles during scripted cutscene.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 036
Scenario:
- Interaction text changes during cooldown.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 037
Scenario:
- Device list is empty but UI prompt remains visible.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 038
Scenario:
- InteractionType is Device but no DeviceStringAction.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 039
Scenario:
- Interaction prompt does not localize key label.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 040
Scenario:
- Raycast hits object with custom collision layers.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 041
Scenario:
- Multiple interactables overlapping in front of the camera.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 042
Scenario:
- Interactable is behind transparent geometry.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 043
Scenario:
- Device list contains destroyed entities.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 044
Scenario:
- Cooldown timer ends while player is out of range.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 045
Scenario:
- Player switches input device mid-interaction.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 046
Scenario:
- Interaction prompt flickers due to update interval.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 047
Scenario:
- Raycast hits a collider without Interactable.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 048
Scenario:
- Device requires key but key is missing.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 049
Scenario:
- InteractionData duration is non-zero but no progress UI.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 050
Scenario:
- Auto-trigger interactions in crowded scenes.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 051
Scenario:
- InteractionDetector has a large offset.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 052
Scenario:
- UsingDevicesSystem is disabled while a device is selected.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 053
Scenario:
- Multiple players share the same device.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 054
Scenario:
- InteractionEvent queue not drained.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 055
Scenario:
- UsableDevice toggles during scripted cutscene.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 056
Scenario:
- Interaction text changes during cooldown.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 057
Scenario:
- Device list is empty but UI prompt remains visible.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 058
Scenario:
- InteractionType is Device but no DeviceStringAction.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 059
Scenario:
- Interaction prompt does not localize key label.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 060
Scenario:
- Raycast hits object with custom collision layers.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 061
Scenario:
- Multiple interactables overlapping in front of the camera.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 062
Scenario:
- Interactable is behind transparent geometry.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 063
Scenario:
- Device list contains destroyed entities.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 064
Scenario:
- Cooldown timer ends while player is out of range.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 065
Scenario:
- Player switches input device mid-interaction.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 066
Scenario:
- Interaction prompt flickers due to update interval.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 067
Scenario:
- Raycast hits a collider without Interactable.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 068
Scenario:
- Device requires key but key is missing.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 069
Scenario:
- InteractionData duration is non-zero but no progress UI.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 070
Scenario:
- Auto-trigger interactions in crowded scenes.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 071
Scenario:
- InteractionDetector has a large offset.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 072
Scenario:
- UsingDevicesSystem is disabled while a device is selected.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 073
Scenario:
- Multiple players share the same device.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 074
Scenario:
- InteractionEvent queue not drained.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 075
Scenario:
- UsableDevice toggles during scripted cutscene.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 076
Scenario:
- Interaction text changes during cooldown.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 077
Scenario:
- Device list is empty but UI prompt remains visible.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 078
Scenario:
- InteractionType is Device but no DeviceStringAction.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 079
Scenario:
- Interaction prompt does not localize key label.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

### Edge Case 080
Scenario:
- Raycast hits object with custom collision layers.
Impact:
- Player perception of reliability is reduced or interactions feel inconsistent.
Mitigation:
- Adjust detection settings, validate entities, and ensure UI uses the same selection logic.
Validation steps:
- Reproduce the scenario with debug rays enabled and verify the target selection.

## Best Practices

- Keep interaction_text short and action-oriented to fit on the HUD.
- Use InteractionType consistently so verbs align with player expectations.
- Provide a unique InteractionData data value for special-case logic to avoid entity-specific checks.
- Avoid setting update_interval too high; it can make prompts feel laggy.
- Pair device interactions with clear visual affordances in the scene.
- Use can_interact to enforce quest or state gating rather than removing the Interactable component.
- Where possible, keep interaction_distance close to the collision bounds of the object.
- Integrate InteractionDebugSettings into dev-only menus for quick tuning.
- Drain InteractionEventQueue each update to prevent replayed actions.

## Testing and QA Checklist

Interaction detection:
- Verify that the raycast origin aligns with the camera.
- Confirm that interactables become selectable at expected distances.
- Ensure that non-interactable colliders do not produce prompts.

Device behavior:
- Confirm devices appear in the list when within range.
- Verify device selection chooses the nearest valid device.
- Test custom distance overrides and angle constraints.

Input behavior:
- Verify interaction input is consumed after activation.
- Test buffered input to ensure late detections still trigger interactions.
- Validate that holding the key does not cause repeated events.

UI behavior:
- Ensure the prompt text updates when selection changes.
- Confirm out-of-range color changes are visible.
- Validate that the prompt hides when no target exists.

State and cooldown:
- Confirm cooldown blocks repeated interactions.
- Verify that interactions resume when cooldown expires.
- Check that UsableDevice text toggles appropriately.

Integration tests:
- Trigger a dialog with Talk interaction and confirm dialog opens.
- Trigger a puzzle action with Device interaction and confirm puzzle progression.
- Trigger a pickup and confirm item is added to inventory.

## Troubleshooting

Problem: The prompt never appears.
- Verify that the target has an Interactable component.
- Check interaction_layers and collision layers for raycast mismatches.
- Ensure the InteractionDetector update_interval is not excessively large.

Problem: The prompt appears but interaction does nothing.
- Confirm can_interact is true and cooldown_timer is zero.
- Validate that input mapping triggers the interaction action.
- Check that the InteractionEventQueue is being processed by downstream systems.

Problem: Interactions trigger multiple times.
- Ensure the input buffer is consumed after use.
- Check for multiple InteractionDetector components on the same entity.
- Verify that InteractionEventQueue is drained each update.

Problem: Device interactions target the wrong object.
- Review device list distances and custom_min_distance values.
- Verify that the device list is not carrying stale entities.
- Enable debug rays and inspect current_device_index selection.

Problem: UsableDevice does not toggle state.
- Confirm that UsableDevice is attached to the target entity.
- Ensure the interaction is processed and not blocked by cooldowns.
- Verify that active_text and inactive_text are not empty.

## Future Extensions

The Interaction System is designed to be extensible.
Potential additions include:
- Line-of-sight checks for device selection using physics raycasts.
- Interaction progress UI for non-instant interactions.
- Multi-step interaction sequences with staged prompts.
- Support for interaction priorities to handle dense scenes.
- Enhanced localization tools for verbs and prompt composition.
- Per-player interaction channels for split-screen or network play.
- Interaction analytics for tracking player engagement and usability.
- Optional audio cues for interaction availability and completion.

## Closing Notes

This document intentionally focuses on system behavior, data models, and operational guidance.
It should be used as the primary reference when designing, implementing, or debugging interactions.
For deeper integration with other systems, consult the documentation for those subsystems and apply the integration guidance described above.
