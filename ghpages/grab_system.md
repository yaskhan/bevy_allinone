# Grab System

## Overview

The Grab System is a comprehensive physics-based object manipulation framework that enables players and AI entities to interact with the game world through realistic grabbing, holding, rotating, throwing, and placing mechanics. Built on Bevy's entity-component system and integrated with Avian3D physics, it provides both simple single-object grabbing for player interaction and advanced "powers"-style multi-object manipulation for complex gameplay scenarios.

**Key Features:**
- Physics-based object grabbing with realistic follow behavior
- Multiple grab modes (Realistic spring physics vs Position-based Powers mode)
- Object rotation while held with configurable sensitivity
- Throwing mechanics with charge-up force accumulation
- Multi-object power grabbing for telekinesis-style abilities
- Object slotting/placement system for puzzles and inventory mechanics
- Melee weapon functionality using grabbed objects
- Visual outline/highlight system for grabbable object feedback
- Physical properties modification (mass, colliders) during grab
- Comprehensive event system for grab/drop/throw actions
- Distance-based drop prevention and validation
- Integration with combat, puzzle, and interaction systems

**Module Location:** `src/grab/`
- `mod.rs` - Plugin definition and resource initialization
- `types.rs` - All component and event definitions
- `systems.rs` - Update systems for grab mechanics

---

## Core Concepts

### Grab Architecture

The Grab System operates through a sophisticated entity-component architecture that separates concerns between grabbers, grabbable objects, and the physics simulation:

1. **Grabber** - The entity that initiates and controls grabbing (usually player or AI)
2. **Grabbable** - Objects that can be grabbed, with physical properties
3. **GrabEventQueue** - Centralized event bus for grab/drop/throw actions
4. **GrabPowerer** - Advanced grabber supporting multiple simultaneous objects
5. **PutObjectSystem** - Slot-based placement and validation
6. **Physical Modifiers** - Runtime adjustment of mass, colliders, and physics properties

### Interaction Flow

The grab interaction follows a multi-stage pipeline:

1. **Detection Phase** - System detects input and scan for grabbable objects in range
2. **Validation Phase** - Distance, weight limits, and slot compatibility checks
3. **Event Generation** - GrabEvent::Grab is queued for processing
4. **Physics Integration** - Held object positions/rotations are updated per-frame
5. **Release Phase** - Drop or throw events trigger physics simulation handoff
6. **State Cleanup** - Grabber and object states are reset

### Physics Integration

Two primary physics modes determine object behavior while grabbed:

**Realistic Mode**: Uses spring physics with velocity-based following, creating natural momentum and response to collisions. Objects have inertia and can collide with environment while held.

**Powers Mode**: Uses direct position manipulation for precise, lag-free control. Ideal for puzzle elements or powers-style telekinesis where immediate response is critical.

### Event-Driven Design

All grab interactions flow through a centralized `GrabEventQueue` resource, providing several benefits:

- Decoupled input handling from physics processing
- Frame-safe event processing with predictable order
- Easy integration with external systems via event listening
- Support for both immediate and queued grab attempts
- Clean separation between logical events and physical effects

---

## Component Reference

### Grabbable

Core component that marks entities as interactable grab targets. Attaches to props, weapons, physics objects, and puzzle elements.

**Physical Properties:**
- `use_weight: bool` - Whether weight affects grab limits and throw physics
- `weight: f32` - Object weight for grab validation (1.0 = standard unit)
- `extra_grab_distance: f32` - Additional range allowance for this object specifically

**Behavior Control:**
- `use_events: bool` - Fire events when grabbed/dropped for external system integration
- `parent_to_grab: Option<Entity>` - Redirect grab action to different entity (for compound objects or hierarchy-based grabs)

**Usage Example:**
```rust
// Simple physics prop
commands.spawn((
    Grabbable {
        use_weight: true,
        weight: 1.5,
        extra_grab_distance: 0.0,
        use_events: true,
        parent_to_grab: None,
    },
    RigidBody::Dynamic,
    Collider::cuboid(0.5, 0.5, 0.5),
    PbrBundle {
        mesh: cube_mesh.clone(),
        material: crate_material.clone(),
        transform: Transform::from_xyz(2.0, 1.0, 3.0),
        ..default()
    },
));

// Lightweight throwable that can be grabbed from further away
commands.spawn((
    Grabbable {
        use_weight: false,
        weight: 0.2,
        extra_grab_distance: 1.5,
        use_events: false,
        parent_to_grab: None,
    },
    Collider::ball(0.3),
    // ... visual components
));

// Compound object - grabbing any part grabs the root
let root_entity = root_entity_id;
commands.spawn((
    Grabbable {
        use_weight: true,
        weight: 5.0,
        extra_grab_distance: 0.0,
        use_events: true,
        parent_to_grab: Some(root_entity),
    },
    // ... visual components for this sub-part
));
```

**Integration Patterns:**
```rust
// Combine with puzzle elements
commands.spawn((
    Grabbable {
        weight: 0.5,
        use_events: true, // Enable puzzle state updates
        ..default()
    },
    PuzzleElement::PressurePlateTrigger,
    // ... physics and visual components
));

// Combine with devices for grab-to-activate mechanics
commands.spawn((
    Grabbable {
        weight: 0.1,
        extra_grab_distance: 0.5,
        ..default()
    },
    Device::KeycardReader,
    Interactable {
        interaction_type: InteractionType::GrabInsert,
        activation_distance: 2.0,
    },
));
```

---

### Grabber

Component attached to entities capable of grabbing objects. Typically placed on player character, AI agents, or remote manipulation devices.

**Core State:**
- `held_object: Option<Entity>` - Entity currently being held, None if empty-handed
- `is_rotating: bool` - Whether rotation mode is active (right-click held)
- `is_charging_throw: bool` - Whether throw charge is being held (R key)

**Position & Range:**
- `hold_distance: f32` - Target distance from grabber to held object
- `max_hold_distance: f32` - Maximum distance before force-drop occurs
- `hold_speed: f32` - Speed multiplier for object following behavior (higher = tighter following)

**Throw Mechanics:**
- `throw_force: f32` - Current throw velocity (accumulates during charge)
- `max_throw_force: f32` - Maximum throw velocity cap
- `throw_force` increases linearly while `is_charging_throw` is true

**Rotation Control:**
- `rotation_speed: f32` - Sensitivity of mouse rotation input while holding object

**Physics Mode:**
- `mode: GrabMode` - Either GrabMode::Powers or GrabMode::Realistic

**Usage Example:**
```rust
// Standard player grabber
commands.spawn((
    Grabber {
        held_object: None,
        hold_distance: 2.0,    // Hold objects 2 units away
        max_hold_distance: 4.0, // Drop if object goes beyond 4 units
        hold_speed: 10.0,      // Responsive following
        rotation_speed: 5.0,   // Moderate rotation sensitivity
        throw_force: 500.0,    // Base throw strength
        max_throw_force: 2000.0, // Maximum throw power
        mode: GrabMode::Powers, // Precise control mode
        is_rotating: false,
        is_charging_throw: false,
    },
    InputState::default(), // Needed for grab input
));

// AI grabber with longer range for utility tasks
commands.spawn((
    Grabber {
        held_object: None,
        hold_distance: 3.5,
        max_hold_distance: 6.0,
        hold_speed: 15.0,      // AI needs precise following
        rotation_speed: 2.0,   // Minimal rotation for AI
        throw_force: 800.0,
        max_throw_force: 2500.0,
        mode: GrabMode::Realistic, // Physics-based movement
        is_rotating: false,
        is_charging_throw: false,
    },
    AIAgent::new(AgentType::Utility),
));

// Remote manipulator device (drone, camera, etc)
commands.spawn((
    Grabber {
        held_object: None,
        hold_distance: 1.5,    // Close manipulation for precision work
        max_hold_distance: 3.0,
        hold_speed: 8.0,
        rotation_speed: 8.0,   // High precision rotation
        throw_force: 200.0,    // Weak throwing for delicate operations
        max_throw_force: 500.0,
        mode: GrabMode::Powers,
        is_rotating: false,
        is_charging_throw: false,
    },
    RemoteManipulator::default(),
    CameraBundle::default(),
));
```

**Combat Integration:**
```rust
// Warrior class with enhanced throwing for tactical combat
commands.spawn((
    Grabber {
        held_object: None,
        hold_distance: 2.0,
        max_hold_distance: 4.0,
        hold_speed: 12.0,      // Fast repositioning for combat
        rotation_speed: 6.0,   // Quick defensive object rotation
        throw_force: 1000.0,  // Strong baselines for weapon throws
        max_throw_force: 3500.0, // Powerful maximum throws
        mode: GrabMode::Realistic, // Physics-based for momentum
        is_rotating: false,
        is_charging_throw: false,
    },
    CombatClass::Thrower,
    Stats {
        throw_power_modifier: 1.5, // Custom stat for throw enhancement
        ..default()
    },
));
```

**Stealth Integration:**
```rust
// Thief with quiet, controlled manipulation
commands.spawn((
    Grabber {
        held_object: None,
        hold_distance: 1.8,
        max_hold_distance: 3.5,
        hold_speed: 6.0,       // Slower movement = less noise/highlights
        rotation_speed: 3.0,   // Precise positioning for stealth gameplay
        throw_force: 400.0,    // Gentle throws to avoid alerting enemies
        max_throw_force: 1000.0,
        mode: GrabMode::Powers, // Precise control for silent manipulation
        is_rotating: false,
        is_charging_throw: false,
    },
    StealthProfile::SilentMover, // AI/reactiveness considerations
    SoundEmitter {
        base_volume: 0.3, // Reduced grab/drop sound radius
        ..default()
    },
));
```

---

### GrabPowerer

Advanced grab component enabling telekinesis-style multi-object manipulation. Supports grabbing, holding, and launching multiple objects simultaneously within a radius.

**Activation & State:**
- `is_enabled: bool` - Whether power grabbing is currently active
- `hold_objects: Vec<Entity>` - Collection of all currently held objects
- `last_grab_time: f32` - Timestamp of last grab attempt (prevents rapid re-grabs)

**Range & Detection:**
- `grab_radius: f32` - Circular area within which objects auto-attach when activated

**Launch Mechanics:**
- `launch_force: f32` - Current accumulated launch power
- `max_launch_force: f32` - Maximum launch power cap
- `launch_speed: f32` - Speed multiplier for launched objects
- `is_charging: bool` - Whether launch is currently being charged

**Usage Example:**
```rust
// Mage/telekinetic character power grabber
commands.spawn((
    Grabber {
        held_object: None,
        hold_distance: 2.0,
        max_hold_distance: 4.0,
        hold_speed: 10.0,
        rotation_speed: 5.0,
        throw_force: 500.0,
        max_throw_force: 2000.0,
        mode: GrabMode::Powers,
        is_rotating: false,
        is_charging_throw: false,
    },
    GrabPowerer {
        is_enabled: false,     // Disabled by default, toggled via ability
        grab_radius: 10.0,     // Large reach for power grabbing
        held_objects: Vec::new(),
        launch_force: 500.0,
        max_launch_force: 3500.0,
        launch_speed: 1200.0,  // Fast object launching
        is_charging: false,
        last_grab_time: 0.0,
    },
    // Ability integration
    AbilitySlot {
        current_ability: Some(AbilityType::Telekinesis),
        ability_active: false,
    },
));

// Boss enemy with area-effect object manipulation
commands.spawn((
    Grabber { /* standard config */ },
    GrabPowerer {
        is_enabled: true,      // Always active for boss
        grab_radius: 15.0,     // Massive control area
        held_objects: Vec::new(),
        launch_force: 2000.0,
        max_launch_force: 8000.0, // Devastating launches
        launch_speed: 2000.0,
        is_charging: false,
        last_grab_time: 0.0,
    },
    AIBehavior::TelekineticBoss,
    PhaseManager::new(vec![
        BossPhase::GrabPhase,
        BossPhase::LaunchPhase,
        BossPhase::Cooldown,
    ]),
));
```

**Puzzle Integration:**
```rust
// Elevated puzzle platform with range-limited power grabber
commands.spawn((
    Grabber { hold_distance: 1.5, mode: GrabMode::Powers, ..default() },
    GrabPowerer {
        is_enabled: true,     // Active when puzzle is active
        grab_radius: 6.0,     // Limited range creates puzzle challenge
        held_objects: Vec::new(),
        launch_force: 200.0, // Gentle launching for precision
        max_launch_force: 400.0,
        launch_speed: 300.0,
        is_charging: false,
        last_grab_time: 0.0,
    },
    PuzzleElement::TelekineticOrbPlacement,
    ActivationZone::new(8.0), // Player must be within 8 units to use
));
```

**Ability Toggle Pattern:**
```rust
// Telekinesis ability activation system
fn toggle_telekinesis(
    input: Res<InputState>,
    mut grab_powerers: Query<&mut GrabPowerer>,
    mut abilities: Query<&mut AbilityState>,
) {
    for (mut powerer, mut ability) in grab_powerers.iter_mut().zip(abilities.iter_mut()) {
        if input.ability_1_pressed && !ability.on_cooldown {
            powerer.is_enabled = !powerer.is_enabled;
            ability.is_active = powerer.is_enabled;
            
            // Drop all objects when disabling
            if !powerer.is_enabled {
                powerer.held_objects.clear();
            }
        }
    }
}
```

**Energy Management Integration:**
```rust
// Power grabber with energy cost per object held
fn manage_telekinesis_energy(
    mut grab_powerers: Query<(&mut GrabPowerer, &ManipulatorStats)>,
    mut energy: Query<&mut EnergyResource>,
) {
    for (powerer, stats) in grab_powerers.iter() {
        if powerer.is_enabled {
            let energy_cost = 
                powerer.held_objects.len() as f32 * stats.energy_per_object_per_second;
            
            // Deduct energy, disable if depleted
            if let Ok(mut energy_res) = energy.get_mut(stats.entity) {
                if energy_res.current >= energy_cost * TIME_STEP {
                    energy_res.current -= energy_cost * TIME_STEP;
                } else {
                    // Force disable due to insufficient energy
                    powerer.is_enabled = false;
                }
            }
        }
    }
}
```

---

### GrabMeleeWeapon

Transforms grabbed objects into functional melee weapons with attacks, blocking, and return-to-thrower capabilities (like boomerangs).

**Attack Configuration:**
- `attacks: Vec<GrabAttackInfo>` - List of available attacks while held as weapon
- `can_block: bool` - Whether object can be used defensively to block attacks
- `block_protection: f32` - Damage reduction percentage (0.0-1.0) when blocking
- `damage_type_id: i32` - Identifier for damage type system integration

**Special Abilities:**
- `can_throw_return: bool` - Object automatically returns after being thrown
- `throw_speed: f32` - Velocity of object when thrown
- `return_speed: f32` - Velocity when returning to thrower

**Attack Structure:**
Each `GrabAttackInfo` contains:
- `name: String` - Display name of attack (e.g., "Overhead Slam")
- `damage: f32` - Base damage amount
- `attack_type: String` - Classification for resistance calculations (e.g., "Blunt", "Piercing")
- `stamina_cost: f32` - Stamina/energy consumption on use
- `duration: f32` - Animation and lockout duration

**Usage Example:**
```rust
// Makeshift weapon - any grabbed object
commands.spawn((
    Grabbable {
        weight: 2.0,
        use_weight: true,
        use_events: false,
        parent_to_grab: None,
    },
    GrabMeleeWeapon {
        attacks: vec![
            GrabAttackInfo {
                name: "Swing".to_string(),
                damage: 15.0,
                attack_type: "Blunt".to_string(),
                stamina_cost: 10.0,
                duration: 0.5,
            },
            GrabAttackInfo {
                name: "Throw".to_string(),
                damage: 25.0,
                attack_type: "Ranged".to_string(),
                stamina_cost: 15.0,
                duration: 0.3,
            },
        ],
        can_block: true,
        block_protection: 0.3, // 30% damage reduction
        can_throw_return: false,
        throw_speed: 15.0,
        return_speed: 20.0,
        damage_type_id: 2, // Blunt weapon type ID
    },
    RigidBody::Dynamic,
    Collider::cuboid(0.8, 0.8, 0.8),
    Mass(2.0),
    // ... visual components
));

// Dedicated throwing weapon with return ability
commands.spawn((
    Grabbable {
        weight: 1.0,
        use_weight: false, // Light weapon, easy to throw
        extra_grab_distance: 2.0, // Can grab from distance
        use_events: true,
        parent_to_grab: None,
    },
    GrabMeleeWeapon {
        attacks: vec![
            GrabAttackInfo {
                name: "Quick Strike".to_string(),
                damage: 12.0,
                attack_type: "Piercing".to_string(),
                stamina_cost: 8.0,
                duration: 0.3,
            },
        ],
        can_block: false,
        block_protection: 0.0,
        can_throw_return: true, // Special return ability
        throw_speed: 25.0,    // Very fast initial throw
        return_speed: 30.0,   // Faster return
        damage_type_id: 3, // Piercing weapon type
    },
    OutlineSettings {
        enabled: true,
        color: Color::srgba(0.0, 1.0, 0.0, 1.0), // Green highlight
        width: 0.05,
        active: false,
    },
    WeaponName::Chakram, // Custom identifier for lore/special effects
    // ... physics and visual components
));

// Two-handed heavy weapon
commands.spawn((
    Grabbable {
        weight: 8.0,
        use_weight: true,
        extra_grab_distance: 0.0,
        use_events: true,
        parent_to_grab: None,
    },
    GrabMeleeWeapon {
        attacks: vec![
            GrabAttackInfo {
                name: "Heavy Slam".to_string(),
                damage: 45.0,          // High damage
                attack_type: "Blunt".to_string(),
                stamina_cost: 35.0,    // High stamina cost
                duration: 1.2,        // Slow windup
            },
            GrabAttackInfo {
                name: "Wide Swing".to_string(),
                damage: 30.0,
                attack_type: "Blunt".to_string(),
                stamina_cost: 25.0,
                duration: 0.8,
            },
        ],
        can_block: true,
        block_protection: 0.6, // Strong blocking
        can_throw_return: false,
        throw_speed: 10.0,    // Heavy, can't throw far
        return_speed: 0.0,
        damage_type_id: 4, // Two-handed blunt type
    },
    TwoHandedWeapon, // Requires both hands, disables other grab
    Mass(8.0),
    // ... visual and physics components
));
```

**Combat System Integration:**
```rust
// Attack execution pattern
fn execute_grab_weapon_attack(
    grabbers: Query<(&Grabber, &GrabMeleeWeapon)>,
    mut combat_events: EventWriter<CombatEvent>,
    targets: Query<&CombatTarget>,
) {
    for (grabber, weapon) in grabbers.iter() {
        if let Some(held_entity) = grabber.held_object {
            if let Ok(target) = targets.get(held_entity) {
                // Use first attack or select based on input
                let attack = &weapon.attacks[0];
                
                combat_events.send(CombatEvent::MeleeAttack {
                    attacker: grabber.entity,
                    target: target.entity,
                    damage: attack.damage,
                    damage_type: attack.attack_type.clone(),
                    stamina_cost: attack.stamina_cost,
                });
            }
        }
    }
}

// Blocking with grabbed weapons
fn handle_weapon_blocking(
    grabbers: Query<(&Grabber, &GrabMeleeWeapon, &Transform)>,
    mut incoming_attacks: EventReader<AttackEvent>,
    mut combat_results: EventWriter<CombatResult>,
) {
    for (grabber, weapon, transform) in grabbers.iter() {
        if weapon.can_block && grabber.held_object.is_some() {
            for attack in incoming_attacks.iter() {
                if attack.target == grabber.entity {
                    // Check if weapon is positioned to block (simplified)
                    if is_weapon_positioned_for_block(transform, attack.direction) {
                        let blocked_damage = attack.damage * (1.0 - weapon.block_protection);
                        combat_results.send(CombatResult::Blocked {
                            original_damage: attack.damage,
                            final_damage: blocked_damage,
                            blocker: grabber.entity,
                        });
                    }
                }
            }
        }
    }
}

// Throw return weapon update
fn update_returning_weapons(
    mut commands: Commands,
    mut grabbers: Query<(&Grabber, &Transform)>,
    mut weapons: Query<(Entity, &mut Transform, &GrabMeleeWeapon), With<ReturningWeapon>>,
    time: Res<Time>,
) {
    for (weapon_entity, mut weapon_transform, weapon) in weapons.iter_mut() {
        if weapon.can_throw_return {
            for (grabber, grabber_transform) in grabbers.iter() {
                // Move weapon back toward thrower
                let direction = (grabber_transform.translation - weapon_transform.translation).normalize();
                weapon_transform.translation += direction * weapon.return_speed * time.delta_seconds();
                
                // Check if returned to grabber
                if weapon_transform.translation.distance(grabber_transform.translation) < 1.0 {
                    commands.entity(weapon_entity).remove::<ReturningWeapon>();
                    // Weapon is now ready to be re-grabbed
                }
            }
        }
    }
}
```

**Lockout and Animation Management:**
```rust
// Handle attack lockout to prevent spam
fn manage_grab_weapon_lockout(
    mut grabbers: Query<&mut Grabber>,
    weapons: Query<&GrabMeleeWeapon>,
    mut attack_states: Query<&mut AttackState>,
    time: Res<Time>,
) {
    for mut grabber in grabbers.iter_mut() {
        if let Some(held) = grabber.held_object {
            if let Ok(weapon) = weapons.get(held) {
                if let Ok(mut attack_state) = attack_states.get_mut(grabber.entity) {
                    if attack_state.is_attacking {
                        // Check if attack duration completed
                        if attack_state.elapsed >= weapon.attacks[0].duration {
                            attack_state.is_attacking = false;
                            grabber.is_rotating = false; // Re-enable rotation
                        } else {
                            attack_state.elapsed += time.delta_seconds();
                        }
                    }
                }
            }
        }
    }
}
```

---

### OutlineSettings

Visual feedback component that highlights grabbable objects with customizable outline effects. Used for player guidance and interaction feedback.

**Visual Properties:**
- `enabled: bool` - Whether outlining is active for this object
- `active: bool` - Whether outline is currently visible (set by proximity system)
- `color: Color` - RGBA color of the outline effect
- `width: f32` - Thickness of the outline (0.01-0.2 typical range)

**Usage Example:**
```rust
// Standard grabbable with yellow highlight
commands.spawn((
    Grabbable::default(),
    OutlineSettings {
        enabled: true,
        active: false, // Will be activated by proximity system
        color: Color::srgba(1.0, 1.0, 0.0, 1.0), // Yellow
        width: 0.05,
    },
    // ... visual and physics components
));

// Important quest item with prominent green outline
commands.spawn((
    Grabbable {
        extra_grab_distance: 2.0, // Easy to grab
        ..default()
    },
    OutlineSettings {
        enabled: true,
        active: false,
        color: Color::srgba(0.0, 1.0, 0.3, 1.0), // Bright green
        width: 0.08, // Thicker outline
    },
    QuestItem::KeyOfEternalLight,
    // ... other components
));

// Locked/unlockable puzzle piece
let puzzle_piece = commands.spawn((
    Grabbable {
        use_weight: false, // Weight doesn't matter for puzzle
        use_events: true,  // Enable puzzle state tracking
        ..default()
    },
    OutlineSettings {
        enabled: true,
        active: false,
        color: Color::srgba(0.5, 0.5, 1.0, 1.0), // Blue
        width: 0.04,
    },
    PuzzlePiece::DoorMechanism(3), // This is piece #3
    // ... components
)).id();

// Update outline color based on game state
fn update_puzzle_piece_outlines(
    mut outlines: Query<&mut OutlineSettings>,
    puzzle_state: Res<PuzzleState>,
) {
    for mut outline in outlines.iter_mut() {
        if puzzle_state.pieces_placed.contains(&piece_id) {
            outline.color = Color::srgba(0.0, 1.0, 0.0, 1.0); // Green = placed
            outline.active = true; // Always visible once placed
        } else if puzzle_state.available_pieces.contains(&piece_id) {
            outline.color = Color::srgba(1.0, 0.5, 0.0, 1.0); // Orange = available
        } else {
            outline.color = Color::srgba(0.3, 0.3, 0.3, 1.0); // Gray = locked
            outline.active = false;
        }
    }
}
```

---

### ObjectToPlace & PutObjectSystem

Advanced placement system for precise object positioning in slots, puzzle receptors, and inventory containers. Supports dynamic movement smoothing and activation events.

#### ObjectToPlace

Attached to movable objects that can be slotted into specific positions.

**Identity:**
- `object_name: String` - Unique identifier for matching with slots (e.g., "BlueGem", "Keycard_1234")

**Placement State:**
- `is_placed: bool` - Current placement status (set by placement system)

**Event Control:**
- `can_call_placed_event: bool` - Whether to trigger events on successful placement
- `can_call_removed_event: bool` - Whether to trigger events on removal from slot

**Usage Example:**
```rust
// Puzzle key for specific door
commands.spawn((
    Grabbable {
        weight: 0.5,
        use_events: true,
        ..default()
    },
    ObjectToPlace {
        object_name: "CathedralKey".to_string(),
        is_placed: false,
        can_call_placed_event: true,  // Enable door unlocking
        can_call_removed_event: false, // Can't remove once placed
    },
    OutlineSettings {
        enabled: true,
        active: false,
        color: Color::srgba(1.0, 0.84, 0.0, 1.0), // Gold
        width: 0.06,
    },
    // ... visual/physics components
));

// Generic inventory item that can go into container
commands.spawn((
    Grabbable {
        weight: 0.2,
        use_events: false,
        ..default()
    },
    ObjectToPlace {
        object_name: "GenericResource".to_string(),
        is_placed: false,
        can_call_placed_event: true,  // Enable inventory updates
        can_call_removed_event: true, // Can take back out of container
    },
    InventoryItem::Resource(ResourceType::TechScrap),
    // ... components
));
```

#### PutObjectSystem

Component for slots, receptors, containers where objects can be placed.

**Object Matching:**
- `use_certain_object: bool` - Whether slot accepts only specific object (vs accepting any valid object)
- `certain_object_to_place: Option<Entity>` - Specific required object entity
- `object_name_to_place: String` - Object name for name-based matching
- `current_object_placed: Option<Entity>` - Currently placed object (None if empty)

**Placement Target:**
- `place_to_put: Option<Entity>` - Child transform where object should be positioned
- `position_speed: f32` - Speed of smooth movement to target position
- `rotation_speed: f32` - Speed of rotation alignment

**Physics & Validation:**
- `max_distance_to_place: f32` - Maximum range for placement to begin
- `is_object_placed: bool` - Current placement state
- `disable_object_on_place: bool` - Whether to disable object physics/components after placement

**Usage Example:**
```rust
// Specific keycard reader
let reader_slot = commands.spawn((
    PutObjectSystem {
        use_certain_object: true, // Only specific cards work
        certain_object_to_place: None, // Will be set at runtime
        object_name_to_place: "SecLevel3_Card".to_string(), // Must match
        current_object_placed: None,
        place_to_put: Some(slot_transform_entity), // Visual slot position
        position_speed: 8.0, // Smooth insertion animation
        rotation_speed: 12.0,
        max_distance_to_place: 1.5, // Must be close to insert
        is_object_placed: false,
        disable_object_on_place: true, // Lock card in place
    },
    Device::CardReader,
    OutlineSettings {
        enabled: true,
        active: false,
        color: Color::srgba(1.0, 0.0, 0.0, 1.0), // Red highlight
        width: 0.04,
    },
    // ... visual components
)).id();

// Generic inventory container
commands.spawn((
    PutObjectSystem {
        use_certain_object: false, // Accepts any object
        certain_object_to_place: None,
        object_name_to_place: "GenericResource".to_string(),
        current_object_placed: None,
        place_to_put: None, // No specific placement target
        position_speed: 10.0,
        rotation_speed: 10.0,
        max_distance_to_place: 2.0,
        is_object_placed: false,
        disable_object_on_place: true, // Items stored in inventory
    },
    InventoryContainer {
        capacity: 20,
        current_items: Vec::new(),
    },
    // ... visual/container components
));

// Puzzle receptor that accepts multiple possible pieces
commands.spawn((
    PutObjectSystem {
        use_certain_object: false, // Flexible: multiple pieces fit
        certain_object_to_place: None,
        object_name_to_place: "PuzzleOrb_Any".to_string(), // Broad category
        current_object_placed: None,
        place_to_put: Some(orb_socket_entity),
        position_speed: 6.0, // Slower for dramatic placement
        rotation_speed: 8.0,
        max_distance_to_place: 0.8, // Must be very close/placed
        is_object_placed: false,
        disable_object_on_place: false, // Puzzle pieces remain active
    },
    PuzzleReceptor::OrbSlot(3), // Slot #3 of puzzle
    ActivationTrigger::PuzzleSolvedOrb,
    // ... components
));
```

**Placement System Example:**
```rust
// Process placement attempts
fn handle_object_placement(
    mut placement_attempts: EventReader<PlacementAttempt>,
    mut objects: Query<(&ObjectToPlace, &Transform)>,
    mut slots: Query<(Entity, &mut PutObjectSystem, &Transform)>,
    mut commands: Commands,
) {
    for attempt in placement_attempts.iter() {
        if let Ok((object, object_transform)) = objects.get(attempt.object) {
            for (slot_entity, mut slot, slot_transform) in slots.iter_mut() {
                // Check distance first
                let distance = object_transform.translation
                    .distance(slot_transform.translation);
                
                if distance <= slot.max_distance_to_place {
                    // Check name matching
                    let name_matches = !slot.use_certain_object ||
                        object.object_name == slot.object_name_to_place;
                    
                    if name_matches && !slot.is_object_placed {
                        // Valid placement, start positioning
                        slot.current_object_placed = Some(attempt.object);
                        slot.is_object_placed = true;
                        
                        // Trigger placed event if enabled
                        if object.can_call_placed_event {
                            commands.trigger(ObjectPlacedEvent {
                                object: attempt.object,
                                slot: slot_entity,
                                object_name: object.object_name.clone(),
                            });
                        }
                    }
                }
            }
        }
    }
}

// Smooth positioning system
fn update_placed_objects(
    mut slots: Query<(&PutObjectSystem, &Transform)>,
    mut objects: Query<&mut Transform, (With<ObjectToPlace>, Without<PutObjectSystem>)>,
    time: Res<Time>,
) {
    for (slot, slot_transform) in slots.iter() {
        if let Some(object_entity) = slot.current_object_placed {
            if let Ok(mut object_transform) = objects.get_mut(object_entity) {
                if let Some(target_transform) = slot.place_to_put {
                    // Get target transform from child entity
                    let target_translation = slot_transform.translation; // Simplified
                    let target_rotation = slot_transform.rotation;
                    
                    // Smooth movement to target
                    object_transform.translation = object_transform.translation
                        .lerp(target_translation, slot.position_speed * time.delta_seconds());
                    
                    object_transform.rotation = object_transform.rotation
                        .slerp(target_rotation, slot.rotation_speed * time.delta_seconds());
                }
            }
        }
    }
}
```

**Dynamic Slot Activation:**
```rust
// Enable slot only when player has required ability/item
fn manage_slot_availability(
    mut slots: Query<(&mut PutObjectSystem, &SlotRequirements)>,
    player_inventory: Query<&Inventory, With<Player>>,
) {
    if let Ok(inventory) = player_inventory.get_single() {
        for (mut slot, requirements) in slots.iter_mut() {
            let has_requirement = match &requirements.item_required {
                Some(item_id) => inventory.contains(*item_id),
                None => true, // No requirement
            };
            
            // Bypass distance check if requirement met
            if has_requirement {
                slot.max_distance_to_place = f32::MAX; // Can place from anywhere
            } else {
                slot.max_distance_to_place = 0.0; // Cannot place at all
            }
        }
    }
}
```

---

### GrabPhysicalObjectSettings

Advanced physics configuration for grabbed objects, enabling runtime modification of physical properties during grab states.

**Grab Physics Control:**
- `grab_physically: bool` - Whether to use physical constraints or position manipulation
- `disable_collider_on_grab: bool` - Temporarily remove collision during grab (prevents jitter)

**Mass Modification:**
- `set_mass: bool` - Whether to modify object mass on grab/drop
- `mass_value: f32` - Mass to set during grab (restored on drop)

**Tag Management:**
- `tag_when_active: String` - Physics tag to apply when grabbed
- `tag_when_inactive: String` - Physics tag when not grabbed

**Usage Example:**
```rust
// Standard physics object with collision toggle
commands.spawn((
    Grabbable {
        weight: 3.0,
        use_events: true,
        ..default()
    },
    GrabPhysicalObjectSettings {
        grab_physically: true,
        set_mass: false,
        mass_value: 1.0,
        tag_when_active: "GrabbedObject".to_string(),
        tag_when_inactive: "WorldPhysics".to_string(),
        disable_collider_on_grab: true, // Prevent grab jitter
    },
    RigidBody::Dynamic,
    Collider::cuboid(1.0, 1.0, 1.0),
    Mass(3.0),
    Restitution(0.3),
    Friction(0.5),
));

// Heavy object that becomes light when grabbed for easy manipulation
commands.spawn((
    Grabbable {
        weight: 10.0,
        use_weight: true,
        ..default()
    },
    GrabPhysicalObjectSettings {
        grab_physically: true,
        set_mass: true,      // Modify mass when grabbed
        mass_value: 0.5,     // Become very light
        tag_when_active: "LightweightGrabbed".to_string(),
        tag_when_inactive: "HeavyWorldObject".to_string(),
        disable_collider_on_grab: false,
    },
    RigidBody::Dynamic,
    Collider::cuboid(2.0, 2.0, 2.0),
    Mass(10.0),
));

// Precision puzzle element with no physics simulation
commands.spawn((
    Grabbable {
        weight: 0.1,
        use_weight: false,
        use_events: true,
        extra_grab_distance: 1.0,
    },
    GrabPhysicalObjectSettings {
        grab_physically: false, // Position-based for precision
        set_mass: false,
        mass_value: 1.0,
        tag_when_active: "Grabbed".to_string(),
        tag_when_inactive: "PuzzleDefault".to_string(),
        disable_collider_on_grab: true,
    },
    PuzzlePiece::Orb,
    // No RigidBody - pure transform manipulation
));
```

**Mass Restoration Pattern:**
```rust
// Restore original mass on drop
fn restore_object_physics(
    mut grab_events: EventReader<GrabEvent>,
    mut objects: Query<(&mut Mass, &OriginalMass)>,
) {
    for event in grab_events.iter() {
        if let GrabEvent::Drop(_, dropped_entity) = event {
            if let Ok((mut mass, original)) = objects.get_mut(*dropped_entity) {
                mass.0 = original.mass; // Restore saved mass
            }
        }
    }
}
```

---

## System Integration

### Combat System Integration

The Grab System integrates seamlessly with the Combat System, enabling grabbed objects to function as improvised or dedicated melee weapons.

**Improvised Weapon Pattern:**
```rust
// Any grabbable object can become a weapon
commands.spawn((
    Grabbable {
        weight: 1.5,
        ..default()
    },
    RigidBody::Dynamic,
    Mass(1.5),
    Collider::cuboid(0.5, 0.5, 0.5),
    // Optional weapon component for enhanced functionality
    GrabMeleeWeapon {
        attacks: vec![
            GrabAttackInfo {
                name: "Bludgeon".to_string(),
                damage: 8.0 + 1.5, // Base + weight modifier
                attack_type: "Blunt".to_string(),
                stamina_cost: 12.0,
                duration: 0.6,
            },
        ],
        can_block: true,
        block_protection: 0.2, // Light protection
        can_throw_return: false,
        ..default()
    },
));

// Weapon selection based on what's grabbed
fn select_grabbed_weapon_attack(
    grabbers: Query<(&Grabber, &CombatStance)>,
    weapons: Query<(&GrabMeleeWeapon, &Mass)>,
    mut attack_queue: ResMut<AttackQueue>,
) {
    for (grabber, stance) in grabbers.iter() {
        if let Some(held) = grabber.held_object {
            if let Ok((weapon, mass)) = weapons.get(held) {
                // Choose attack based on combat stance
                let attack_index = match stance.current {
                    CombatStance::Aggressive => 0, // Heavy attack
                    CombatStance::Defensive => 1,  // Quick attack
                    CombatStance::Balanced => 2,   // Medium attack
                };
                
                if let Some(attack) = weapon.attacks.get(attack_index) {
                    attack_queue.push(CombatAction::Melee {
                        damage: attack.damage + mass.0 * 2.0, // Weight bonus
                        range: 2.5,
                        angle: 90.0,
                        stamina_cost: attack.stamina_cost,
                    });
                }
            }
        }
    }
}
```

**Block System Integration:**
```rust
// Weapons can block incoming attacks
fn handle_weapon_blocking(
    grabbers: Query<(&Grabber, &Transform, &CombatStats)>,
    weapons: Query<&GrabMeleeWeapon>,
    mut incoming_attacks: EventReader<AttackEvent>,
    mut blocked_attacks: EventWriter<BlockedAttack>,
    mut damage_events: EventWriter<DamageEvent>,
) {
    for attack in incoming_attacks.iter() {
        // Find defending entity's grabber
        if let Ok((grabber, defender_transform, stats)) = grabbers.get(attack.target) {
            // Check if holding a weapon that can block
            if let Some(held) = grabber.held_object {
                if let Ok(weapon) = weapons.get(held) {
                    if weapon.can_block {
                        // Check if weapon is positioned to block
                        let block_angle = calculate_block_angle(
                            defender_transform,
                            attack.direction,
                            grabber.hold_distance
                        );
                        
                        if block_angle < stats.block_angle_threshold {
                            // Successful block
                            let blocked_damage = attack.damage * (1.0 - weapon.block_protection);
                            
                            blocked_attacks.send(BlockedAttack {
                                attacker: attack.attacker,
                                defender: attack.target,
                                original_damage: attack.damage,
                                blocked_amount: attack.damage - blocked_damage,
                                weapon_held: held,
                            });
                            
                            // Lingering block damage (if any)
                            if blocked_damage > 0.0 {
                                damage_events.send(DamageEvent {
                                    target: attack.target,
                                    amount: blocked_damage,
                                    damage_type: attack.damage_type.clone(),
                                    source: attack.attacker,
                                });
                            }
                        }
                    }
                }
            }
        }
    }
}
```

### Puzzle System Integration

Grab mechanics are fundamental to puzzle interactions, enabling physics-based solutions and object manipulation challenges.

**Pressure Plate Puzzle:**
```rust
// Weight-sensitive pressure plate
commands.spawn((
    PutObjectSystem {
        use_certain_object: false, // Any object works
        object_name_to_place: "AnyObject".to_string(),
        current_object_placed: None,
        place_to_put: Some(plate_surface_transform),
        max_distance_to_place: 1.0,
        is_object_placed: false,
        disable_object_on_place: false, // Object can be moved later
        position_speed: 5.0,
        rotation_speed: 5.0,
    },
    PressurePlate {
        required_weight: 3.0, // Minimum weight to activate
        current_weight: 0.0,
        activated: false,
        activation_targets: vec![door_entity, light_entity],
    },
    OutlineSettings {
        enabled: true,
        active: false,
        color: Color::srgba(0.0, 1.0, 0.0, 1.0), // Green when active
        width: 0.03,
    },
));

// Weight calculation system
fn update_pressure_plates(
    mut plates: Query<(&mut PutObjectSystem, &mut PressurePlate)>,
    mut placed_objects: Query<(&Grabbable, &Mass)>,
    mut activation_events: EventWriter<PlateActivation>,
) {
    for (mut slot, mut plate) in plates.iter_mut() {
        if let Some(object) = slot.current_object_placed {
            if let Ok((grabbable, mass)) = placed_objects.get(object) {
                let effective_weight = if grabbable.use_weight {
                    mass.0
                } else {
                    grabbable.weight
                };
                
                plate.current_weight = effective_weight;
                
                // Check activation threshold
                let should_activate = effective_weight >= plate.required_weight;
                
                if should_activate && !plate.activated {
                    plate.activated = true;
                    activation_events.send(PlateActivation {
                        plate: slot_entity,
                        weight: effective_weight,
                        objects: vec![object],
                    });
                } else if !should_activate && plate.activated {
                    plate.activated = false;
                    activation_events.send(PlateDeactivation {
                        plate: slot_entity,
                        weight: effective_weight,
                    });
                }
            }
        } else {
            plate.current_weight = 0.0;
            if plate.activated {
                plate.activated = false;
                activation_events.send(PlateDeactivation {
                    plate: slot_entity,
                    weight: 0.0,
                });
            }
        }
    }
}
```

**Orb Placement Puzzle (Sequence-Based):**
```rust
// Multi-slot orb puzzle with correct order
#[derive(Component)]
struct OrbPuzzleState {
    slots: Vec<Entity>,
    correct_sequence: Vec<String>, // Object names in order
    current_placement: Vec<String>,
    solved: bool,
}

// Spawn puzzle slots
let slot_positions = vec![vec3(0, 0, 0), vec3(2, 0, 0), vec3(4, 0, 0), vec3(6, 0, 0)];
let mut slot_entities = Vec::new();

for (index, position) in slot_positions.iter().enumerate() {
    let slot = commands.spawn((
        PutObjectSystem {
            use_certain_object: false, // Accepts any orb
            object_name_to_place: "ColorOrb".to_string(),
            current_object_placed: None,
            place_to_put: Some(commands.spawn(Transform::from_translation(*position)).id()),
            max_distance_to_place: 1.5,
            is_object_placed: false,
            disable_object_on_place: true,
            position_speed: 8.0,
            rotation_speed: 10.0,
        },
        OrbSlot { index, orb_type: OrbType::Any },
        OutlineSettings {
            enabled: true,
            active: false,
            color: Color::srgba(1.0, 1.0, 1.0, 1.0), // White when empty
            width: 0.04,
        },
    )).id();
    
    slot_entities.push(slot);
}

// State management and validation
fn validate_orb_sequence(
    mut puzzles: Query<&mut OrbPuzzleState>,
    slots: Query<(&PutObjectSystem, &OrbSlot)>,
    objects: Query<&ObjectToPlace>,
    mut solved_events: EventWriter<PuzzleSolved>,
) {
    for mut puzzle in puzzles.iter_mut() {
        if puzzle.solved { continue; }
        
        puzzle.current_placement.clear();
        
        // Collect objects in order of slots
        for slot_entity in &puzzle.slots {
            if let Ok((slot_system, slot)) = slots.get(*slot_entity) {
                if let Some(object_entity) = slot_system.current_object_placed {
                    if let Ok(object) = objects.get(object_entity) {
                        puzzle.current_placement.push(object.object_name.clone());
                    }
                } else {
                    puzzle.current_placement.push("Empty".to_string());
                }
            }
        }
        
        // Validate sequence
        if puzzle.current_placement == puzzle.correct_sequence {
            puzzle.solved = true;
            solved_events.send(PuzzleSolved {
                puzzle: puzzle_entity,
                sequence: puzzle.current_placement.clone(),
            });
            
            // Trigger rewards, door opening, etc.
        }
    }
}
```

**Physics-Based Puzzle (Counterweight):**
```rust
// Lever/pulley puzzle requiring counterbalance
commands.spawn((
    PutObjectSystem {
        use_certain_object: false, // Place counterweights
        object_name_to_place: "Counterweight".to_string(),
        current_object_placed: None,
        place_to_put: Some(basket_transform),
        max_distance_to_place: 2.0,
        is_object_placed: false,
        disable_object_on_place: false, // Weights can be moved
        position_speed: 5.0,
        rotation_speed: 5.0,
    },
    CounterweightSystem {
        required_weight: 10.0,
        current_weight: 0.0,
        lift_height: 0.0,
        target_height: 8.0,
        lift_speed: 0.02,
    },
    Joint::new() // Physical pulley joint
        .with_parent(crate_entity)
        .with_local_anchor1(Vec3::new(0.0, 4.0, 0.0))
        .with_local_anchor2(Vec3::new(0.0, -1.0, 0.0))
        .with_motor_velocity(0.0, 100.0)
));

// Counterweight physics
fn update_counterweights(
    mut counterweights: Query<(&mut PutObjectSystem, &mut CounterweightSystem)>,
    objects: Query<(&Grabbable, &Mass)>,
    mut visual_lift: Query<&mut Transform, With<PulleyVisual>>,
) {
    for (slot, mut counterweight) in counterweights.iter_mut() {
        // Calculate current weight in basket
        let mut total_weight = 0.0;
        if let Some(object) = slot.current_object_placed {
            if let Ok((grabbable, mass)) = objects.get(object) {
                total_weight = if grabbable.use_weight {
                    mass.0
                } else {
                    grabbable.weight
                };
            }
        }
        
        counterweight.current_weight = total_weight;
        
        // Calculate lift progress
        let weight_ratio = (total_weight / counterweight.required_weight).min(1.0);
        let target_lift = weight_ratio * counterweight.target_height;
        
        // Smooth lift interpolation
        counterweight.lift_height = counterweight.lift_height
            .lerp(target_lift, counterweight.lift_speed);
        
        // Apply lift to visual representation
        for mut transform in visual_lift.iter_mut() {
            transform.translation.y = counterweight.lift_height;
        }
    }
}
```

### Stealth System Integration

Grab mechanics integrate with stealth gameplay through sound, visibility, and detection radius management.

**Silent Grabbing:**
```rust
// Thief character with quiet manipulation
commands.spawn((
    Grabber {
        hold_speed: 6.0, // Slow movement = less sound
        rotation_speed: 4.0,
        throw_force: 300.0, // Weak throws
        mode: GrabMode::Powers, // Precise control
        ..default()
    },
    StealthProfile {
        movement_noise_radius: 3.0,
        grab_noise_modifier: 0.5, // Halved noise when grabbing
        throw_noise_radius: 8.0,
    },
    SoundEmitter {
        base_volume: 0.2,
        sound_types: vec![SoundType::Movement, SoundType::Grab],
    },
));

// Sound emission based on grab activity
fn emit_grab_sounds(
    grabbers: Query<(&Grabber, &StealthProfile, &Transform)>,
    mut sound_events: EventWriter<SoundEvent>,
) {
    for (grabber, stealth, transform) in grabbers.iter() {
        let base_radius = stealth.movement_noise_radius;
        let grab_modifier = stealth.grab_noise_modifier;
        
        // Emit grab sounds
        if grabber.is_rotating {
            sound_events.send(SoundEvent {
                position: transform.translation,
                radius: base_radius * grab_modifier * 0.7, // Rotation is quieter
                sound_type: SoundType::Movement,
                intensity: 0.3,
            });
        }
        
        // Emit throw charge sounds
        if grabber.is_charging_throw {
            sound_events.send(SoundEvent {
                position: transform.translation,
                radius: base_radius * 1.5, // Charging is noticeable
                sound_type: SoundType::Grab,
                intensity: 0.6,
            });
        }
        
        // Drop/throw impact sounds are handled by physics/contact systems
    }
}
```

**Visibility Management:**
```rust
// Held objects can reveal player position
fn update_held_object_visibility(
    grabbers: Query<&Grabber>,
    stealthed_entities: Query<&Visibility, With<Stealth>>, 
    mut held_objects: Query<(Entity, &mut Visibility), Without<Stealth>>,
) {
    for grabber in grabbers.iter() {
        if let Some(held) = grabber.held_object {
            if let Ok((object_entity, mut visibility)) = held_objects.get_mut(held) {
                // Inherit stealth from grabber
                if stealthed_entities.contains(grabber.entity) {
                    visibility.is_visible = false; // Hidden while in stealth
                } else {
                    visibility.is_visible = true;
                }
            }
        }
    }
}

// AI perception of grabbed objects
fn ai_perception_grabbed_objects(
    grabbers: Query<(&Grabber, &Team)>,
    aipers: Query<(&AIAgent, &Transform)>,
    objects: Query<&Transform>,
    mut ai_targets: ResMut<AITargetList>,
) {
    for (ai_agent, ai_transform) in aipers.iter() {
        for (grabber, team) in grabbers.iter() {
            if team.id != ai_agent.team { // Enemy team
                if let Some(held) = grabber.held_object {
                    if let Ok(held_transform) = objects.get(held) {
                        let distance = ai_transform.translation
                            .distance(held_transform.translation);
                        
                        if distance < ai_agent.detection_range * 1.5 {
                            // Can see/suspect held object even if grabber is hidden
                            ai_targets.add_suspected_position(
                                held_transform.translation,
                                AlertLevel::Suspicious,
                            );
                        }
                    }
                }
            }
        }
    }
}
```

---

## Advanced Features

### Physics Modes

Two distinct physics implementations provide different gameplay feels:

**Realistic Mode (Spring Physics):**
```rust
commands.spawn((
    Grabber {
        mode: GrabMode::Realistic,
        hold_distance: 2.0,
        hold_speed: 10.0, // Spring force
        max_hold_distance: 4.0,
        ..default()
    },
    GrabPhysicalObjectSettings {
        grab_physically: true, // Use constraints
        set_mass: true,
        mass_value: 0.5, // Lighten when grabbed for easier control
        disable_collider_on_grab: false, // Keep collision
    },
));

// Realistic mode provides:
// - Natural momentum and inertia
// - Objects respond to collisions while held
// - Physics-based swinging motion
// - Velocity-based throwing (momentum inherited)
// - More immersive for survival/horror games
```

**Powers Mode (Direct Positioning):**
```rust
commands.spawn((
    Grabber {
        mode: GrabMode::Powers,
        hold_distance: 2.0,
        hold_speed: 20.0, // Instant following
        max_hold_distance: 3.0, // Tighter leash
        ..default()
    },
    GrabPhysicalObjectSettings {
        grab_physically: false, // Position manipulation
        set_mass: false,
        disable_collider_on_grab: true, // Prevent jitter
    },
));

// Powers mode provides:
// - Instant, precise object control
// - No momentum or inertia
// - Perfect for puzzle games
// - Consistent behavior
// - Better for telekinesis/superpower gameplay
```

### Throwing Mechanics

Comprehensive throwing system with charge-up, trajectory prediction, and physics inheritance.

**Basic Throw:**
```rust
// Hold R to charge, release to throw
pub fn handle_throwing_input(
    input: Res<InputState>,
    time: Res<Time>,
    mut grabbers: Query<&mut Grabber>,
) {
    for mut grabber in grabbers.iter_mut() {
        if input.throw_key_pressed {
            grabber.is_charging_throw = true;
            
            // Accumulate throw force over time
            grabber.throw_force = (grabber.throw_force + 200.0 * time.delta_seconds())
                .min(grabber.max_throw_force);
        }
        
        if input.throw_key_released && grabber.is_charging_throw {
            if let Some(held) = grabber.held_object {
                // Calculate throw direction (forward vector)
                let direction = grabber.forward_vector(); // Custom method for forward direction
                
                // Queue throw event
                event_queue.0.push(GrabEvent::Throw(
                    grabber.entity,
                    held,
                    direction,
                    grabber.throw_force,
                ));
                
                // Reset throw state
                grabber.held_object = None;
                grabber.is_charging_throw = false;
                grabber.throw_force = grabber.max_throw_force * 0.25; // Reset to 25% of max
            }
        }
        
        // Reset on drop
        if grabber.is_charging_throw && grabber.held_object.is_none() {
            grabber.is_charging_throw = false;
            grabber.throw_force = grabber.max_throw_force * 0.25;
        }
    }
}
```

**Throw Event Processing:**
```rust
pub fn process_throw_events(
    mut event_queue: ResMut<GrabEventQueue>,
    mut physics_objects: Query<(Entity, &mut LinearVelocity, &mut Mass, &Transform)>,
    weapon_components: Query<&GrabMeleeWeapon>,
) {
    let events: Vec<GrabEvent> = event_queue.0.drain(..).collect();
    
    for event in events {
        if let GrabEvent::Throw(grabber, object, direction, force) = event {
            if let Ok((entity, mut velocity, mass, transform)) = physics_objects.get_mut(object) {
                // Apply throw velocity
                let throw_velocity = direction.normalize() * force * THROW_FORCE_MULTIPLIER;
                velocity.0 = throw_velocity;
                
                // Check if returning weapon
                if let Ok(weapon) = weapon_components.get(object) {
                    if weapon.can_throw_return {
                        // Add returning component
                        commands.entity(object).insert(ReturningWeapon {
                            return_to: grabber,
                            return_speed: weapon.return_speed,
                            active: true,
                        });
                    }
                }
                
                // Enable collision if it was disabled during grab
                mass.0 = mass.0.max(0.1); // Restore minimum mass
                // collider.enable(); // Uncomment when collision toggling is implemented
            }
        }
    }
}
```

**Trajectory Prediction UI:**
```rust
// Show throw arc preview during charge
fn draw_throw_prediction(
    grabbers: Query<(&Grabber, &Transform), With<ShowThrowPrediction>>,
    gizmos: &mut Gizmos,
) {
    for (grabber, transform) in grabbers.iter() {
        if grabber.is_charging_throw {
            let direction = transform.forward();
            let velocity = direction * grabber.throw_force;
            
            // Simulate projectile arc
            let mut position = transform.translation + transform.forward() * grabber.hold_distance;
            let mut sim_velocity = velocity;
            let gravity = Vec3::new(0.0, -9.81, 0.0);
            
            for _ in 0..30 { // Draw 30 segments of the arc
                let next_position = position + sim_velocity * 0.05; // 50ms steps
                gizmos.line(position, next_position, Color::srgba(1.0, 1.0, 0.0, 0.5));
                
                position = next_position;
                sim_velocity += gravity * 0.05;
                
                // Stop drawing if hitting ground
                if position.y < 0.0 { break; }
            }
        }
    }
}
```

### Object Rotation

Precision rotation mechanics for held objects, essential for puzzle-solving and tactical positioning.

**Rotation Input Handling:**  
```rust
// Right-click + mouse movement to rotate held object
fn handle_rotation_input(
    input: Res<InputState>,
    mut grabbers: Query<&mut Grabber>,
) {
    for mut grabber in grabbers.iter_mut() {
        if input.secondary_action_pressed {
            if grabber.held_object.is_some() {
                grabber.is_rotating = true;
            }  
        } else {
            grabber.is_rotating = false;
        }
    }
}

// Apply rotation to held object
fn update_held_object_rotation(
    grabbers: Query<(&Grabber, &Transform)>,
    input: Res<InputState>,
    mut held_objects: Query<&mut Transform, (With<Grabbable>, Without<Grabber>)>,
) {
    for (grabber, grabber_transform) in grabbers.iter() {
        if let (Some(held_entity), true) = (grabber.held_object, grabber.is_rotating) {
            if let Ok(mut held_transform) = held_objects.get_mut(held_entity) {
                // Get mouse delta for rotation
                let mouse_delta = input.mouse_delta;
                let rotation_sensitivity = grabber.rotation_speed * 0.01;
                
                // Apply rotation around local axes
                let x_rotation = Quat::from_rotation_x(-mouse_delta.y * rotation_sensitivity);
                let y_rotation = Quat::from_rotation_y(-mouse_delta.x * rotation_sensitivity);
                
                held_transform.rotation = y_rotation * held_transform.rotation * x_rotation;
            }
        }
    }
}
```

**Rotation Constraints:**
```rust
// Optional rotation limiting for puzzle elements
fn constrain_object_rotation(
    mut objects: Query<(&mut Transform, &RotationConstraints), With<Grabbable>>,
) {
    for (mut transform, constraints) in objects.iter_mut() {
        if let Some(held_entity) = transform.entity {
            // Get current rotation as Euler angles
            let (current_x, current_y, current_z) = transform.rotation.to_euler(EulerRot::XYZ);
            
            // Constrain each axis
            let clamped_x = current_x.clamp(constraints.min_x, constraints.max_x);
            let clamped_y = current_y.clamp(constraints.min_y, constraints.max_y);
            let clamped_z = current_z.clamp(constraints.min_z, constraints.max_z);
            
            // Apply constraints if needed
            if current_x != clamped_x || current_y != clamped_y || current_z != clamped_z {
                transform.rotation = Quat::from_euler(EulerRot::XYZ, clamped_x, clamped_y, clamped_z);
            }
        }
    }
}
```

### Multi-Object Power Grabbing

Telekinesis-style object manipulation with multiple simultaneous targets.

**Activation and Object Collection:**
```rust
// Power grabbing system with radius-based object collection
fn handle_power_grab_input(
    input: Res<InputState>,
    mut powerers: Query<&mut GrabPowerer>,
) {
    for mut powerer in powerers.iter_mut() {
        if input.ability_2_pressed && powerer.is_enabled {
            // Toggle power grab
            if powerer.held_objects.is_empty() {
                // Find nearby grabbable objects
                powerer.is_collecting = true;
            } else {
                // Release all held objects
                for held in powerer.held_objects.drain(..) {
                    // Send drop event for each
                }
            }
        }
    }
}

// Collect objects within radius
fn collect_nearby_objects(
    mut powerers: Query<(&mut GrabPowerer, &Transform)>,
    grabbables: Query<(Entity, &Transform), With<Grabbable>>,
    mut event_queue: ResMut<GrabEventQueue>,
) {
    for (mut powerer, powerer_transform) in powerers.iter_mut() {
        if powerer.is_collecting {
            for (grabbable_entity, grabbable_transform) in grabbables.iter() {
                // Skip already held objects
                if powerer.held_objects.contains(&grabbable_entity) {
                    continue;
                }
                
                // Check distance within collection radius
                let distance = powerer_transform.translation
                    .distance(grabbable_transform.translation);
                
                if distance <= powerer.grab_radius {
                    // Queue grab event
                    event_queue.0.push(GrabEvent::Grab(
                        powerer.entity,
                        grabbable_entity
                    ));
                    
                    powerer.held_objects.push(grabbable_entity);
                }
            }
            
            powerer.is_collecting = false;
        }
    }
}
```

**Multi-Object Position Management:**
```rust
// Update positions of multiple held objects (orbital pattern)
fn update_power_held_objects(
    powerers: Query<(&GrabPowerer, &Transform)>,
    mut held_objects: Query<&mut Transform, (With<Grabbable>, Without<GrabPowerer>)>,
) {
    for (powerer, powerer_transform) in powerers.iter() {
        if powerer.is_enabled && !powerer.held_objects.is_empty() {
            for (index, held_entity) in powerer.held_objects.iter().enumerate() {
                if let Ok(mut held_transform) = held_objects.get_mut(*held_entity) {
                    // Arrange objects in circle/orbit around powerer
                    let angle = (index as f32 / powerer.held_objects.len() as f32) * TAU;
                    let radius = 3.0; // Orbital distance
                    
                    let offset = Vec3::new(
                        angle.cos() * radius,
                        (index as f32 * 0.5).sin(), // Slight vertical variation
                        angle.sin() * radius
                    );
                    
                    let target_position = powerer_transform.translation + offset;
                    held_transform.translation = target_position;
                    
                    // Make objects face outward
                    held_transform.look_at(
                        held_transform.translation + offset.normalize(),
                        Vec3::Y
                    );
                }
            }
        }
    }
}

// Launch all held objects at once
fn handle_power_throw(
    input: Res<InputState>,
    mut powerers: Query<(&mut GrabPowerer, &Transform)>,
    mut event_queue: ResMut<GrabEventQueue>,
) {
    for (mut powerer, transform) in powerers.iter_mut() {
        if input.throw_key_pressed && !powerer.held_objects.is_empty() {
            // Calculate direction (mouse position or forward)
            let direction = (input.mouse_world_position - transform.translation)
                .normalize_or(transform.forward());
            
            // Launch each object with slight spread
            for (index, held) in powerer.held_objects.iter().enumerate() {
                let spread = Quat::from_rotation_y((index as f32 - 1.0) * 0.1);
                let launch_direction = spread * direction;
                
                event_queue.0.push(GrabEvent::Throw(
                    powerer.entity,
                    *held,
                    launch_direction,
                    powerer.launch_force
                ));
            }
            
            // Clear held objects
            powerer.held_objects.clear();
            powerer.launch_force = powerer.max_launch_force * 0.3; // Reset
        }
    }
}
```

---

## Best Practices

### Performance Optimization

**Object Pooling for Grabbables:**
```rust
// Pre-spawn common grabbable objects
fn setup_grabbable_pool(mut commands: Commands) {
    for i in 0..50 {
        commands.spawn((
            Grabbable::default(),
            DeactivatedGrabbable, // Custom marker for pooled objects
            RigidBody::Disabled,
            Visibility::Hidden,
        ));
    }
}

// Request grabbable from pool
fn get_pooled_grabbable(
    mut commands: Commands,
    pool: Query<(Entity, &DeactivatedGrabbable), With<Grabbable>>,
) -> Option<Entity> {
    if let Some((entity, _)) = pool.iter().next() {
        // Activate and return pooled object
        commands.entity(entity).remove::<DeactivatedGrabbable>();
        commands.entity(entity).insert((
            RigidBody::Dynamic,
            Visibility::Visible,
        ));
        Some(entity)
    } else {
        None // Pool exhausted
    }
}
```

**Distance-Based LOD for Outlines:**
```rust
// Disable outlines for distant objects
fn update_outline_lod(
    mut outlines: Query<(&mut OutlineSettings, &Transform)>,
    camera: Query<&Transform, With<Camera3d>>,
) {
    if let Ok(camera_transform) = camera.get_single() {
        for (mut outline, transform) in outlines.iter_mut() {
            let distance = camera_transform.translation
                .distance(transform.translation);
            
            // Disable outlines beyond 20 units for performance
            outline.active = outline.active && distance < 20.0;
        }
    }
}
```

**Event Batching:**
```rust
// Batch multiple grab events into single physics update
fn batch_grab_events(
    mut event_queue: ResMut<GrabEventQueue>,
    mut batched_queue: Local<Vec<GrabEvent>>
) {
    // Accumulate events over multiple frames
    if !event_queue.0.is_empty() {
        batched_queue.extend(event_queue.0.drain(..));
    }
    
    // Process batch every 3 frames for performance
    if batched_queue.len() > 10 || should_process_batch() {
        for event in batched_queue.drain(..) {
            process_grab_event(event); // Your processing function
        }
    }
}
```

### Design Patterns

**Factory Pattern for Object Creation:**
```rust
// Factory for creating different grabbable types
pub struct GrabbableFactory;

impl GrabbableFactory {
    pub fn create_crate(commands: &mut Commands, position: Vec3) -> Entity {
        commands.spawn((
            Grabbable {
                weight: 2.0,
                use_weight: true,
                ..default()
            },
            GrabMeleeWeapon {
                attacks: vec![crate_attack()],
                can_block: true,
                block_protection: 0.3,
                ..default()
            },
            OutlineSettings {
                color: Color::srgba(0.8, 0.6, 0.2, 1.0),
                ..default()
            },
            PhysicsBundle::crate(position),
        )).id()
    }
    
    pub fn create_telekinesis_orb(
        commands: &mut Commands,
        position: Vec3,
        color: Color
    ) -> Entity {
        commands.spawn((
            Grabbable {
                weight: 0.2,
                use_weight: false,
                extra_grab_distance: 1.5,
                use_events: true,
                ..default()
            },
            ObjectToPlace {
                object_name: "TelekinesisOrb".to_string(),
                ..default()
            },
            OutlineSettings {
                color,
                width: 0.06,
                ..default()
            },
            PuzzlePiece::TelekinesisOrb,
            LightBundle::point_light(position, color),
            RigidBody::Dynamic,
        )).id()
    }
    
    pub fn create_heavy_weapon(commands: &mut Commands, weapon_type: HeavyWeaponType) -> Entity {
        let (weight, damage, name) = match weapon_type {
            HeavyWeaponType::Sledgehammer => (8.0, 40.0, "Sledgehammer"),
            HeavyWeaponType::Battleaxe => (6.0, 35.0, "Battleaxe"),
            HeavyWeaponType::Maul => (10.0, 45.0, "Maul"),
        };
        
        commands.spawn((
            Grabbable {
                weight,
                use_weight: true,
                ..default()
            },
            GrabMeleeWeapon {
                attacks: vec![heavy_attack(name, damage)],
                can_block: true,
                block_protection: 0.5,
                can_throw_return: false,
                ..default()
            },
            HeavyWeapon::new(weapon_type),
            TwoHandedWeapon, // Requires both hands
            PhysicsBundle::heavy_weapon(name),
        )).id()
    }
}
```

**Strategy Pattern for Throw Calculations:**
```rust
trait ThrowStrategy {
    fn calculate_trajectory(
        &self,
        start: Vec3,
        direction: Vec3,
        force: f32,
        mass: f32
    ) -> ThrowResult;
    
    fn get_effective_force(&self, base_force: f32, stat_modifiers: &Stats) -> f32;
}

struct ArcThrow;
impl ThrowStrategy for ArcThrow {
    fn calculate_trajectory(&self, start: Vec3, dir: Vec3, force: f32, mass: f32) -> ThrowResult {
        // Arc physics simulation
        let velocity = dir * force / mass.max(0.1);
        ThrowResult::Arc(velocity, Vec3::Y * -9.81)
    }
    
    fn get_effective_force(&self, base_force: f32, stats: &Stats) -> f32 {
        base_force * stats.throw_power
    }
}

struct StraightThrow;
impl ThrowStrategy for StraightThrow {
    fn calculate_trajectory(&self, start: Vec3, dir: Vec3, force: f32, mass: f32) -> ThrowResult {
        // Straight line with gravity
        let velocity = dir * force / mass.max(0.1);
        ThrowResult::Straight(velocity)
    }
    
    fn get_effective_force(&self, base_force: f32, stats: &Stats) -> f32 {
        base_force * stats.throw_power * 0.8 // Less affected by stats
    }
}

// Usage with grabbers
fn execute_throw(
    grabber: &Grabber,
    held_object: Entity,
    direction: Vec3,
    throw_strategy: &dyn ThrowStrategy,
    stats: &Stats,
) -> ThrowResult {
    let effective_force = throw_strategy.get_effective_force(grabber.throw_force, stats);
    
    // Get object mass
    let mass = 1.0; // Would query from object components
    
    throw_strategy.calculate_trajectory(
        grabber.transform.translation,
        direction,
        effective_force,
        mass
    )
}
```

**Observer Pattern for Grab Events:**
```rust
// Event subscription system for grab/drop/throw
pub trait GrabEventObserver: Send + Sync {
    fn on_grab(&mut self, grabber: Entity, object: Entity);
    fn on_drop(&mut self, grabber: Entity, object: Entity);
    fn on_throw(&mut self, grabber: Entity, object: Entity, velocity: Vec3);
}

pub struct GrabEventManager {
    observers: Vec<Box<dyn GrabEventObserver>>,
}

impl GrabEventManager {
    pub fn add_observer(&mut self, observer: Box<dyn GrabEventObserver>) {
        self.observers.push(observer);
    }
    
    pub fn notify_grab(&mut self, grabber: Entity, object: Entity) {
        for observer in &mut self.observers {
            observer.on_grab(grabber, object);
        }
    }
    
    pub fn notify_drop(&mut self, grabber: Entity, object: Entity) {
        for observer in &mut self.observers {
            observer.on_drop(grabber, object);
        }
    }
}

// Usage in puzzle system
struct PuzzleGrabObserver {
    puzzle_state: ResMut<PuzzleState>,
}

impl GrabEventObserver for PuzzleGrabObserver {
    fn on_grab(&mut self, _grabber: Entity, object: Entity) {
        // Remove from placement tracking when grabbed
        self.puzzle_state.placed_objects.remove(&object);
    }
    
    fn on_drop(&mut self, _grabber: Entity, object: Entity) {
        // Check if dropped in puzzle area
        // Resume placement checking
    }
}

// Usage in quest system  
struct QuestGrabObserver {
    quest_log: ResMut<QuestLog>,
}

impl GrabEventObserver for QuestGrabObserver {
    fn on_grab(&mut self, _grabber: Entity, object: Entity) {
        // Update quests that require picking up items
        for quest in self.quest_log.active_quests.iter_mut() {
            if quest.objectives.contains(&QuestObjective::CollectItem(object)) {
                quest.progress_collect_item(object);
            }
        }
    }
}
```

---

## Troubleshooting

### Common Issues

**Objects Falling Through Floor When Dropped:**
```rust
// Issue: Physics teleportation causes tunneling
// Solution: Add small upward velocity and use safer teleport
fn safe_drop_object(
    mut objects: Query<(Entity, &mut Transform, &mut LinearVelocity)>,
    drop_height: f32,
) {
    for (entity, mut transform, mut velocity) in objects.iter_mut() {
        // Ensure object is above ground
        if transform.translation.y < drop_height {
            transform.translation.y = drop_height;
        }
        
        // Add small upward impulse to prevent ground clipping
        velocity.0.y = 0.5; // Gentle upward nudge
        
        // Use physics system's move_to instead of direct transform manipulation
        commands.entity(entity).insert(PhysicsTeleport {
            target_position: transform.translation,
            safe_mode: true,
        });
    }
}
```

**Grab Jitter/Flickering:**
```rust
// Issue: Collision with grabber or environment causes shaking
// Solution: Temporarily disable collision and use interpolation
fn prevent_grab_jitter(
    mut commands: Commands,
    grab_events: EventReader<GrabEvent>,
    mut collisions: Query<&mut CollisionGroups>,
) {
    for event in grab_events.iter() {
        if let GrabEvent::Grab(grabber, object) = event {
            // Remove collision between grabber and held object
            if let Ok(mut object_collisions) = collisions.get_mut(*object) {
                object_collisions.filters &= !ColliderGroup::Grabber;
            }
            
            // Add smooth interpolation component
            commands.entity(*object).insert(SmoothInterpolation {
                target_position: calculate_hold_position(grabber, object),
                lerp_factor: 0.1,
                disable_physics: true, // Disable rigidbody while grabbed
            });
        }
    }
}
```

**Distance-Based Drop Not Working:**
```rust
// Issue: Objects not dropping when exceeding max_hold_distance
// Solution: Calculate true distance accounting for object bounds
fn check_distance_drops(
    grabbers: Query<(&Grabber, &Transform, &Collider)>,
    mut held_objects: Query<(&Transform, &Collider), With<Grabbable>>,
    mut drop_events: EventWriter<DropEvent>,
) {
    for (grabber, grabber_transform, grabber_collider) in grabbers.iter() {
        if let Some(held) = grabber.held_object {
            if let Ok((held_transform, held_collider)) = held_objects.get(held) {
                // Calculate true distance between collider surfaces
                let center_distance = grabber_transform.translation
                    .distance(held_transform.translation);
                
                // Approximate collider sizes
                let grabber_radius = grabber_collider.radius_approx();
                let held_radius = held_collider.radius_approx();
                
                let surface_distance = center_distance - grabber_radius - held_radius;
                
                // Include extra_grab_distance from grabbable component
                let effective_max_distance = grabber.max_hold_distance + 
                    held_object.extra_grab_distance;
                
                if surface_distance > effective_max_distance {
                    drop_events.send(DropEvent {
                        grabber: grabber.entity,
                        object: held,
                        reason: DropReason::DistanceExceeded,
                    });
                }
            }
        }
    }
}
```

**Power Grabbing Performance Issues:**
```rust
// Issue: Many power-grabbed objects cause frame drops
// Solution: Distance-based culling and simplified physics
fn optimize_power_grab(
    mut powerers: Query<&mut GrabPowerer>,
    objects: Query<&Transform>,
    camera: Query<&Transform, With<Camera3d>>,
) {
    if let Ok(camera_transform) = camera.get_single() {
        for mut powerer in powerers.iter_mut() {
            // Limit held objects based on distance and count
            let mut objects_to_remove = Vec::new();
            
            for (index, held) in powerer.held_objects.iter().enumerate() {
                if let Ok(held_transform) = objects.get(*held) {
                    let distance_to_camera = held_transform.translation
                        .distance(camera_transform.translation);
                    
                    // Remove far objects
                    if distance_to_camera > 50.0 {
                        objects_to_remove.push(index);
                    }
                }
            }
            
            // Remove far objects (in reverse order to maintain indices)
            for index in objects_to_remove.iter().rev() {
                powerer.held_objects.remove(*index);
            }
            
            // Limit maximum objects for performance
            if powerer.held_objects.len() > 20 {
                powerer.held_objects.truncate(20);
            }
        }
    }
}
```

**PlaceSystem Not Placing Objects:**
```rust
// Issue: Objects not snapping to placement positions
// Solution: Verify transform hierarchy and add tolerance
fn verify_placementsystem_setup(
    commands: &mut Commands,
    slot_entity: Entity,
    placement_position: Vec3,
) {
    // Create explicit transform hierarchy
    let placement_transform = commands.spawn(Transform::from_translation(placement_position)).id();
    
    commands.entity(slot_entity).insert(PutObjectSystem {
        place_to_put: Some(placement_transform),
        position_speed: 8.0,
        rotation_speed: 10.0,
        max_distance_to_place: 2.0,
        is_object_placed: false,
        ..default()
    });
    
    // Add tolerance to placement distance check
    commands.entity(slot_entity).insert(PlacementTolerance {
        position_tolerance: 0.2, // Allow slight misalignment
        rotation_tolerance: 0.1,
    });
}

// Modified placement distance check with tolerance
fn check_placement_distance(
    slot_pos: Vec3,
    object_pos: Vec3,
    max_distance: f32,
    tolerance: Option<PlacementTolerance>,
) -> bool {
    let distance = slot_pos.distance(object_pos);
    
    if let Some(tol) = tolerance {
        distance <= max_distance + tol.position_tolerance
    } else {
        distance <= max_distance
    }
}
```

**Melee Weapon Not Dealing Damage:**
```rust
// Issue: GrabMeleeWeapon attacks not registering hits
// Solution: Ensure proper collision channels and attack state management
fn debug_melee_weapon_attacks(
    grabbers: Query<(&Grabber, &GrabMeleeWeapon, &AttackState)>,
    held_objects: Query<(Entity, &Collider)>,
) {
    for (grabber, weapon, attack_state) in grabbers.iter() {
        if let Some(held) = grabber.held_object {
            if attack_state.is_attacking {
                // Ensure collider is active
                if let Ok((object_entity, collider)) = held_objects.get(held) {
                    // Verify collision layer
                    if !collider.collision_enabled() {
                        warn!("Held weapon collider is disabled!");
                    }
                    
                    // Check attack range
                    if weapon.attacks[0].range > 0.0 {
                        debug!("Weapon attack with range: {}", weapon.attacks[0].range);
                    }
                }
            }
        }
    }
}

// Proper attack execution with state management
fn execute_grab_weapon_attack(
    mut grabbers: Query<(&Grabber, &mut AttackState)>,
    weapons: Query<&GrabMeleeWeapon>,
    input: Res<InputState>,
    mut attack_events: EventWriter<WeaponAttackEvent>,
) {
    for (grabber, mut attack_state) in grabbers.iter_mut() {
        if let Some(held) = grabber.held_object {
            if input.attack_pressed && !attack_state.is_attacking {
                if let Ok(weapon) = weapons.get(held) {
                    // Start attack
                    attack_state.is_attacking = true;
                    attack_state.elapsed = 0.0;
                    attack_state.current_attack = Some(weapon.attacks[0].clone());
                    
                    // Send attack event for combat system
                    attack_events.send(WeaponAttackEvent {
                        weapon: held,
                        attacker: grabber.entity,
                        attack: weapon.attacks[0].clone(),
                    });
                }
            }
            
            // Progress attack
            if attack_state.is_attacking {
                attack_state.elapsed += time.delta_seconds();
                
                // End attack when duration complete
                if let Some(attack) = &attack_state.current_attack {
                    if attack_state.elapsed >= attack.duration {
                        attack_state.is_attacking = false;
                        attack_state.current_attack = None;
                    }
                }
            }
        }
    }
}
```

---

## Advanced Implementation Patterns

### Custom Grab Event Processing

Extend the grab system with custom event handling for specialized gameplay mechanics.

```rust
// Custom grab event processor for specific game mechanics
pub struct CustomGrabProcessor {
    pub event_handlers: HashMap<String, Box<dyn Fn(&GrabEvent, &mut Commands)>>,
}

impl CustomGrabProcessor {
    pub fn new() -> Self {
        let mut processor = Self {
            event_handlers: HashMap::new(),
        };
        
        // Register handlers
        processor.event_handlers.insert("QuestItemGrab".to_string(), Box::new(handle_quest_item_grab));
        processor.event_handlers.insert("PuzzlePieceGrab".to_string(), Box::new(handle_puzzle_piece_grab));
        processor.event_handlers.insert("EnemyWeaponGrab".to_string(), Box::new(handle_enemy_weapon_grab));
        
        processor
    }
    
    pub fn process_event(&self, event: &GrabEvent, commands: &mut Commands) {
        match event {
            GrabEvent::Grab(grabber, object) => {
                // Check for custom handlers based on object type
                if let Some(handler) = self.get_handler_for_object(*object) {
                    handler(event, commands);
                }
            }
            GrabEvent::Drop(grabber, object) => {
                // Handle custom drop logic
            }
            GrabEvent::Throw(grabber, object, direction, force) => {
                // Handle custom throw logic
            }
        }
    }
    
    fn get_handler_for_object(&self, object: Entity) -> Option<&Box<dyn Fn(&GrabEvent, &mut Commands)>> {
        // Query object components to determine handler
        // Simplified: would use queries to check object types
        None
    }
}

fn handle_quest_item_grab(event: &GrabEvent, commands: &mut Commands) {
    if let GrabEvent::Grab(grabber, object) = event {
        commands.trigger(QuestUpdateEvent::ItemPickup {
            item: *object,
            picker: *grabber,
        });
    }
}

fn handle_puzzle_piece_grab(event: &GrabEvent, commands: &mut Commands) {
    if let GrabEvent::Grab(_, object) = event {
        commands.trigger(PuzzleStateEvent::PieceRemoved {
            piece: *object,
        });
    }
}

fn handle_enemy_weapon_grab(event: &GrabEvent, commands: &mut Commands) {
    if let GrabEvent::Grab(grabber, object) = event {
        // Disarm the enemy
        commands.trigger(CombatEvent::Disarm {
            target: find_weapon_owner(*object),
            disarmer: *grabber,
            weapon: *object,
        });
    }
}
```

### Dynamic Grab Mode Switching

Switch between grab modes based on game context (combat, puzzle, exploration).

```rust
// Grab mode manager for adaptive gameplay
pub struct GrabModeManager {
    pub modes: HashMap<GameplayContext, GrabMode>,
    pub current_context: GameplayContext,
}

impl GrabModeManager {
    pub fn new() -> Self {
        let mut modes = HashMap::new();
        modes.insert(GameplayContext::Exploration, GrabMode::Realistic);
        modes.insert(GameplayContext::Combat, GrabMode::Realistic);
        modes.insert(GameplayContext::Puzzle, GrabMode::Powers);
        modes.insert(GameplayContext::Stealth, GrabMode::Powers);
        
        Self {
            modes,
            current_context: GameplayContext::Exploration,
        }
    }
    
    pub fn switch_context(&mut self, context: GameplayContext) {
        self.current_context = context;
    }
    
    pub fn get_current_mode(&self) -> GrabMode {
        self.modes.get(&self.current_context)
            .copied()
            .unwrap_or(GrabMode::Powers)
    }
    
    pub fn update_grabber_mode(&self, grabbers: &mut Query<&mut Grabber>) {
        let target_mode = self.get_current_mode();
        
        for mut grabber in grabbers.iter_mut() {
            // Only switch if empty-handed to avoid jarring transitions
            if grabber.held_object.is_none() && grabber.mode != target_mode {
                grabber.mode = target_mode;
                
                // Adjust other settings based on mode
                match target_mode {
                    GrabMode::Powers => {
                        grabber.hold_speed = 15.0;
                        grabber.hold_distance = 2.0;
                    }
                    GrabMode::Realistic => {
                        grabber.hold_speed = 8.0;
                        grabber.hold_distance = 2.5;
                    }
                }
            }
        }
    }
}

// Context detection system
fn detect_gameplay_context(
    player: Query<(&Transform, &Health, &StealthLevel)>,
    nearby_enemies: Query<&Enemy, With<NearPlayer>>,
    nearby_puzzles: Query<&PuzzleElement, With<NearPlayer>>,
    mut mode_manager: ResMut<GrabModeManager>,
) {
    if let Ok((transform, health, stealth)) = player.get_single() {
        // Determine context based on environment
        let has_nearby_enemies = !nearby_enemies.is_empty();
        let has_nearby_puzzles = !nearby_puzzles.is_empty();
        let is_low_health = health.current < health.max * 0.3;
        let is_hidden = stealth.level > 0.7;
        
        let new_context = match (has_nearby_enemies, has_nearby_puzzles, is_low_health, is_hidden) {
            (_, true, _, false) => GameplayContext::Puzzle,
            (true, _, true, _) => GameplayContext::Combat,
            (true, _, false, true) => GameplayContext::Stealth,
            (_, _, _, _) => GameplayContext::Exploration,
        };
        
        if new_context != mode_manager.current_context {
            mode_manager.switch_context(new_context);
            mode_manager.update_grabber_mode(&mut grabbers);
        }
    }
}
```

### Asynchronous Grab Animations

Coordinate grab animations with physics simulation and input response.

```rust
// Animation state management for grabs
pub struct GrabAnimationState {
    pub current_animation: Option<GrabAnimation>,
    pub blend_weight: f32,
    pub target_weight: f32,
    pub animation_duration: f32,
    pub elapsed: f32,
}

pub enum GrabAnimation {
    ReachOut, // Initial grab attempt
    GrabSuccess, // Successful grab
    HoldLoop, // Holding object
    ThrowWindup, // Throw charge-up
    ThrowRelease, // Throw execution
    DropObject, // Drop animation
}

// Animation system
fn update_grab_animations(
    mut grabbers: Query<(&Grabber, &mut GrabAnimationState, &mut AnimationPlayer)>,
    mut grab_events: EventReader<GrabEvent>,
    time: Res<Time>,
) {
    for (grabber, mut anim_state, mut player) in grabbers.iter_mut() {
        // Trigger animations based on events
        for event in grab_events.iter() {
            match event {
                GrabEvent::Grab(_, _) => {
                    anim_state.current_animation = Some(GrabAnimation::GrabSuccess);
                    anim_state.animation_duration = 0.3;
                    anim_state.elapsed = 0.0;
                }
                GrabEvent::Throw(_, _, _, _) => {
                    anim_state.current_animation = Some(GrabAnimation::ThrowRelease);
                    anim_state.animation_duration = 0.4;
                    anim_state.elapsed = 0.0;
                }
                _ => {}
            }
        }
        
        // Update animation playback
        if let Some(animation) = &anim_state.current_animation {
            anim_state.elapsed += time.delta_seconds();
            
            // Calculate blend weight
            let progress = (anim_state.elapsed / anim_state.animation_duration).min(1.0);
            let blend_curve = 1.0 - (1.0 - progress).powi(2); // Ease out
            anim_state.blend_weight = blend_curve;
            
            // Set animation targets
            match animation {
                GrabAnimation::HoldLoop => {
                    // Looping animation while holding
                    player.play("grab_hold".to_string()).repeat();
                }
                _ => {
                    // One-shot animation
                    if anim_state.elapsed >= anim_state.animation_duration {
                        anim_state.current_animation = None;
                    }
                }
            }
        }
    }
}

// Blend animations with physics
fn blend_grab_physics_with_animation(
    mut objects: Query<(&mut Transform, &Grabbable, &HeldObjectState)>,
    mut animation_targets: Query<&AnimationTargetTransform>,
    time: Res<Time>,
) {
    for (mut transform, grabbable, held_state) in objects.iter_mut() {
        if let Ok(target) = animation_targets.get(held_state.animation_target) {
            // Blend between physics-derived position and animation target
            let blend_factor = held_state.animation_weight * time.delta_seconds() * 10.0;
            
            transform.translation = transform.translation.lerp(
                target.translation,
                blend_factor
            );
            
            transform.rotation = transform.rotation.slerp(
                target.rotation,
                blend_factor
            );
        }
    }
}
```

---

## Future Enhancements

### Proposed Extensions

**Dual Wielding Support:**
```rust
// Extended Grabber for two-handed weapon wielding
pub struct DualGrabber {
    pub primary_hand: Grabber,
    pub secondary_hand: Grabber,
    pub coordination_mode: HandCoordinationMode,
}

pub enum HandCoordinationMode {
    Independent, // Hands work separately
    Coordinated, // Hands move together (two-handed weapon)
    Assisted,    // Off-hand assists main hand
}

// Usage example
commands.spawn((
    DualGrabber {
        primary_hand: Grabber::default(),
        secondary_hand: Grabber::default(),
        coordination_mode: HandCoordinationMode::Independent,
    },
    TwoHandedWeaponProficiency(0.8), // 80% effectiveness with two-handed
    Ambidextrous, // Trait allowing balanced dual wielding
));
```

**Advanced Throwing Techniques:**
```rust
pub enum ThrowTechnique {
    Overhand,    // Standard arc throw
    Underhand,   // Gentle toss
    HookThrow,   // Curved trajectory
    SpinThrow,   // Affects object rotation
    DropThrow,   // Drop from above
    SlideThrow,  // Slide along ground
}

pub struct AdvancedThrower {
    pub techniques: HashMap<ThrowTechnique, f32>, // Technique -> mastery level
    pub current_technique: ThrowTechnique,
    pub technique_adaptation: TechniqueAdaptation,
}

// Context-sensitive technique selection
fn auto_select_throwing_technique(
    target: Vec3,
    obstacles: &[Obstacle],
    available_techniques: &HashMap<ThrowTechnique, f32>,
) -> ThrowTechnique {
    // Determine best technique based on environment
    for technique in &available_techniques {
        if Self::evaluate_throwing_path(technique, target, obstacles) {
            return technique;
        }
    }
    ThrowTechnique::Overhand // Fallback to basic
}
```

**Environmental Grabbing:**
```rust
// Dynamic environment interaction
pub struct EnvironmentalGrab {
    pub supports_terrain_grab: bool, // Grab terrain/structures
    pub destructible_grabbing: bool, // Break objects when grabbed
    pub movable_object_linking: bool, // Connect objects together
    pub gravity_well_capable: bool, // Create localized gravity
}

// Terrain deformation on grab
fn handle_terrain_grab(
    grab_point: Vec3,
    grab_force: f32,
    terrain: &mut Terrain,
) -> TerrainModification {
    if grab_force > TERRAIN_DEFORM_THRESHOLD {
        terrain.deform(grab_point, grab_force * 0.1);
        TerrainModification::Permanent
    } else {
        terrain.temporarily_displace(grab_point, grab_force * 0.01);
        TerrainModification::Temporary
    }
}
```

**Magnetic/Force Field Grabbing:**
```rust
pub struct MagneticGrabber {
    pub magnetic_field_strength: f32,
    pub field_radius: f32,
    pub field_shape: MagneticFieldShape,
    pub polarity: Polarity,
    pub attraction_force: f32,
    pub repulsion_force: f32,
}

pub enum MagneticFieldShape {
    Sphere,      // Omnidirectional
    Cone,        // Directional
    Hemisphere,  // Half-sphere
    CustomMesh,  // Arbitrary shape
}

// Attract metallic objects automatically
fn update_magnetic_fields(
    magnetic_grabbers: Query<(&MagneticGrabber, &Transform)>,
    metallic_objects: Query<(Entity, &Transform, &Metallic), With<Grabbable>>,
    mut forces: Query<&mut ExternalForce>,
) {
    for (magnet, magnet_transform) in magnetic_grabbers.iter() {
        for (object, object_transform, metallic) in metallic_objects.iter() {
            if metallic.is_magnetic {
                let distance = magnet_transform.translation
                    .distance(object_transform.translation);
                
                if distance <= magnet.field_radius {
                    let direction = (magnet_transform.translation - object_transform.translation)
                        .normalize();
                    
                    let force_strength = magnet.magnetic_field_strength * 
                        metallic.magnetic_susceptibility / distance.powi(2);
                    
                    if let Ok(mut external_force) = forces.get_mut(object) {
                        external_force.apply_force(direction * force_strength);
                    }
                }
            }
        }
    }
}
```

---

## Performance Characteristics

### Computational Complexity

**Per-Frame Operations:**
- Grab input handling: O(1) per grabber
- Event processing: O(n) where n = grab events per frame (typically < 10)
- Physics updates: O(m) where m = held objects (1-5 typical, up to 20 in power grab)
- Outline updates: O(p) where p = grabbable objects in range (affected by LOD)
- Placement checks: O(s) where s = active slots (typically 1-3)

**Memory Footprint:**
- Base components: ~200 bytes per grabbable object
- Grabber state: ~150 bytes per grabber
- Event queue: Dynamic, ~100 bytes per pending event
- Power grabbing: Additional ~50 bytes per held object

### Optimization Strategies

**Caching:**
- Cache distance calculations between grabbers and potential grab targets
- Cache physics query results for visibility determination
- Pre-compute throw trajectories (can be frame-skipped)

**Culling:**
- Distance-based culling for outlines beyond camera far plane
- Disable physics updates for held objects far from camera  
- Skip placement checks for completed/rewarded puzzle slots

**Threading:**
- Event processing can be parallelized (independent grabber updates)
- Outline rendering can be moved to render thread
- Physics prediction for throws can use job system

---

## Real-World Usage Examples

### Survival Horror: Improvised Defense
In a survival horror game, the grab system enables players to use any object as an improvised weapon or shield when cornered by enemies.

```rust
// Panic grab system for survival horror
fn handle_panic_grab(
    player: Query<(&Transform, &StressLevel), With<Player>>,
    nearby_objects: Query<(Entity, &Transform, &PanicWeaponRating), With<Grabbable>>,
    mut grabber: Query<&mut Grabber>,
    mut panic_state: Query<&mut PanicState>,
) {
    for (player_transform, stress) in player.iter() {
        if stress.level > 0.8 { // High stress = panic mode
            // Auto-grab nearest object without precise aiming
            let mut nearest_object = None;
            let mut min_distance = f32::MAX;
            
            for (object, object_transform, rating) in nearby_objects.iter() {
                let distance = player_transform.translation
                    .distance(object_transform.translation);
                
                // Prefer weapons, then heavy objects
                let effective_distance = distance / rating.effectiveness;
                
                if effective_distance < min_distance {
                    min_distance = effective_distance;
                    nearest_object = Some(object);
                }
            }
            
            if let Some(object) = nearest_object {
                // Auto-grab with increased range
                grabber.max_hold_distance += 2.0; // Panic strength
                events.send(GrabEvent::Grab(player_entity, object));
            }
        }
    }
}
```

**Implementation Details:**
- Panic grab increases grab range by 100%
- Automatically selects most effective nearby weapon
- Blocks with held object when attacked
- Stress decreases when successfully defending with grabbed object

---

### Puzzle Platformer: Telekinesis Orbs
In a puzzle platformer, players use power grabbing to manipulate colored orbs that activate mechanisms and create platforms.

```rust
// Telekinesis-based puzzle system
pub struct TelekinesisOrb {
    pub color: OrbColor,
    pub activation_radius: f32,
    pub power_requirement: f32,
    pub target_slots: Vec<Entity>,
    pub current_energy: f32,
}

fn orb_power_management(
    mut orbs: Query<(&mut TelekinesisOrb, &Grabbable)>,
    powerers: Query<&GrabPowerer, With<Player>>,
    mut energy_conduits: EventWriter<EnergyTransfer>,
) {
    for (mut orb, grabbable) in orbs.iter_mut() {
        // Drain powerer energy while orb is held
        if let Ok(powerer) = powerers.get_single() {
            if powerer.held_objects.contains(&orb.entity) {
                let energy_drain = orb.power_requirement * TIME_STEP;
                energy_conduits.send(EnergyTransfer {
                    from: powerer.entity,
                    to: orb.entity,
                    amount: energy_drain,
                });
                
                orb.current_energy += energy_drain * 0.7; // Some energy loss
            }
        }
        
        // Glow intensity based on stored energy
        let glow_intensity = (orb.current_energy / 100.0).min(1.0);
        commands.entity(orb.entity).insert(Emissive {
            color: orb.color.as_rgb() * glow_intensity,
        });
    }
}

fn orb_slot_activation(
    mut slots: Query<(&mut PutObjectSystem, &OrbSlot)>,
    orbs: Query<(&TelekinesisOrb, &Transform)>,
    mut activation_events: EventWriter<SlotActivation>,
) {
    for (mut slot, slot_config) in slots.iter_mut() {
        if let Some(placed_orb) = slot.current_object_placed {
            if let Ok((orb, orb_transform)) = orbs.get(placed_orb) {
                // Check energy threshold
                if orb.current_energy >= orb.power_requirement {
                    if orb.color == slot_config.required_color {
                        // Activate slot effect (platform, door, etc.)
                        activation_events.send(SlotActivation {
                            slot: slot.entity,
                            orb: placed_orb,
                            effect: slot_config.effect,
                            active: true,
                        });
                    }
                }
            }
        }
    }
}
```

**Game Design Integration:**
- Orbs require continuous energy to maintain activation
- Different colors match different slot types
- Energy management becomes a resource puzzle
- Distance to slot affects energy transfer efficiency

---

### Action RPG: Tactical Throwing
In an action RPG, players use throwing mechanics to create environmental combos and tactical advantages.

```rust
// Tactical throwing with element combinations
pub struct ElementalObject {
    pub element: Element,
    pub elemental_potency: f32,
    pub combo_triggers: Vec<ComboTrigger>,
}

pub struct ComboTrigger {
    pub trigger_element: Element,
    pub effect: ComboEffect,
    pub delay: f32,
}

fn handle_elemental_combos(
    mut collision_events: EventReader<CollisionEvent>,
    elemental_objects: Query<(Entity, &ElementalObject, &Transform)>,
    mut terrain: Query<&mut TerrainElement>,
) {
    for collision in collision_events.iter() {
        // Check if both objects have elements
        if let (Ok((obj1, elem1, _)), Ok((obj2, elem2, _))) = (
            elemental_objects.get(collision.entity1),
            elemental_objects.get(collision.entity2)
        ) {
            // Check for combo triggers
            for trigger in &elem1.combo_triggers {
                if trigger.trigger_element == elem2.element {
                    // Apply combo effect
                    match trigger.effect {
                        ComboEffect::Explosion => {
                            create_explosion(collision.point, elem1.elemental_potency);
                        },
                        ComboEffect::Freeze => {
                            create_ice_patch(collision.point);
                        },
                        ComboEffect::Electrify => {
                            create_electric_field(collision.point);
                        },
                        _ => {}
                    }
                }
            }
        }
    }
}

// Tactical classification system
fn classify_throw_target(
    target_position: Vec3,
    enemies: Query<(&Enemy, &Transform)>,
    obstacles: Query<&Obstacle>,
) -> ThrowTactics {
    let mut tactics = ThrowTactics::default();
    
    // Count enemies in area
    let mut enemy_count = 0;
    for (enemy, enemy_transform) in enemies.iter() {
        if enemy_transform.translation.distance(target_position) < 5.0 {
            enemy_count += 1;
            
            // Check enemy state
            if enemy.is_armored {
                tactics.armor_targets.push(enemy.entity);
            }
            if enemy.is_flying {
                tactics.aerial_targets.push(enemy.entity);
            }
        }
    }
    
    tactics.total_enemies = enemy_count;
    tactics.recommended_object = determine_best_object_for_tactics(&tactics);
    tactics.throw_force = calculate_optimal_throw_force(&tactics);
    
    tactics
}

fn determine_best_object_for_tactics(tactics: &ThrowTactics) -> RecommendedObject {
    match (tactics.total_enemies, tactics.armor_targets.len()) {
        (1, 0) => RecommendedObject::SmallObject, // Single target, light weapon
        (1, 1..) => RecommendedObject::HeavyObject, // Single armored target, heavy weapon
        (2..=4, _) => RecommendedObject::ExplosiveObject, // Group, area damage
        (5.., 0) => RecommendedObject::LargeObject, // Crowd, crowd control
        (5.., 1..) => RecommendedObject::PiercingObject, // Crowd with armor, penetration
        _ => RecommendedObject::StandardObject, // Default
    }
}
```

**Strategic Depth:**
- Different enemy types require different thrown objects
- Environmental combos reward tactical thinking
- Throw force affects trajectory and impact type
- Smart object highlighting guides optimal choices

---

## Related Systems

### Integration Dependencies

The Grab System relies on and integrates with:

- **Physics System** (Avian3D) - Core physics simulation and collision detection
- **Input System** - Action mapping and input state management
- **Interaction System** - Detection of interactable objectives and NPCs
- **Combat System** - Damage application and blocking mechanics
- **Puzzle System** - Object placement validation and state management
- **Stealth System** - Sound emission and detection radius control
- **Ability System** - Power grab activation and energy management
- **Dialog System** - Contextual tutorials and grab-related dialogue
- **Stats System** - Throw power, grab strength, and stamina modifications
- **Inventory System** - Object ownership and container management
- **AI System** - Enemy reactions to grabbed objects and thrown weapons
- **Player System** - High-level player state and control mode selection
- **Camera System** - Rotate-around-held-object camera modes
- **UI System** - Grab prompts, throw arc visualization, and slot indicators

### Cross-System Communication

**Event Flow:**
```
Input → GrabInputSystem → GrabEvent → [CombatSystem, PuzzleSystem, QuestSystem, etc.]
Physics → GrabPhysicsSystem → PositionUpdate → [CameraSystem, AISystem]  
StatChange → StatsSystem → ModifiedGrabParams → GrabberComponentUpdate
```

**Component Dependencies:**
- Requires `InputState` for grab controls
- Requires `PhysicsWorld` for collision queries
- Requires `Transform` for position calculations
- Requires `Health/Stamina` for grab attempt costing

This comprehensive documentation covers all aspects of the Grab System, from core components to advanced integrations. The system provides a foundation for physics-based object manipulation that can be extended and customized for any gameplay style or genre requirement.