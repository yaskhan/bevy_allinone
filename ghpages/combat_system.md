# Combat System - Comprehensive Documentation

## Overview

The Combat System is a sophisticated, modular framework designed to handle all aspects of combat interactions in Bevy-based games. Built with flexibility and extensibility in mind, it supports a wide range of combat scenarios from simple melee encounters to complex tactical engagements. The system integrates seamlessly with other game systems such as AI, inventory, abilities, and UI to provide a cohesive combat experience.

**Key Features:**
- Advanced damage calculation pipeline with multipliers and resistances
- Comprehensive health and shield management
- Melee and ranged combat mechanics
- Blocking, parrying, and defensive maneuvers
- Damage over time (DoT) effects
- Area of effect (AoE) attacks
- Locational damage with weak spots
- Combat feedback with floating damage numbers
- Integration with physics for environmental interactions
- Support for destructible objects and explosions
- Event-driven architecture for easy extension

**Module Location:** `src/combat.rs`

---

## Table of Contents

- [Overview](#overview)
- [Core Concepts](#core-concepts)
  - [Combat Architecture](#combat-architecture)
  - [Damage Processing Pipeline](#damage-processing-pipeline)
  - [Combat State Management](#combat-state-management)
- [Component Reference](#component-reference)
  - [Health](#health)
  - [Shield](#shield)
  - [DamageReceiver](#damagereceiver)
  - [MeleeCombat](#meleecombat)
  - [Blocking](#blocking)
  - [Parrying](#parrying)
  - [DamageOverTime](#damageovertime)
  - [AreaEffect](#areaeffect)
  - [Destroyable](#destroyable)
  - [ExplosionSettings](#explosionsettings)
  - [CombatStats](#combatstats)
  - [CombatFeedback](#combatfeedback)
- [System Integration](#system-integration)
  - [AI System Integration](#ai-system-integration)
  - [Inventory System Integration](#inventory-system-integration)
  - [Abilities System Integration](#abilities-system-integration)
  - [Stats System Integration](#stats-system-integration)
  - [UI System Integration](#ui-system-integration)
  - [Physics System Integration](#physics-system-integration)
- [Combat Mechanics](#combat-mechanics)
  - [Damage System](#damage-system)
  - [Melee Combat](#melee-combat)
  - [Ranged Combat](#ranged-combat)
  - [Blocking and Parrying](#blocking-and-parrying)
  - [Combat Combos](#combat-combos)
  - [Critical Hits and Weak Spots](#critical-hits-and-weak-spots)
  - [Damage Over Time Effects](#damage-over-time-effects)
  - [Area of Effect Attacks](#area-of-effect-attacks)
  - [Destructible Objects](#destructible-objects)
  - [Explosions and Environmental Damage](#explosions-and-environmental-damage)
- [Advanced Features](#advanced-features)
  - [Combat State Machine](#combat-state-machine)
  - [Hit Detection Methods](#hit-detection-methods)
  - [Damage Modifiers and Resistances](#damage-modifiers-and-resistances)
  - [Combat Events and Callbacks](#combat-events-and-callbacks)
  - [Combat Animation Integration](#combat-animation-integration)
  - [Combat Sound Effects](#combat-sound-effects)
  - [Combat Camera Effects](#combat-camera-effects)
  - [Combat UI Feedback](#combat-ui-feedback)
- [Best Practices](#best-practices)
  - [Performance Optimization](#performance-optimization)
  - [Combat Balancing](#combat-balancing)
  - [State Management](#state-management)
  - [Error Handling](#error-handling)
  - [Testing Strategies](#testing-strategies)
- [Troubleshooting](#troubleshooting)
  - [Common Issues](#common-issues)
  - [Debugging Techniques](#debugging-techniques)
  - [Performance Bottlenecks](#performance-bottlenecks)
- [Advanced Implementation Patterns](#advanced-implementation-patterns)
  - [Custom Damage Types](#custom-damage-types)
  - [Dynamic Combat Modifiers](#dynamic-combat-modifiers)
  - [Combat AI Integration](#combat-ai-integration)
  - [Multiplayer Combat Synchronization](#multiplayer-combat-synchronization)
  - [Procedural Combat Generation](#procedural-combat-generation)
- [Future Enhancements](#future-enhancements)
  - [Proposed Extensions](#proposed-extensions)
  - [Roadmap](#roadmap)
- [Performance Characteristics](#performance-characteristics)
  - [Computational Complexity](#computational-complexity)
  - [Memory Usage](#memory-usage)
  - [Optimization Strategies](#optimization-strategies)
- [Real-World Usage Examples](#real-world-usage-examples)
  - [Action RPG: Dynamic Combat System](#action-rpg-dynamic-combat-system)
  - [Tactical Shooter: Precision Combat](#tactical-shooter-precision-combat)
  - [Survival Horror: Tension-Based Combat](#survival-horror-tension-based-combat)
  - [Fighting Game: Combo-Driven Combat](#fighting-game-combo-driven-combat)
- [Related Systems](#related-systems)
  - [Integration Dependencies](#integration-dependencies)
  - [Cross-System Communication](#cross-system-communication)
- [Appendix](#appendix)
  - [Glossary](#glossary)
  - [Configuration Reference](#configuration-reference)
  - [Migration Guide](#migration-guide)

---

## Core Concepts

### Combat Architecture

The Combat System is built on a modular, event-driven architecture that separates concerns and allows for easy extension. The system consists of several key layers:

1. **Core Layer**: Contains fundamental combat components and systems
2. **Mechanics Layer**: Implements specific combat mechanics (melee, ranged, blocking)
3. **Effects Layer**: Handles combat effects (DoT, AoE, explosions)
4. **Integration Layer**: Provides interfaces to other game systems
5. **Feedback Layer**: Manages combat feedback (UI, sound, animations)

### Damage Processing Pipeline

The damage processing pipeline is the heart of the combat system. It consists of the following stages:

1. **Event Generation**: Combat actions generate damage events
2. **Target Resolution**: Determines the target entity and damage receiver
3. **Damage Calculation**: Applies multipliers, resistances, and modifiers
4. **Defense Processing**: Checks for blocking, parrying, and other defensive actions
5. **Shield Application**: Deducts damage from shields if available
6. **Health Application**: Applies remaining damage to health
7. **Feedback Generation**: Creates visual and audio feedback
8. **State Update**: Updates combat states and triggers follow-up actions

### Combat State Management

The combat system maintains state for all combat entities through a combination of components and resources:

- **Entity-Level State**: Stored in components attached to combat entities
- **Global State**: Maintained in resources for system-wide tracking
- **Temporary State**: Used for short-lived combat interactions

---

## Component Reference

### Health

The `Health` component manages the life points and regeneration properties of combat entities.

**Key Fields:**
- `current: f32` - Current health points
- `maximum: f32` - Maximum health points
- `can_regenerate: bool` - Whether health can regenerate
- `regeneration_rate: f32` - Health points regenerated per second
- `regeneration_delay: f32` - Delay after damage before regeneration starts
- `is_invulnerable: bool` - Whether the entity is currently invulnerable
- `temporal_invincibility_duration: f32` - Duration of invincibility frames after damage
- `last_damage_time: f32` - Timestamp of last damage received
- `last_damage_source: Option<Entity>` - Entity that last damaged this entity

**Behavior:**
- Health automatically regenerates when `can_regenerate` is true
- Regeneration starts after `regeneration_delay` seconds of no damage
- Invincibility frames prevent damage for `temporal_invincibility_duration` seconds
- When health reaches 0, a `DeathEvent` is triggered

### Shield

The `Shield` component provides a secondary health pool that absorbs damage before health.

**Key Fields:**
- `current: f32` - Current shield points
- `maximum: f32` - Maximum shield points
- `can_regenerate: bool` - Whether shield can regenerate
- `regeneration_rate: f32` - Shield points regenerated per second
- `regeneration_delay: f32` - Delay after damage before regeneration starts
- `is_active: bool` - Whether the shield is currently active
- `last_damage_time: f32` - Timestamp of last damage received
- `shield_type: ShieldType` - Type of shield (Energy, Physical, Magical)

**Behavior:**
- Shield absorbs damage before health
- Shield regeneration follows the same pattern as health
- Shield can be temporarily disabled (e.g., by EMP effects)
- Different shield types may have different damage absorption properties

### DamageReceiver

The `DamageReceiver` component is used for hitboxes or body parts that forward damage to a main `Health` component.

**Key Fields:**
- `damage_multiplier: f32` - Scale incoming damage (e.g., 2.0 for headshots)
- `is_weak_spot: bool` - Flags hits as "Critical" for UI feedback
- `health_root: Entity` - The entity with the `Health` component to damage
- `hitbox_type: HitboxType` - Type of hitbox (Head, Torso, Limb, etc.)
- `armor_value: f32` - Additional armor value for this hitbox
- `hit_reaction: HitReaction` - Type of reaction to trigger on hit

**Behavior:**
- Allows for locational damage with different multipliers
- Can trigger specific hit reactions based on hit location
- Forwards damage to the root health component
- Provides feedback for weak spot hits

### MeleeCombat

The `MeleeCombat` component handles close-quarters attack capabilities.

**Key Fields:**
- `damage: f32` - Base damage per attack
- `range: f32` - Maximum attack range
- `attack_speed: f32` - Attacks per second
- `combo_enabled: bool` - Whether combos are enabled
- `combo_window: f32` - Time window to chain attacks
- `combo_damage_multiplier: f32` - Damage multiplier for combo attacks
- `current_combo_count: u32` - Current combo counter
- `attack_animation: String` - Animation to play during attack
- `hit_animation: String` - Animation to play on successful hit
- `attack_sound: String` - Sound to play during attack
- `hit_sound: String` - Sound to play on successful hit

**Behavior:**
- Handles melee attack detection and damage application
- Supports combo systems with increasing damage
- Manages attack animations and sounds
- Can be extended with special attack types

### Blocking

The `Blocking` component provides defensive capabilities to reduce incoming damage.

**Key Fields:**
- `block_reduction: f32` - Damage reduction factor (0.0-1.0)
- `is_blocking: bool` - Whether currently blocking
- `block_animation: String` - Animation to play when blocking
- `block_sound: String` - Sound to play when blocking
- `block_stamina_cost: f32` - Stamina cost per second of blocking
- `can_parry: bool` - Whether parrying is enabled
- `parry_window: f32` - Time window for successful parry
- `parry_multiplier: f32` - Damage multiplier when parrying

**Behavior:**
- Reduces incoming damage by `block_reduction` factor
- Can be toggled on/off based on player input
- Supports parrying for perfect timing blocks
- Integrates with stamina system for resource management

### Parrying

The `Parrying` component extends blocking with perfect timing mechanics.

**Key Fields:**
- `parry_window: f32` - Time window for successful parry
- `parry_cooldown: f32` - Cooldown between parry attempts
- `parry_animation: String` - Animation to play on successful parry
- `parry_sound: String` - Sound to play on successful parry
- `parry_counter_attack: bool` - Whether to enable counter attacks
- `counter_attack_damage: f32` - Damage for counter attacks
- `last_parry_time: f32` - Timestamp of last parry attempt

**Behavior:**
- Negates all damage if blocked within parry window
- Triggers special feedback and animations
- Can enable counter attacks after successful parry
- Has cooldown to prevent spamming

### DamageOverTime

The `DamageOverTime` component applies damage at regular intervals.

**Key Fields:**
- `damage_per_tick: f32` - Damage applied per interval
- `tick_frequency: f32` - Time between damage applications
- `total_duration: f32` - Total duration of the effect
- `remaining_duration: f32` - Remaining time for the effect
- `source_entity: Option<Entity>` - Entity that applied the DoT
- `damage_type: DamageType` - Type of damage (Poison, Fire, Bleed, etc.)
- `is_stackable: bool` - Whether multiple instances can stack
- `max_stack_count: u32` - Maximum number of stacks

**Behavior:**
- Applies damage at regular intervals
- Can be used for poison, fire, bleed effects
- Supports stacking for increased damage
- Can be removed or modified during application

### AreaEffect

The `AreaEffect` component creates zones that apply effects to entities within range.

**Key Fields:**
- `radius: f32` - Size of the effect zone
- `interval: f32` - Time between effect applications
- `damage_type: DamageType` - Type of effect (Damage, Heal, Buff, Debuff)
- `effect_amount: f32` - Amount of effect per application
- `duration: f32` - Total duration of the area effect
- `is_pulsing: bool` - Whether effect pulses or is continuous
- `target_filter: TargetFilter` - Filter for affected entities
- `visual_effect: String` - Visual effect to display

**Behavior:**
- Creates circular zones that affect entities
- Can be used for environmental hazards or healing zones
- Supports different effect types and filters
- Can pulse or apply effects continuously

### Destroyable

The `Destroyable` component marks entities that can be destroyed.

**Key Fields:**
- `health_threshold: f32` - Health threshold for destruction
- `debris_prefab: Option<String>` - Prefab to spawn on destruction
- `explosion_settings: Option<ExplosionSettings>` - Explosion configuration
- `destruction_sound: String` - Sound to play on destruction
- `destruction_effect: String` - Visual effect on destruction
- `is_destructible: bool` - Whether currently destructible
- `destruction_delay: f32` - Delay before destruction after health threshold reached

**Behavior:**
- Triggers destruction when health falls below threshold
- Can spawn debris and trigger explosions
- Plays destruction sounds and effects
- Can be temporarily made indestructible

### ExplosionSettings

The `ExplosionSettings` component configures explosion effects.

**Key Fields:**
- `radius: f32` - Explosion radius
- `damage: f32` - Base damage at center
- `falloff: f32` - Damage falloff with distance
- `force: f32` - Physical force applied
- `damage_type: DamageType` - Type of damage
- `explosion_prefab: String` - Visual explosion prefab
- `explosion_sound: String` - Explosion sound effect
- `affects_player: bool` - Whether explosion affects player
- `affects_enemies: bool` - Whether explosion affects enemies
- `affects_objects: bool` - Whether explosion affects objects

**Behavior:**
- Applies damage to all entities in radius
- Applies physical force to push objects
- Creates visual and audio effects
- Can be configured to affect specific entity types

### CombatStats

The `CombatStats` component tracks combat-related statistics.

**Key Fields:**
- `total_damage_dealt: f32` - Total damage dealt
- `total_damage_received: f32` - Total damage received
- `total_kills: u32` - Total kills
- `total_deaths: u32` - Total deaths
- `critical_hits: u32` - Number of critical hits
- `blocks_successful: u32` - Successful blocks
- `parries_successful: u32` - Successful parries
- `combo_max: u32` - Maximum combo achieved
- `longest_spree: u32` - Longest kill spree

**Behavior:**
- Tracks comprehensive combat statistics
- Can be used for achievements and progression
- Provides data for combat analysis
- Can be reset or saved as needed

### CombatFeedback

The `CombatFeedback` component manages visual and audio feedback.

**Key Fields:**
- `damage_numbers_enabled: bool` - Whether to show damage numbers
- `damage_number_style: DamageNumberStyle` - Style configuration
- `hit_effects_enabled: bool` - Whether to show hit effects
- `block_effects_enabled: bool` - Whether to show block effects
- `critical_hit_effect: String` - Effect for critical hits
- `weak_spot_effect: String` - Effect for weak spot hits
- `damage_sound: String` - Sound for damage
- `block_sound: String` - Sound for blocking
- `parry_sound: String` - Sound for parrying

**Behavior:**
- Manages all combat feedback effects
- Configures damage number display
- Handles hit, block, and parry effects
- Provides audio feedback for combat actions

---

## System Integration

### AI System Integration

The Combat System integrates with the AI System to provide intelligent combat behavior:

- **Target Selection**: AI entities use combat data to select targets
- **Combat Tactics**: AI can adapt tactics based on combat state
- **Defensive Behavior**: AI can block, parry, and dodge based on combat events
- **Offensive Behavior**: AI can chain attacks and use combos
- **Tactical Retreat**: AI can retreat when health is low

**Integration Points:**
- `CombatAIPlugin` - Main integration plugin
- `AICombatBehavior` - Component for AI combat configuration
- `AICombatState` - Tracks current AI combat state
- `AICombatEvents` - Events for AI combat actions

### Inventory System Integration

The Combat System works with the Inventory System to support weapon-based combat:

- **Weapon Damage**: Uses weapon stats for damage calculation
- **Weapon Types**: Supports different weapon types (swords, axes, bows)
- **Weapon Effects**: Applies weapon-specific effects
- **Ammunition**: Tracks ammunition for ranged weapons
- **Weapon Durability**: Handles weapon wear and breakage

**Integration Points:**
- `WeaponCombatPlugin` - Weapon combat integration
- `WeaponDamageCalculator` - Calculates weapon-based damage
- `AmmunitionTracker` - Tracks ammunition usage
- `WeaponDurability` - Handles weapon durability

### Abilities System Integration

The Combat System integrates with the Abilities System to support special combat abilities:

- **Ability Damage**: Uses ability stats for damage calculation
- **Ability Effects**: Applies ability-specific effects
- **Cooldowns**: Manages ability cooldowns
- **Resource Costs**: Handles ability resource costs (mana, stamina)
- **Combo Abilities**: Supports abilities that chain with combos

**Integration Points:**
- `CombatAbilitiesPlugin` - Combat abilities integration
- `AbilityDamageCalculator` - Calculates ability-based damage
- `AbilityCombatEffects` - Handles ability combat effects
- `CombatAbilityCooldowns` - Manages ability cooldowns

### Stats System Integration

The Combat System works with the Stats System to provide character progression:

- **Stat-Based Damage**: Uses character stats for damage calculation
- **Stat Modifiers**: Applies stat-based damage modifiers
- **Level Scaling**: Scales combat based on character level
- **Attribute Effects**: Applies attribute-based combat effects
- **Skill Bonuses**: Provides combat bonuses based on skills

**Integration Points:**
- `CombatStatsPlugin` - Combat stats integration
- `StatDamageCalculator` - Calculates stat-based damage
- `CombatStatModifiers` - Handles stat-based modifiers
- `CombatLevelScaling` - Manages level-based scaling

### UI System Integration

The Combat System integrates with the UI System to provide combat feedback:

- **Health Bars**: Displays health and shield status
- **Damage Numbers**: Shows floating damage numbers
- **Combat Notifications**: Displays combat-related notifications
- **Ability Cooldowns**: Shows ability cooldown timers
- **Combat Stats**: Displays combat statistics

**Integration Points:**
- `CombatUIPlugin` - Combat UI integration
- `HealthBarSystem` - Manages health bar display
- `DamageNumberSystem` - Handles damage number display
- `CombatNotificationSystem` - Manages combat notifications

### Physics System Integration

The Combat System works with the Physics System for realistic combat interactions:

- **Hit Detection**: Uses physics for accurate hit detection
- **Force Application**: Applies physical forces from attacks
- **Ragdoll Physics**: Handles death physics
- **Environmental Interactions**: Supports physics-based environmental damage
- **Projectile Physics**: Manages projectile trajectories

**Integration Points:**
- `CombatPhysicsPlugin` - Combat physics integration
- `PhysicsHitDetection` - Physics-based hit detection
- `CombatForceApplication` - Handles force application
- `CombatRagdollSystem` - Manages ragdoll physics

---

## Combat Mechanics

### Damage System

The damage system is the core of the combat mechanics, handling all aspects of damage calculation and application.

**Damage Calculation:**
```
Final Damage = Base Damage × General Multiplier × Part Multiplier × Resistance Factor × Random Factor
```

**Damage Types:**
- `Physical` - Standard physical damage
- `Slashing` - Slashing weapon damage
- `Piercing` - Piercing weapon damage
- `Blunt` - Blunt weapon damage
- `Fire` - Fire-based damage
- `Ice` - Ice-based damage
- `Lightning` - Lightning-based damage
- `Poison` - Poison-based damage
- `Holy` - Holy/magical damage
- `Dark` - Dark/necrotic damage

**Damage Processing:**
1. Calculate base damage
2. Apply general damage multiplier
3. Apply part-specific multiplier (if hit on damage receiver)
4. Apply resistance based on damage type
5. Apply random damage variation
6. Check for blocking/parrying
7. Apply shield absorption
8. Apply remaining damage to health

### Melee Combat

Melee combat handles close-quarters attacks with various detection methods.

**Detection Methods:**
- **Sphere Casting**: Casts a sphere forward to detect targets
- **Ray Casting**: Uses ray casting for precise hit detection
- **Hitbox Overlap**: Checks for overlap with target hitboxes
- **Sweep Testing**: Performs sweep tests for moving attacks

**Combo System:**
- Successful attacks within combo window increment combo counter
- Combo counter resets if no attack within combo window
- Each combo level can have different damage multipliers
- Combo animations can be different for each level

**Hit Reactions:**
- **Light Hit**: Small reaction for minor damage
- **Heavy Hit**: Large reaction for significant damage
- **Critical Hit**: Special reaction for critical hits
- **Knockdown**: Knocks target to the ground
- **Stagger**: Stuns target briefly

### Ranged Combat

Ranged combat handles projectile-based attacks and ranged weapons.

**Projectile Types:**
- **Instant Hit**: Hits target immediately (hit-scan)
- **Physical Projectile**: Travels through space with physics
- **Energy Projectile**: Energy-based projectiles
- **Area Projectile**: Explodes on impact or after delay

**Projectile Properties:**
- `speed: f32` - Projectile speed
- `range: f32` - Maximum range
- `damage: f32` - Base damage
- `piercing: bool` - Whether projectile pierces targets
- `homing: bool` - Whether projectile homes on targets
- `explosion: Option<ExplosionSettings>` - Explosion on impact

**Ranged Mechanics:**
- **Ammunition**: Tracks ammunition usage
- **Reloading**: Handles weapon reloading
- **Accuracy**: Manages weapon accuracy
- **Recoil**: Handles weapon recoil
- **Projectile Drop**: Simulates gravity effects

### Blocking and Parrying

Defensive mechanics to reduce or negate incoming damage.

**Blocking:**
- Reduces incoming damage by block reduction factor
- Can be directional (only blocks attacks from front)
- Consumes stamina while blocking
- Can be broken by heavy attacks
- Triggers block animations and sounds

**Parrying:**
- Perfect timing block that negates all damage
- Has a small timing window for success
- Can trigger counter attacks
- Has cooldown to prevent spamming
- Triggers special parry animations and sounds

**Defensive States:**
- **Idle**: No defensive action
- **Blocking**: Currently blocking
- **Parry Ready**: Ready to parry
- **Parry Success**: Successfully parried
- **Parry Failure**: Failed parry attempt
- **Block Broken**: Block broken by heavy attack

### Combat Combos

The combo system allows for chaining attacks for increased damage and special effects.

**Combo Properties:**
- `combo_window: f32` - Time window to chain attacks
- `combo_damage_multiplier: f32` - Damage multiplier per combo level
- `max_combo_level: u32` - Maximum combo level
- `combo_reset_time: f32` - Time before combo resets
- `combo_special_attacks: Vec<ComboSpecialAttack>` - Special attacks at combo levels

**Combo Execution:**
1. Player performs initial attack
2. If next attack within combo window, increment combo level
3. Apply combo damage multiplier
4. Check for special attacks at current combo level
5. Execute special attack if available
6. Reset combo if no attack within combo window

**Combo Feedback:**
- Visual indicators for combo level
- Audio cues for successful combos
- Animation changes for different combo levels
- Special effects for high combo levels

### Critical Hits and Weak Spots

Critical hits provide bonus damage for precise attacks on weak spots.

**Critical Hit Properties:**
- `critical_chance: f32` - Chance for critical hit (0.0-1.0)
- `critical_multiplier: f32` - Damage multiplier for critical hits
- `critical_effect: String` - Visual effect for critical hits
- `critical_sound: String` - Sound for critical hits

**Weak Spot Properties:**
- `weak_spot_multiplier: f32` - Damage multiplier for weak spots
- `weak_spot_effect: String` - Visual effect for weak spot hits
- `weak_spot_sound: String` - Sound for weak spot hits
- `weak_spot_hitbox: Entity` - Hitbox entity for weak spot

**Critical Hit Calculation:**
1. Check if attack hits weak spot
2. If weak spot hit, apply weak spot multiplier
3. Roll for critical hit based on critical chance
4. If critical hit, apply critical multiplier
5. Combine multipliers for final damage

### Damage Over Time Effects

DoT effects apply damage at regular intervals over time.

**DoT Types:**
- `Poison` - Nature-based damage over time
- `Bleed` - Physical damage over time
- `Burn` - Fire-based damage over time
- `Corrosion` - Acid-based damage over time
- `Radiation` - Radiation-based damage over time
- `HolyBurn` - Holy damage over time
- `DarkCorruption` - Dark damage over time

**DoT Properties:**
- `damage_per_tick: f32` - Damage per application
- `tick_frequency: f32` - Time between applications
- `total_duration: f32` - Total duration
- `is_stackable: bool` - Whether effects can stack
- `max_stack_count: u32` - Maximum stacks
- `stack_damage_multiplier: f32` - Damage multiplier per stack

**DoT Application:**
1. Apply initial DoT effect
2. Start timer for next application
3. Apply damage at each interval
4. Check for stacking with existing effects
5. Remove effect when duration expires

### Area of Effect Attacks

AoE attacks affect multiple targets within a specified area.

**AoE Types:**
- `Circular` - Circular area around point
- `Cone` - Cone-shaped area in direction
- `Line` - Line-shaped area
- `Rectangular` - Rectangular area
- `Custom` - Custom-shaped area

**AoE Properties:**
- `radius: f32` - Size of area
- `shape: AoEShape` - Shape of area
- `damage: f32` - Base damage
- `falloff: f32` - Damage falloff with distance
- `target_filter: TargetFilter` - Filter for affected entities
- `application_interval: f32` - Time between applications

**AoE Application:**
1. Determine area shape and size
2. Find all entities within area
3. Filter entities based on target filter
4. Apply damage to each entity
5. Apply damage falloff based on distance
6. Repeat at specified intervals

### Destructible Objects

Objects that can be destroyed through combat interactions.

**Destructible Properties:**
- `health_threshold: f32` - Health threshold for destruction
- `debris_prefab: Option<String>` - Prefab to spawn on destruction
- `explosion_settings: Option<ExplosionSettings>` - Explosion configuration
- `destruction_sound: String` - Sound to play on destruction
- `destruction_effect: String` - Visual effect on destruction
- `is_destructible: bool` - Whether currently destructible

**Destruction Process:**
1. Apply damage to destructible object
2. Check if health falls below threshold
3. Trigger destruction sequence
4. Spawn debris if configured
5. Trigger explosion if configured
6. Play destruction sounds and effects
7. Remove object from game

### Explosions and Environmental Damage

Explosions provide area damage with physical force effects.

**Explosion Properties:**
- `radius: f32` - Explosion radius
- `damage: f32` - Base damage at center
- `falloff: f32` - Damage falloff with distance
- `force: f32` - Physical force applied
- `damage_type: DamageType` - Type of damage
- `explosion_prefab: String` - Visual explosion prefab
- `explosion_sound: String` - Explosion sound effect

**Explosion Application:**
1. Determine explosion center and radius
2. Find all entities within radius
3. Calculate damage based on distance and falloff
4. Apply physical force to entities
5. Spawn visual explosion effect
6. Play explosion sound
7. Apply environmental damage (e.g., break windows, damage structures)

---

## Advanced Features

### Combat State Machine

The combat state machine manages the current state of combat entities.

**Combat States:**
- `Idle` - Not in combat
- `Engaged` - In combat
- `Attacking` - Currently attacking
- `Blocking` - Currently blocking
- `Dodging` - Currently dodging
- `Stunned` - Stunned and unable to act
- `Dead` - Dead

**State Transitions:**
- Idle → Engaged (when entering combat)
- Engaged → Attacking (when attacking)
- Engaged → Blocking (when blocking)
- Engaged → Dodging (when dodging)
- Any → Stunned (when stunned)
- Any → Dead (when health reaches 0)

**State Management:**
- Each state has specific behaviors
- States can have entry and exit actions
- States can be interrupted by higher priority states
- State transitions can trigger events

### Hit Detection Methods

Various methods for detecting combat hits.

**Sphere Casting:**
- Casts a sphere forward from attacker
- Detects entities within sphere radius
- Good for melee weapons with area effect

**Ray Casting:**
- Casts a ray from attacker in direction
- Detects first entity hit
- Good for precise ranged attacks

**Hitbox Overlap:**
- Checks for overlap between attack hitbox and target hitboxes
- Good for precise melee combat
- Supports complex hitbox shapes

**Sweep Testing:**
- Performs sweep test along attack path
- Detects entities along path
- Good for moving attacks (e.g., sword swings)

### Damage Modifiers and Resistances

Systems for modifying damage based on various factors.

**Damage Modifiers:**
- `General Multiplier` - Global damage multiplier
- `Part Multiplier` - Multiplier for specific body parts
- `Critical Multiplier` - Multiplier for critical hits
- `Combo Multiplier` - Multiplier for combo attacks
- `Elemental Multiplier` - Multiplier for elemental damage

**Resistances:**
- `Physical Resistance` - Resistance to physical damage
- `Elemental Resistance` - Resistance to elemental damage
- `Magic Resistance` - Resistance to magical damage
- `Specific Resistance` - Resistance to specific damage types

**Resistance Calculation:**
```
Effective Damage = Base Damage × (1.0 - Resistance Factor)
```

### Combat Events and Callbacks

Event system for combat interactions.

**Combat Events:**
- `DamageEvent` - Triggered when damage is applied
- `BlockEvent` - Triggered when damage is blocked
- `ParryEvent` - Triggered when damage is parried
- `CriticalHitEvent` - Triggered on critical hits
- `DeathEvent` - Triggered when entity dies
- `ComboEvent` - Triggered on combo execution
- `DoTAppliedEvent` - Triggered when DoT is applied
- `AoEAppliedEvent` - Triggered when AoE is applied

**Event Callbacks:**
- Can register callbacks for specific events
- Callbacks can modify event data
- Callbacks can trigger additional actions
- Supports priority-based callback execution

### Combat Animation Integration

Integration with animation system for combat visuals.

**Animation Types:**
- `Attack Animations` - Different animations for attack types
- `Block Animations` - Animations for blocking
- `Parry Animations` - Animations for parrying
- `Hit Reactions` - Animations for being hit
- `Death Animations` - Animations for dying
- `Special Attacks` - Unique animations for special attacks

**Animation Triggers:**
- Triggered by combat events
- Can be interrupted by higher priority animations
- Can blend between animations
- Support animation callbacks for combat actions

### Combat Sound Effects

Audio feedback for combat actions.

**Sound Types:**
- `Attack Sounds` - Sounds for different attack types
- `Hit Sounds` - Sounds for successful hits
- `Block Sounds` - Sounds for blocking
- `Parry Sounds` - Sounds for parrying
- `Critical Hit Sounds` - Sounds for critical hits
- `Death Sounds` - Sounds for dying
- `Environmental Sounds` - Sounds for environmental interactions

**Sound Management:**
- Volume based on distance
- Pitch variation for realism
- Sound occlusion for indoor/outdoor
- Dynamic sound mixing based on combat intensity

### Combat Camera Effects

Camera effects to enhance combat feel.

**Camera Effects:**
- `Screen Shake` - Shakes screen on heavy hits
- `Slow Motion` - Slows time for dramatic moments
- `Zoom In` - Zooms in on critical hits
- `Field of View` - Adjusts FOV for combat intensity
- `Camera Angle` - Changes angle for special attacks
- `Motion Blur` - Adds blur for fast movements

**Effect Triggers:**
- Triggered by combat events
- Can be configured per entity type
- Support intensity scaling
- Can be disabled for accessibility

### Combat UI Feedback

User interface elements for combat feedback.

**UI Elements:**
- `Health Bars` - Shows health and shield status
- `Damage Numbers` - Floating damage numbers
- `Combo Meter` - Shows current combo level
- `Stamina Bar` - Shows stamina for blocking/dodging
- `Cooldown Timers` - Shows ability cooldowns
- `Hit Indicators` - Shows hit direction and type
- `Kill Feed` - Shows recent kills and combat events

**UI Configuration:**
- Position and size customization
- Color schemes for different damage types
- Animation styles for feedback
- Accessibility options (size, contrast, etc.)

---

## Best Practices

### Performance Optimization

Techniques for optimizing combat system performance.

**Optimization Strategies:**
- **Spatial Partitioning**: Use spatial data structures for efficient hit detection
- **Object Pooling**: Reuse objects instead of creating/destroying
- **Batch Processing**: Process combat events in batches
- **Level of Detail**: Reduce detail for distant combat
- **Culling**: Skip processing for off-screen entities
- **Multithreading**: Use parallel processing for combat calculations

**Performance Tips:**
- Limit number of active combat entities
- Use efficient hit detection methods
- Minimize physics calculations
- Optimize damage calculation pipeline
- Cache frequently accessed data

### Combat Balancing

Techniques for balancing combat mechanics.

**Balancing Principles:**
- **Fairness**: Ensure combat is fair and skill-based
- **Variety**: Provide diverse combat options
- **Progression**: Allow for character growth and improvement
- **Challenge**: Provide appropriate challenge level
- **Feedback**: Give clear feedback on combat actions

**Balancing Techniques:**
- Adjust damage values and multipliers
- Tune combo windows and damage
- Balance defensive mechanics
- Test with different playstyles
- Gather player feedback

### State Management

Best practices for managing combat state.

**State Management Tips:**
- Keep state transitions clear and predictable
- Use finite state machines for complex states
- Minimize state duplication
- Validate state transitions
- Provide state change callbacks

**State Debugging:**
- Log state transitions for debugging
- Visualize state machines
- Provide state inspection tools
- Validate state consistency

### Error Handling

Techniques for handling combat system errors.

**Error Handling Strategies:**
- **Validation**: Validate combat data before processing
- **Fallbacks**: Provide fallback behaviors for errors
- **Logging**: Log errors for debugging
- **Recovery**: Implement recovery mechanisms
- **Testing**: Thoroughly test combat scenarios

**Common Error Types:**
- Invalid entity references
- Missing components
- Invalid state transitions
- Physics calculation errors
- Animation system errors

### Testing Strategies

Approaches for testing combat systems.

**Testing Levels:**
- **Unit Testing**: Test individual combat components
- **Integration Testing**: Test component interactions
- **System Testing**: Test complete combat scenarios
- **Performance Testing**: Test combat performance
- **User Testing**: Test with real players

**Testing Techniques:**
- Automated test scripts
- Manual testing scenarios
- Stress testing with many entities
- Edge case testing
- Regression testing

---

## Troubleshooting

### Common Issues

Frequently encountered combat system problems.

**Hit Detection Issues:**
- Hits not registering properly
- False positive hits
- Hitbox misalignment
- Detection range problems

**Damage Calculation Issues:**
- Incorrect damage values
- Multiplier not applying
- Resistance not working
- Critical hits not triggering

**State Management Issues:**
- Invalid state transitions
- State synchronization problems
- State inconsistency
- State change not triggering events

**Performance Issues:**
- Combat lag with many entities
- Hit detection performance problems
- Animation system slowdowns
- Memory leaks in combat systems

### Debugging Techniques

Methods for debugging combat systems.

**Debugging Tools:**
- Combat event logging
- Hit detection visualization
- State machine visualization
- Damage calculation breakdown
- Performance profiling tools

**Debugging Strategies:**
- Isolate problem components
- Reproduce issue consistently
- Check event sequences
- Validate data flow
- Test with minimal configuration

### Performance Bottlenecks

Common performance issues in combat systems.

**Performance Problems:**
- Too many active combat entities
- Inefficient hit detection
- Excessive physics calculations
- Complex damage calculations
- Unoptimized animations

**Optimization Approaches:**
- Reduce active entity count
- Optimize hit detection algorithms
- Simplify physics calculations
- Cache damage calculations
- Optimize animation systems

---

## Advanced Implementation Patterns

### Custom Damage Types

Creating and using custom damage types.

**Custom Damage Type Creation:**
1. Define new damage type enum variant
2. Implement damage calculation for new type
3. Add resistance handling for new type
4. Create visual effects for new type
5. Add sound effects for new type

**Custom Damage Type Usage:**
- Apply custom damage in combat actions
- Handle custom damage in damage processing
- Provide feedback for custom damage
- Balance custom damage with existing types

### Dynamic Combat Modifiers

Implementing modifiers that change during combat.

**Dynamic Modifier Types:**
- Time-based modifiers (buffs/debuffs)
- Condition-based modifiers (low health, combo level)
- Environmental modifiers (weather, terrain)
- Equipment-based modifiers (weapons, armor)

**Dynamic Modifier Implementation:**
1. Define modifier conditions
2. Implement modifier application
3. Handle modifier stacking
4. Provide visual feedback
5. Manage modifier duration

### Combat AI Integration

Advanced AI integration for combat.

**AI Combat Behaviors:**
- Tactical positioning
- Formation combat
- Flanking maneuvers
- Cover usage
- Retreat and regroup
- Special ability usage

**AI Combat Implementation:**
1. Define AI combat states
2. Implement tactical decision making
3. Handle group coordination
4. Provide combat feedback to AI
5. Balance AI difficulty levels

### Multiplayer Combat Synchronization

Synchronizing combat across networked players.

**Synchronization Challenges:**
- Network latency
- Prediction and reconciliation
- State synchronization
- Hit detection consistency
- Animation synchronization

**Synchronization Techniques:**
- Client-side prediction
- Server reconciliation
- State interpolation
- Event-based synchronization
- Delta compression

### Procedural Combat Generation

Generating combat scenarios procedurally.

**Procedural Combat Elements:**
- Enemy spawn points
- Combat arenas
- Cover positions
- Environmental hazards
- Loot placement

**Procedural Generation Techniques:**
- Rule-based generation
- Noise-based generation
- Grammar-based generation
- Evolutionary algorithms
- Machine learning approaches

---

## Future Enhancements

### Proposed Extensions

Potential additions to the combat system.

**Combat Enhancements:**
- Advanced combo system with branching paths
- Environmental combat interactions
- Dynamic difficulty adjustment
- Combat replay system
- Spectator mode for combat

**System Integrations:**
- Enhanced AI combat tactics
- Advanced physics interactions
- Improved multiplayer synchronization
- Better procedural generation
- Advanced analytics and telemetry

### Roadmap

Planned development path for combat system.

**Short-term Goals:**
- Improve hit detection accuracy
- Enhance combo system
- Add more damage types
- Improve performance
- Expand documentation

**Medium-term Goals:**
- Advanced AI integration
- Multiplayer support
- Procedural combat generation
- Combat analytics
- Enhanced UI feedback

**Long-term Goals:**
- Complete combat system overhaul
- Next-generation combat mechanics
- Advanced physics integration
- Machine learning for combat balancing
- Virtual reality combat support

---

## Performance Characteristics

### Computational Complexity

Analysis of combat system computational requirements.

**Complexity Factors:**
- Number of active combat entities
- Hit detection method complexity
- Damage calculation complexity
- Physics calculation requirements
- Animation system demands

**Complexity Analysis:**
- Hit detection: O(n) to O(n²) depending on method
- Damage calculation: O(1) per entity
- Physics calculations: O(n) to O(n²)
- Animation system: O(n) per entity
- Event processing: O(n) per event type

### Memory Usage

Memory requirements for combat system.

**Memory Components:**
- Entity components: ~100-500 bytes per entity
- System resources: ~1-10 MB depending on configuration
- Event queues: ~1-10 KB per active event type
- Animation data: ~10-100 KB per animation
- Sound data: ~100 KB-1 MB per sound

**Memory Optimization:**
- Use component pooling
- Minimize event data size
- Compress animation data
- Stream sound data
- Use efficient data structures

### Optimization Strategies

Techniques for optimizing combat system performance.

**Optimization Approaches:**
- **Data-Oriented Design**: Organize data for cache efficiency
- **Parallel Processing**: Use multithreading for combat calculations
- **Level of Detail**: Reduce detail for distant combat
- **Culling**: Skip processing for off-screen entities
- **Object Pooling**: Reuse objects to reduce allocation
- **Batch Processing**: Process combat events in batches
- **Spatial Partitioning**: Use efficient spatial data structures
- **Algorithm Selection**: Choose efficient algorithms for operations

---

## Real-World Usage Examples

### Action RPG: Dynamic Combat System

Implementation of combat system in an action RPG.

**Combat Features:**
- Real-time combat with combos
- Multiple weapon types
- Magic and special abilities
- Enemy variety with different combat styles
- Boss battles with unique mechanics

**Implementation Details:**
- Use combo system for chained attacks
- Implement weapon switching mechanics
- Add magic system with elemental damage
- Create diverse enemy AI behaviors
- Design boss battles with multiple phases

### Tactical Shooter: Precision Combat

Combat system for a tactical shooter game.

**Combat Features:**
- Realistic weapon handling
- Cover system integration
- Team-based combat
- Destructible environments
- Tactical equipment

**Implementation Details:**
- Implement realistic ballistics
- Add cover detection and usage
- Create team coordination mechanics
- Design destructible environment system
- Add tactical equipment (grenades, etc.)

### Survival Horror: Tension-Based Combat

Combat system for survival horror games.

**Combat Features:**
- Limited resources
- High stakes combat
- Environmental interactions
- Stealth mechanics
- Psychological effects

**Implementation Details:**
- Implement resource scarcity
- Add tension-building mechanics
- Create environmental combat options
- Integrate stealth system
- Add psychological effects (fear, panic)

### Fighting Game: Combo-Driven Combat

Combat system for fighting games.

**Combat Features:**
- Complex combo system
- Special moves and supers
- Character-specific movesets
- Blocking and counter mechanics
- Stage interactions

**Implementation Details:**
- Design deep combo system
- Implement special move inputs
- Create character-specific abilities
- Add advanced blocking mechanics
- Integrate stage interaction system

---

## Related Systems

### Integration Dependencies

Systems that the combat system depends on.

**Core Dependencies:**
- Bevy ECS framework
- Physics system
- Animation system
- Audio system
- UI system

**Game System Dependencies:**
- AI system
- Inventory system
- Abilities system
- Stats system
- Save system

### Cross-System Communication

How combat system communicates with other systems.

**Communication Methods:**
- Event system for combat events
- Component sharing for entity data
- Resource sharing for global data
- Direct system calls for specific interactions
- Message passing for complex interactions

**Communication Patterns:**
- Publish-subscribe for events
- Shared component access
- Resource-based data sharing
- Direct function calls
- Message queues

---

## Appendix

### Glossary

Key terms and definitions for the combat system.

**Combat Terms:**
- **DoT**: Damage Over Time - Damage applied at regular intervals
- **AoE**: Area of Effect - Effects that cover an area
- **DPS**: Damage Per Second - Measure of damage output
- **Hitbox**: Collision volume for hit detection
- **I-frames**: Invincibility frames after being hit
- **Combo**: Sequence of chained attacks
- **Parry**: Perfect timing block that negates damage
- **Stagger**: Brief stun from heavy hits
- **Knockdown**: Being knocked to the ground
- **Ragdoll**: Physics-based death animation

### Configuration Reference

Configuration options for the combat system.

**Configuration Parameters:**
- Damage calculation parameters
- Hit detection settings
- Combat feedback options
- Performance tuning parameters
- Debugging and logging settings

**Configuration Files:**
- `combat_config.ron` - Main configuration file
- `damage_types.ron` - Damage type definitions
- `hit_detection.ron` - Hit detection configuration
- `feedback.ron` - Feedback configuration
- `performance.ron` - Performance tuning

### Migration Guide

Guide for migrating from older combat system versions.

**Migration Steps:**
1. Backup existing combat data
2. Update to new combat system version
3. Convert old combat components
4. Update combat event handlers
5. Test migrated combat scenarios
6. Optimize migrated combat systems

**Migration Tools:**
- Component conversion scripts
- Event mapping tools
- Configuration migrators
- Testing frameworks
- Performance analysis tools

---

## Conclusion

The Combat System provides a comprehensive framework for implementing sophisticated combat mechanics in Bevy-based games. With its modular architecture, extensive feature set, and flexible integration capabilities, it can support a wide range of combat scenarios from simple melee encounters to complex tactical engagements.

This documentation covers all major aspects of the combat system, including core components, advanced features, integration points, best practices, and troubleshooting guidance. By following the patterns and recommendations outlined in this documentation, developers can create engaging and balanced combat experiences that integrate seamlessly with other game systems.

For the most up-to-date information and additional resources, please refer to the official repository and community forums. The combat system is continuously evolving, with new features and improvements being added regularly based on community feedback and development needs.