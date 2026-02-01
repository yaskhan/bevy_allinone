# Player System Documentation
 
## Table of Contents
 
1. [Overview](#overview)
2. [Architecture & Core Concepts](#architecture--core-concepts)
3. [System Components](#system-components)
   - [Player Modes System](#player-modes-system)
   - [Player State System](#player-state-system)
   - [Player Idle System](#player-idle-system)
   - [NavMesh Override System](#navmesh-override-system)
   - [Ragdoll System](#ragdoll-system)
   - [Sprite Animator System](#sprite-animator-system)
   - [Upper Body Rotation System](#upper-body-rotation-system)
   - [Extra Movement Systems](#extra-movement-systems)
4. [Advanced Features](#advanced-features)
5. [System Integration](#system-integration)
6. [Usage Patterns](#usage-patterns)
7. [Best Practices](#best-practices)
8. [Common Patterns](#common-patterns)
9. [Troubleshooting](#troubleshooting)
10. [Performance Considerations](#performance-considerations)
11. [Future Enhancements](#future-enhancements)
 
---
 
## Overview
 
The Player System is a comprehensive collection of plugins and components that manage all aspects of player character behavior, movement, and state management within the Bevy All-in-One Controller framework. This system provides a modular architecture that allows developers to implement complex player mechanics ranging from basic movement to advanced features like ragdolling, flying, swimming, and AI-controlled sequences.
 
The Player System consists of multiple interconnected subsystems that work together to provide a complete player character framework. Each subsystem is designed to be independently configurable while maintaining seamless integration with other systems throughout the game framework.
 
### Key Features
 
- **Multi-Modal Movement Support**: Comprehensive support for various movement modes including walking, running, swimming, flying, wall-running, and special movement mechanics
- **State-Driven Architecture**: Sophisticated state management system with priority handling, interruptible states, and duration-based transitions
- **Input-Responsive Systems**: Intelligent idle management, animation state control, and input state integration
- **Physics Integration**: Ragdoll physics, collision handling, and ground detection capabilities
- **Animation Support**: Both 3D skeletal animation and 2D sprite-based animation systems
- **AI Integration**: NavMesh override capabilities for cutscenes and scripted sequences
- **Modular Design**: Each subsystem can be used independently or combined for complex player behaviors
 
### Supported Movement Types
 
The Player System supports an extensive range of movement mechanics:
 
- **Ground-Based Movement**: Walking, running, sprinting, crouching, and directional movement
- **Vertical Movement**: Jumping, climbing ladders, wall-running, and ledge grabbing
- **Aerial Movement**: Flying, gliding with paragliders, and free-fall mechanics
- **Water-Based Movement**: Swimming, diving, and underwater navigation
- **Special Movement**: Sphere mode (rolling), jetpack flight, and roll-on-landing
- **Physics-Based Movement**: Ragdoll physics for realistic falls and death sequences
 
---
 
## Architecture & Core Concepts
 
The Player System follows a component-entity-system (CES) architecture that leverages Bevy's built-in systems and custom plugins. The architecture is designed around several key principles:
 
### Component-Based Design
 
Each player behavior is implemented as a separate component that can be attached to player entities. This allows for:
 
- **Selective Feature Activation**: Enable only the features needed for your game
- **Runtime Configuration**: Modify player behavior dynamically during gameplay
- **Easy Testing**: Test individual components in isolation
- **Flexible Composition**: Combine different systems to create unique player experiences
 
### Event-Driven Communication
 
The Player System uses Bevy's event system for inter-component communication:
 
- **State Change Events**: Events fired when player states change
- **Mode Switch Events**: Events for switching between different player modes
- **Action Events**: Events triggered by player actions like jumping, attacking, or interacting
- **Animation Events**: Events for coordinating animations with gameplay states
 
### Resource-Based Queues
 
For high-frequency operations and batch processing, the Player System uses resource queues:
 
- **Event Queues**: Process multiple events efficiently in batch operations
- **State Queues**: Queue state changes to be processed in the next update cycle
- **Mode Queues**: Manage player mode transitions
 
### Priority-Based State Management
 
The state management system implements a sophisticated priority system:
 
- **Priority Levels**: States can be assigned different priority levels
- **Interrupt Handling**: Higher priority states can interrupt lower priority ones
- **Duration Management**: States can be time-limited or permanent
- **Concurrent States**: Multiple states can be active simultaneously when configured
 
 
---
 
## System Components
 
### Player Modes System
 
The Player Modes System manages high-level player modes and control states. This system is fundamental to organizing different player behaviors and capabilities.
 
#### Core Concepts
 
**Player Modes** represent different capability sets or gameplay modes that the player can be in:
 
- **Weapons Mode**: Player has access to weapon systems and combat abilities
- **Powers Mode**: Player has access to special abilities or powers
- **Exploration Mode**: Player has enhanced movement and interaction capabilities
- **Stealth Mode**: Player has stealth-related abilities and reduced visibility
 
**Control States** represent different input handling modes:
 
- **Regular Mode**: Standard FPS/TPS controller input
- **Driving Mode**: Vehicle-specific input handling
- **Flying Mode**: Aerial movement input schemes
- **Swimming Mode**: Underwater movement input patterns
 
#### Component Structure
 
##### PlayerMode Component
 
```rust
pub struct PlayerMode {
    pub name: String,              // Unique identifier for the mode
    pub mode_enabled: bool,         // Whether this mode is currently enabled
    pub is_current_state: bool,    // Whether this mode is the active mode
}
```
 
**Field Descriptions:**
- `name`: Unique string identifier for the mode. Used for referencing and switching
- `mode_enabled`: Boolean flag to enable/disable the mode without removing it
- `is_current_state`: Tracks whether this mode is currently active
 
##### PlayerControlState Component
 
```rust
pub struct PlayerControlState {
    pub name: String,                                        // State identifier
    pub mode_enabled: bool,                                  // Whether the state is enabled
    pub is_current_state: bool,                              // Whether this is the active state
    pub avoid_to_set_regular_mode_when_active: bool,        // Prevents automatic regular mode switching
}
```
 
**Field Descriptions:**
- `name`: Unique identifier for the control state
- `mode_enabled`: Enable/disable flag for the control state
- `is_current_state`: Indicates if this control state is currently active
- `avoid_to_set_regular_mode_when_active`: Prevents automatic switching to regular mode when this state is active
 
##### PlayerModesSystem Component
 
```rust
pub struct PlayerModesSystem {
    pub use_default_players_mode: bool,         // Whether to use default mode
    pub default_players_mode_name: String,      // Name of the default player mode
    pub current_players_mode_name: String,      // Currently active player mode
    pub player_modes: Vec,          // List of available player modes
    
    pub default_control_state_name: String,      // Default control state
    pub current_control_state_name: String,     // Currently active control state
    pub player_control_states: Vec, // List of available control states
    
    pub change_mode_enabled: bool,              // Whether mode changes are enabled
    pub change_player_control_enabled: bool,   // Whether control state changes are enabled
}
```
 
**Field Descriptions:**
- `use_default_players_mode`: When true, automatically sets the default mode on initialization
- `default_players_mode_name`: The name of the mode to use as default
- `current_players_mode_name`: Currently active player mode name
- `player_modes`: Vector containing all available player modes
- `default_control_state_name`: Name of the default control state
- `current_control_state_name`: Currently active control state name
- `player_control_states`: Vector containing all available control states
- `change_mode_enabled`: Global flag to enable/disable player mode switching
- `change_player_control_enabled`: Global flag to enable/disable control state switching
 
#### Event System
 
##### SetPlayerModeEvent
 
Fires when a player mode should be changed:
 
```rust
pub struct SetPlayerModeEvent {
    pub player_entity: Entity,  // Entity of the player to change mode for
    pub mode_name: String,      // Name of the mode to switch to
}
```
 
##### SetControlStateEvent
 
Fires when a control state should be changed:
 
```rust
pub struct SetControlStateEvent {
    pub player_entity: Entity,   // Entity of the player to change state for
    pub state_name: String,     // Name of the control state to switch to
}
```
 
##### PlayerModeChangedEvent
 
Fired when a mode or control state has been successfully changed:
 
```rust
pub struct PlayerModeChangedEvent {
    pub player_entity: Entity,   // Player entity
    pub new_mode_name: String,  // Name of the new mode/state
    pub is_control_state: bool, // True if this was a control state change, false if player mode
}
```
 
#### Mode Management Logic
 
##### Mode Change Processing
 
The system processes mode changes through the following logic:
 
1. **Validation**: Checks if the requested mode exists and is enabled
2. **Deactivation**: Deactivates any currently active modes of the same type
3. **Activation**: Activates the requested mode
4. **Event Firing**: Fires appropriate events to notify other systems
 
##### Control State Management
 
Control states follow similar logic to player modes but include additional considerations:
 
1. **Exclusivity**: Only one control state can be active at a time
2. **Avoid Regular Mode Logic**: Respects the `avoid_to_set_regular_mode_when_active` flag
3. **Priority Handling**: Some control states can prevent regular mode activation
 
#### Integration Points
 
The Player Modes System integrates with:
 
- **Input System**: Different control states affect input handling
- **Animation System**: Mode changes can trigger animation state changes
- **Camera System**: Control states can affect camera behavior
- **UI System**: Mode changes can update UI elements and indicators
 
---
 
### Player State System
 
The Player State System provides sophisticated state management with priority handling, duration management, and interrupt capabilities. This system is essential for managing temporary player conditions like being stunned, under effects, or in specific action states.
 
#### Core Concepts
 
**Player States** represent temporary conditions or modes that the player can be in:
 
- **Combat States**: Attacking, blocking, parrying, dodge rolling
- **Effect States**: Stunned, poisoned, healing, buffed, debuffed
- **Action States**: Jumping, falling, climbing, interacting
- **Ability States**: Using special abilities, channeling, cooldowns
 
#### State Priority System
 
The priority system allows for sophisticated state management:
 
- **Higher Priority States**: Cannot be interrupted by lower priority states
- **Lower Priority States**: Can be overridden by higher priority states
- **Equal Priority States**: Cannot interrupt each other unless configured otherwise
- **Priority Inheritance**: Some states can inherit priority from their source
 
#### Component Structure
 
##### PlayerStateInfo Component
 
```rust
pub struct PlayerStateInfo {
    pub name: String,              // Unique identifier for the state
    pub state_enabled: bool,        // Whether this state is enabled
    pub state_active: bool,        // Whether this state is currently active
    pub state_priority: i32,       // Priority level of the state
    pub can_be_interrupted: bool,  // Whether this state can be interrupted
    pub use_state_duration: bool,  // Whether this state has a time limit
    pub state_duration: f32,       // Duration of the state in seconds
    pub current_duration_timer: f32, // Current timer value
}
```
 
**Field Descriptions:**
- `name`: Unique string identifier for the state
- `state_enabled`: Enable/disable flag for the state
- `state_active`: Indicates if the state is currently active
- `state_priority`: Integer priority level (higher numbers = higher priority)
- `can_be_interrupted`: Whether this state can be interrupted by other states
- `use_state_duration`: Whether this state should automatically expire after duration
- `state_duration`: How long the state lasts when time-limited
- `current_duration_timer`: Internal timer tracking elapsed time
 
##### PlayerStateSystem Component
 
```rust
pub struct PlayerStateSystem {
    pub player_states_enabled: bool,  // Whether the state system is enabled
    pub player_state_list: Vec, // List of all available states
    pub current_state_name: String,   // Name of the currently active state
}
```
 
**Field Descriptions:**
- `player_states_enabled`: Global enable/disable flag for the state system
- `player_state_list`: Vector containing all player states
- `current_state_name`: Name of the currently active state
 
#### Event System
 
##### SetPlayerStateEvent
 
Used to request a state change:
 
```rust
pub struct SetPlayerStateEvent {
    pub player_entity: Entity,  // Entity of the player
    pub state_name: String,      // Name of the state to activate
}
```
 
##### PlayerStateChangedEvent
 
Fired when a state has been activated or deactivated:
 
```rust
pub struct PlayerStateChangedEvent {
    pub player_entity: Entity,  // Player entity
    pub state_name: String,    // Name of the changed state
    pub active: bool,           // True if state was activated, false if deactivated
}
```
 
#### State Management Logic
 
##### State Activation Process
 
1. **State Lookup**: Finds the requested state in the player state list
2. **Enabled Check**: Verifies the state is enabled
3. **Priority Validation**: Checks if the state can interrupt current active states
4. **Deactivation**: Deactivates conflicting states
5. **Activation**: Activates the new state
6. **Event Firing**: Notifies other systems of the state change
 
##### Priority Handling
 
The priority system follows these rules:
 
1. **Active State Check**: Identifies currently active states
2. **Interrupt Permission**: Checks if active states can be interrupted
3. **Priority Comparison**: Compares priorities to determine if interruption is allowed
4. **Decision Making**: Allows or denies the state change based on priority rules
 
##### Duration Management
 
For time-limited states:
 
1. **Timer Initialization**: Sets the timer to zero when state activates
2. **Time Accumulation**: Accumulates time using Bevy's delta time
3. **Expiration Check**: Checks if the timer exceeds the state duration
4. **Automatic Deactivation**: Deactivates the state when duration expires
5. **Event Firing**: Notifies systems of the automatic expiration
 
#### State Types and Examples
 
##### High Priority States (Priority 100+)
- **Death**: Player is dead and cannot perform actions
- **Stunned**: Player cannot move or act for a duration
- **Unconscious**: Player is knocked out and vulnerable
- **Cutscene**: Player is under AI control during scripted sequence
 
##### Medium Priority States (Priority 50-99)
- **Combat**: Player is in active combat mode
- **Blocking**: Player is actively blocking attacks
- **Dodge Rolling**: Player is performing evasive maneuver
- **Special Ability**: Player is using a special ability
 
##### Low Priority States (Priority 1-49)
- **Walking**: Basic walking movement
- **Running**: Fast movement state
- **Crouching**: Low profile movement
- **Swimming**: Water-based movement
 
#### Integration with Other Systems
 
##### Animation System Integration
- State changes can trigger animation transitions
- Animation completion can deactivate states
- State priority affects animation blending
 
##### Input System Integration
- Active states can modify input handling
- Input can trigger state changes
- State priority can override input processing
 
##### Physics System Integration
- States can modify physics properties
- Physics events can trigger state changes
- State transitions can affect movement capabilities
 
 
---
 
### Player Idle System
 
The Player Idle System manages idle behaviors and animations when the player is not actively using input. This system enhances the player experience by providing realistic idle animations and behaviors that make the character feel more alive and responsive.
 
#### Core Concepts
 
**Idle States** represent different idle behaviors that can be cycled through:
 
- **Basic Idle**: Standing animation when not moving
- **Look Around**: Character looks around the environment
- **Stretch**: Character performs stretching animations
- **Check Equipment**: Character examines or adjusts equipment
- **Environmental Reactions**: Character reacts to nearby objects or events
 
#### Idle Trigger Conditions
 
The system monitors various conditions to determine when to activate idle states:
 
- **Input Inactivity**: No movement or look input for a specified duration
- **Movement Speed**: Velocity below idle threshold
- **Environmental Context**: Proximity to interesting objects or features
- **Time-Based Activation**: Random idle triggers based on time intervals
 
#### Component Structure
 
##### IdleInfo Component
 
```rust
pub struct IdleInfo {
    pub name: String,   // Identifier for the idle state
    pub duration: f32,  // How long this idle animation/state lasts
}
```
 
**Field Descriptions:**
- `name`: Unique identifier for the idle state
- `duration`: Duration in seconds that this idle state should play
 
##### PlayerIdleSystem Component
 
```rust
pub struct PlayerIdleSystem {
    pub idle_enabled: bool,                    // Whether idle system is enabled
    pub idle_active: bool,                      // Whether idle system is currently active
    pub current_idle_index: usize,              // Index of currently playing idle state
    pub play_random_idle: bool,                 // Whether to play random idle states
    pub idle_info_list: Vec,          // List of available idle states
    pub timer: f32,                             // Timer for idle state progression
    pub idle_stopped_automatically: bool,       // Whether idle was stopped by input
}
```
 
**Field Descriptions:**
- `idle_enabled`: Global enable/disable flag for the idle system
- `idle_active`: Indicates if idle states are currently playing
- `current_idle_index`: Index in the idle list of the currently active idle state
- `play_random_idle`: When true, selects idle states randomly instead of sequentially
- `idle_info_list`: Vector containing all available idle states
- `timer`: Internal timer tracking elapsed time for current idle state
- `idle_stopped_automatically`: Flag to track whether input stopped idle automatically
 
#### Idle State Management
 
##### Idle Activation Logic
 
The system follows this process for idle activation:
 
1. **Input Monitoring**: Continuously monitors player input state
2. **Inactivity Detection**: Detects when player has been inactive for threshold duration
3. **Activation Decision**: Determines whether to activate idle based on conditions
4. **State Selection**: Chooses appropriate idle state from available options
5. **Timer Initialization**: Starts timer for the selected idle state
 
##### Idle State Progression
 
1. **Timer Accumulation**: Accumulates time while idle state is active
2. **Duration Check**: Compares elapsed time with state duration
3. **Next State Selection**: Chooses next idle state based on configuration
4. **Random vs Sequential**: Uses random selection or sequential progression
5. **State Transition**: Transitions to new idle state
6. **Repeat Process**: Continues cycling through idle states
 
##### Idle Deactivation
 
Idle states are deactivated when:
 
- **Input Detection**: Player provides movement or look input
- **External Request**: Other systems request idle deactivation
- **Game State Change**: Game state changes require player attention
- **Manual Disable**: Idle system is manually disabled
 
#### Idle State Types
 
##### Sequential Idle States
Played in order for predictable behavior:
1. **Idle 1**: Basic standing pose (5.0 seconds)
2. **Idle 2**: Looking around (5.0 seconds)
3. **Idle 3**: Equipment check (5.0 seconds)
4. **Loop back to Idle 1**: Continuous cycle
 
##### Random Idle States
Selected randomly for varied behavior:
- **Random Selection**: Chooses from available states randomly
- **Previous State Avoidance**: Avoids repeating the same state immediately
- **Weighted Selection**: Can be extended to include weighted probability
 
#### Integration Points
 
##### Input System Integration
- **Movement Input**: Deactivates idle when movement keys are pressed
- **Look Input**: Deactivates idle when mouse/look input is detected
- **Action Input**: May or may not deactivate idle depending on configuration
 
##### Animation System Integration
- **Animation Triggers**: Idle system can trigger specific animations
- **Animation Completion**: Can wait for animation completion before state transition
- **Blend Coordination**: Coordinates with animation blending systems
 
##### Environmental Integration
- **Proximity Detection**: Can activate different idle states near interesting objects
- **Weather Effects**: Different weather might trigger different idle behaviors
- **Time of Day**: Time-based idle state selection
 
---
 
### NavMesh Override System
 
The NavMesh Override System allows external systems to temporarily take control of player movement for purposes like cutscenes, scripted sequences, or AI-directed movement. This system provides a bridge between the player's autonomous control and externally-driven movement.
 
#### Core Concepts
 
**NavMesh Override** represents a temporary state where player movement is controlled by external navigation systems rather than direct player input:
 
- **Cutscene Control**: Player follows scripted paths during cutscenes
- **AI Guidance**: AI systems direct player movement for quest objectives
- **Tutorial Sequences**: Tutorial systems guide player movement for instruction
- **Scripted Events**: Game events that require controlled player movement
 
#### Override States
 
The system supports different override states:
 
- **Inactive**: Player movement is controlled normally by input
- **Active**: Player movement is controlled by external navigation system
- **Moving**: Player is actively following navigation path
- **Reached**: Player has reached the target destination
- **Error**: Navigation system encountered an error
 
#### Component Structure
 
##### NavMeshOverride Component
 
```rust
pub struct NavMeshOverride {
    pub active: bool,                              // Whether override is currently active
    pub target_entity: Option,             // Optional target entity to follow
    pub target_position: Option,           // Optional target position to move to
    pub path_status: String,                       // Current status of the pathfinding
}
```
 
**Field Descriptions:**
- `active`: Boolean flag indicating if NavMesh override is currently active
- `target_entity`: Optional entity reference that the player should move towards
- `target_position`: Optional world position that the player should move towards
- `path_status`: String describing the current status ("Idle", "Moving", "Reached", "Error")
 
#### Event System
 
##### EnableNavMeshOverrideEvent
 
```rust
pub struct EnableNavMeshOverrideEvent {
    pub entity: Entity,  // Player entity to enable override for
}
```
 
##### DisableNavMeshOverrideEvent
 
```rust
pub struct DisableNavMeshOverrideEvent {
    pub entity: Entity,  // Player entity to disable override for
}
```
 
##### SetNavMeshTargetEvent
 
```rust
pub struct SetNavMeshTargetEvent {
    pub entity: Entity,                 // Player entity to set target for
    pub target_position: Option, // Target world position
    pub target_entity: Option, // Target entity to follow
}
```
 
#### Override Logic
 
##### Enable Process
 
1. **Entity Validation**: Validates that the entity exists and has NavMesh override component
2. **State Activation**: Sets the active flag to true
3. **Status Update**: Updates path status to indicate override is active
4. **Event Logging**: Logs the enable operation for debugging
 
##### Disable Process
 
1. **State Deactivation**: Sets active flag to false
2. **Target Cleanup**: Clears target entity and position references
3. **Status Reset**: Resets path status to "Idle"
4. **Control Return**: Returns control to normal player input systems
 
##### Target Setting Process
 
1. **Active Check**: Verifies that override is currently active
2. **Target Assignment**: Sets either target position or target entity
3. **Status Update**: Updates path status to "Moving"
4. **Path Request**: Initiates pathfinding request (in full implementation)
 
#### Movement Logic
 
##### Target Resolution
 
The system resolves targets through this process:
 
1. **Position Priority**: Uses target position if available
2. **Entity Fallback**: Uses entity position if target position is not set
3. **Validation**: Validates that resolved target is valid
4. **Distance Calculation**: Calculates distance to target
 
##### Movement Simulation
 
1. **Distance Check**: Calculates distance between player and target
2. **Proximity Detection**: Determines if player is close enough to consider target reached
3. **Movement Application**: Applies movement towards target (simplified in current implementation)
4. **Status Updates**: Updates path status based on movement progress
 
---
 
### Ragdoll System
 
The Ragdoll System manages physics-based player deaths and falls, providing realistic ragdoll physics for player characters. This system enables characters to fall realistically, respond to impacts, and recover from ragdoll states through animation blending.
 
#### Core Concepts
 
**Ragdoll Physics** provides a realistic simulation of player character bodies when they fall, die, or are knocked down:
 
- **Impact Simulation**: Realistic falling and collision with environment
- **Physics-Driven Movement**: Body parts move according to physics laws
- **Recovery Animation**: Smooth transition from ragdoll back to animation
- **Force Application**: Ability to apply forces to ragdoll bodies for dramatic effects
 
#### Ragdoll States
 
The system implements a state machine with three main states:
 
##### Animated State
- **Description**: Normal character animation and control
- **Control**: Full player control over character movement and animations
- **Transition**: Can transition to ragdolled state through events
- **Usage**: Standard gameplay state
 
##### Ragdolled State
- **Description**: Physics controls the character body
- **Control**: Physics system controls all body movement
- **Transition**: Remains in this state until conditions for recovery are met
- **Usage**: Death, falling from heights, knockout situations
 
##### BlendToAnim State
- **Description**: Transition state from ragdoll back to animation
- **Control**: Gradual return of animation control
- **Transition**: Automatically transitions to Animated state
- **Usage**: Recovery from ragdoll state
 
#### Component Structure
 
##### Ragdoll Component
 
```rust
pub struct Ragdoll {
    pub current_state: RagdollState,       // Current ragdoll state
    pub active: bool,                     // Whether ragdoll system is active
    pub time_to_get_up: f32,              // Time required to get up from ragdoll
    pub max_ragdoll_velocity: f32,        // Maximum velocity before forced ragdoll
    pub timer: f32,                       // Internal timer for state management
    pub on_ground: bool,                   // Whether ragdoll body is on ground
    pub root_bone: Option,        // Optional reference to root bone
    pub body_parts: Vec,          // Vector of all body part entities
}
```
 
**Field Descriptions:**
- `current_state`: Current state in the ragdoll state machine
- `active`: Global enable/disable flag for the ragdoll system
- `time_to_get_up`: Time in seconds required to transition from ragdolled to animated
- `max_ragdoll_velocity`: Velocity threshold that triggers automatic ragdolling
- `timer`: Internal timer used for state transitions and duration tracking
- `on_ground`: Boolean indicating if the ragdoll body is currently on the ground
- `root_bone`: Optional reference to the root bone entity for the ragdoll
- `body_parts`: Vector containing all body part entities that participate in ragdoll physics
 
##### RagdollState Enum
 
```rust
pub enum RagdollState {
    Animated,      // Normal animation state
    Ragdolled,     // Physics-controlled ragdoll state
    BlendToAnim,   // Transition state back to animation
}
```
 
#### Event System
 
##### ActivateRagdollEvent
 
```rust
pub struct ActivateRagdollEvent {
    pub entity: Entity,                    // Entity to activate ragdoll for
    pub force_direction: Option,    // Optional direction for force application
    pub force_magnitude: f32,             // Magnitude of force to apply
}
```
 
##### DeactivateRagdollEvent
 
```rust
pub struct DeactivateRagdollEvent {
    pub entity: Entity,  // Entity to deactivate ragdoll for
}
```
 
#### State Machine Logic
 
##### State Transitions
 
###### Animated to Ragdolled
1. **Trigger Detection**: Detects trigger condition for ragdoll activation
2. **Force Application**: Applies specified force if provided
3. **State Change**: Transitions to Ragdolled state
4. **Timer Reset**: Resets timer for state management
5. **Physics Activation**: Activates physics simulation for body parts
 
###### Ragdolled to BlendToAnim
1. **Timer Monitoring**: Monitors timer for duration requirements
2. **Ground Check**: Verifies ragdoll body is on ground
3. **Velocity Check**: Ensures ragdoll velocity is below threshold
4. **Transition Decision**: Makes decision to begin recovery
5. **State Change**: Transitions to BlendToAnim state
 
###### BlendToAnim to Animated
1. **Timer Accumulation**: Accumulates time during blend state
2. **Blend Calculation**: Calculates appropriate blend ratio
3. **Animation Control**: Gradually returns control to animation system
4. **State Completion**: Transitions to Animated state when blend complete
 
#### Physics Integration
 
##### Body Part Management
 
The system manages ragdoll physics through:
 
- **Entity References**: Maintains references to all body part entities
- **Hierarchy Organization**: Organizes body parts in proper hierarchy
- **Physics Properties**: Sets appropriate physics properties for each part
- **Constraint Management**: Manages constraints between body parts
 
##### Force Application
 
Force application includes:
 
- **Direction Specification**: Uses provided force direction
- **Magnitude Scaling**: Applies force with specified magnitude
- **Impact Simulation**: Simulates realistic impact forces
- **Velocity Limits**: Enforces velocity limits for stability
 
##### Ground Detection
 
Ground detection system:
 
- **Collision Detection**: Detects collision with ground objects
- **Surface Analysis**: Analyzes surface properties for realistic interaction
- **Friction Application**: Applies appropriate friction based on surface
- **Recovery Conditions**: Uses ground detection for recovery decisions
 
 
---
 
### Sprite Animator System
 
The Sprite Animator System manages sprite sheet animation states and logic for 2.5D and 2D player characters. This system provides automatic animation state transitions based on player movement, input, and character status.
 
#### Core Concepts
 
**Sprite Animation** provides frame-based animation for 2D and 2.5D characters:
 
- **State-Based Animation**: Automatic transitions between animation states
- **Direction Handling**: Sprite flipping based on movement direction
- **Movement Correlation**: Animation states correlated with movement types
- **Input Response**: Animation changes in response to player input
 
#### Animation States
 
The system supports multiple animation states:
 
##### Movement States
- **Idle**: Character is stationary
- **Walk**: Character is walking at normal speed
- **Run**: Character is running at increased speed
- **Jump**: Character is jumping or in air
- **Fall**: Character is falling downward
- **Land**: Character is landing from a fall
 
#### Component Structure
 
##### SpriteAnimator Component
 
```rust
pub struct SpriteAnimator {
    pub active: bool,                 // Whether animation system is active
    pub current_state: SpriteAnimationState, // Current animation state
    pub flip_x: bool,                 // Whether sprite should be flipped horizontally
    pub is_grounded: bool,           // Whether character is currently grounded
    pub velocity: Vec3,               // Current character velocity
}
```
 
**Field Descriptions:**
- `active`: Global enable/disable flag for the sprite animation system
- `current_state`: Currently active animation state
- `flip_x`: Boolean flag for horizontal sprite flipping
- `is_grounded`: Boolean indicating if character is on the ground
- `velocity`: Current velocity vector for movement-based state decisions
 
##### SpriteAnimationState Enum
 
```rust
pub enum SpriteAnimationState {
    Idle,   // Stationary character
    Walk,   // Walking animation
    Run,    // Running animation
    Jump,   // Jumping animation
    Fall,   // Falling animation
    Land,   // Landing animation
}
```
 
#### Animation State Logic
 
##### State Determination
 
The system determines the appropriate animation state through this logic:
 
1. **Grounded Check**: Determines if character is grounded or in air
2. **Vertical Velocity Analysis**: Analyzes vertical velocity for jump/fall states
3. **Horizontal Speed Calculation**: Calculates horizontal movement speed
4. **Speed Threshold Comparison**: Compares speed against thresholds for walk/run
5. **State Assignment**: Assigns appropriate animation state
 
##### State Priority System
 
Animation states follow this priority order:
 
1. **Jump/Fall**: Highest priority for aerial states
2. **Land**: High priority for landing transitions
3. **Run**: Higher priority for fast movement
4. **Walk**: Medium priority for normal movement
5. **Idle**: Lowest priority for stationary state
 
##### Speed Thresholds
 
The system uses speed thresholds for state transitions:
 
- **Idle Threshold**: Speed < 0.1 (stationary)
- **Walk Threshold**: Speed 0.1 - 5.0 (walking speed)
- **Run Threshold**: Speed > 5.0 (running speed)
 
#### Direction Handling
 
##### Flip Logic
 
Sprite flipping is handled through this logic:
 
1. **Movement Analysis**: Analyzes horizontal movement direction
2. **Direction Determination**: Determines if character is moving left or right
3. **Flip Decision**: Decides whether to flip sprite based on direction
4. **Component Update**: Updates both animator and sprite component flip flags
 
---
 
### Upper Body Rotation System
 
The Upper Body Rotation System handles procedural rotation of the player's upper body (spine, chest) to face targets. This system provides realistic character aiming and looking behavior that enhances player immersion and gameplay mechanics.
 
#### Core Concepts
 
**Upper Body Rotation** allows characters to:
 
- **Aim at Targets**: Rotate upper body to aim weapons or look at targets
- **Environmental Awareness**: Look at interesting objects or points of interest
- **Combat Enhancement**: Improve combat positioning and aiming
- **Realistic Movement**: Provide more natural and realistic character movement
 
#### Rotation Components
 
The system targets specific body parts:
 
##### Spine Rotation
- **Primary Rotation**: Main rotation component for upper body
- **Movement Range**: Configurable rotation limits and constraints
- **Smooth Transitions**: Smooth rotation to avoid jerky movements
 
##### Chest Rotation
- **Secondary Rotation**: Additional rotation for more detailed control
- **Independent Control**: Can rotate independently from spine
- **Fine Adjustment**: Provides fine-tuned aiming control
 
#### Component Structure
 
##### UpperBodyRotation Component
 
```rust
pub struct UpperBodyRotation {
    pub enabled: bool,                        // Whether system is enabled
    pub rotation_speed: f32,                  // Speed of rotation interpolation
    pub max_bending_angle: f32,               // Maximum rotation angle
    pub horizontal_enabled: bool,             // Enable horizontal rotation
    pub vertical_enabled: bool,              // Enable vertical rotation
    pub spine_bone: Option,           // Reference to spine bone entity
    pub chest_bone: Option,           // Reference to chest bone entity
}
```
 
**Field Descriptions:**
- `enabled`: Global enable/disable flag for the upper body rotation system
- `rotation_speed`: Speed multiplier for rotation interpolation (higher = faster)
- `max_bending_angle`: Maximum angle in degrees that the body can rotate
- `horizontal_enabled`: Boolean flag to enable horizontal (left-right) rotation
- `vertical_enabled`: Boolean flag to enable vertical (up-down) rotation
- `spine_bone`: Optional reference to the spine bone entity for rotation
- `chest_bone`: Optional reference to the chest bone entity for rotation
 
##### UpperBodyTarget Component
 
```rust
pub struct UpperBodyTarget {
    pub target_entity: Option,        // Entity to look at
    pub target_position: Option,       // World position to look at
}
```
 
**Field Descriptions:**
- `target_entity`: Optional entity reference that the character should look at
- `target_position`: Optional world position that the character should look at
 
#### Rotation Logic
 
##### Target Resolution
 
The system resolves targets through this priority:
 
1. **Entity Target**: Uses target entity if provided and valid
2. **Position Target**: Uses target position if entity target is not available
3. **Default Behavior**: Returns to neutral position if no target is available
 
##### Rotation Calculation
 
1. **Direction Vector**: Calculates direction from character to target
2. **Local Transformation**: Converts world direction to local character space
3. **Angle Limitation**: Limits rotation angle to maximum bending angle
4. **Constraint Application**: Applies horizontal/vertical constraints
5. **Rotation Interpolation**: Interpolates from current to target rotation
 
##### Smooth Transitions
 
The system implements smooth transitions through:
 
1. **Current Rotation**: Captures current bone rotation
2. **Target Rotation**: Calculates desired rotation towards target
3. **Interpolation**: Uses spherical linear interpolation (slerp) for smooth transitions
4. **Speed Control**: Controls transition speed through rotation_speed parameter
 
---
 
### Extra Movement Systems
 
The Extra Movement Systems provide advanced movement capabilities beyond basic walking and running. These systems handle specialized movement types like flying, swimming, wall-running, and other unique movement mechanics that can be enabled based on game requirements.
 
#### Movement System Overview
 
Each extra movement system is implemented as a separate plugin that can be enabled independently:
 
- **Fly System**: Enables free-form flying movement
- **Jetpack System**: Provides limited-duration flying with fuel management
- **Wall Run System**: Allows running along walls and transitioning between surfaces
- **Swim System**: Handles underwater movement and swimming mechanics
- **Paraglider System**: Enables gliding through the air with controlled descent
- **Roll on Landing System**: Adds rolling animations and mechanics for falls
- **Sphere Mode System**: Allows character to roll in spherical form
- **Free Fall System**: Handles extended falling sequences with style
 
#### Common Architecture
 
All extra movement systems follow a common architectural pattern:
 
##### Plugin Structure
Each system is implemented as a Bevy plugin that:
- Registers necessary components and resources
- Sets up event systems for system communication
- Configures update systems for movement processing
- Integrates with input and physics systems
 
##### Component Design
Common component patterns include:
- **Movement State**: Current state of the movement system
- **Configuration**: Movement parameters and limits
- **Input Handling**: Input mapping for the movement type
- **Physics Integration**: Integration with physics systems
 
#### Fly System
 
##### Core Functionality
The Fly System enables three-dimensional free-form movement:
 
- **3D Movement**: Full freedom of movement in three dimensions
- **Altitude Control**: Vertical movement and altitude management
- **Speed Control**: Variable movement speeds
- **Direction Control**: Independent direction and altitude control
 
#### Jetpack System
 
##### Core Functionality
The Jetpack System provides controlled flight with resource management:
 
- **Fuel System**: Limited-duration flight with fuel consumption
- **Thrust Control**: Variable thrust for ascent and maneuverability
- **Heat Management**: Heat buildup system for realistic jetpack behavior
- **Emergency Systems**: Emergency fuel and safety features
 
#### Wall Run System
 
##### Core Functionality
The Wall Run System enables movement along vertical surfaces:
 
- **Surface Detection**: Automatic detection of wall-running surfaces
- **Momentum Conservation**: Conservation of momentum when entering wall runs
- **Jump Transitions**: Smooth transitions from wall runs to jumps
- **Gravity Management**: Altered gravity during wall-running sequences
 
#### Swim System
 
##### Core Functionality
The Swim System handles underwater and surface swimming:
 
- **Underwater Movement**: Full three-dimensional underwater movement
- **Surface Swimming**: Swimming along the water surface
- **Breath Management**: Oxygen and breathing mechanics
- **Current Systems**: Water current effects on movement
 
#### System Integration
 
##### Input Integration
Each system integrates with the input system through:
- **Custom Input Mapping**: Input mappings specific to each movement type
- **Context-Sensitive Input**: Input that changes based on current movement state
- **Combination Input**: Input combinations for advanced movements
 
##### Physics Integration
All systems integrate with physics through:
- **Collision Handling**: Proper collision detection and response
- **Mass and Inertia**: Appropriate mass and inertia values
- **Force Application**: Proper force application for movement
- **Constraint Management**: Management of physics constraints
 
##### Animation Integration
Integration with animation systems includes:
- **State Animation**: Appropriate animations for each movement state
- **Transition Animation**: Smooth transitions between movement types
- **Blend Management**: Animation blending between different movement states
- **Speed Animation**: Animation speed matching movement speed
 
---
 
## Advanced Features
 
### Priority-Based State Management
 
The Player System implements a sophisticated priority-based state management system that allows for complex interactions between different player states and behaviors.
 
#### Priority Hierarchy
 
States are organized in a hierarchical priority system:
 
##### Critical States (Priority 100+)
- **Death State**: Player is dead and cannot perform any actions
- **Stunned State**: Player is immobilized and cannot act
- **Cutscene State**: Player is under external control
- **Loading State**: Player is in loading or transition state
 
##### High Priority States (Priority 50-99)
- **Combat States**: Active combat situations requiring full attention
- **Special Abilities**: Use of powerful abilities that require focus
- **Vehicle Control**: Driving or piloting vehicles
- **Important Interactions**: Critical interactions with game objects
 
##### Medium Priority States (Priority 10-49)
- **Movement States**: Different types of movement (walking, running, swimming)
- **Standard Interactions**: Normal interactions with the environment
- **Exploration States**: Exploration and non-critical activities
- **Social Interactions**: Communication and social activities
 
##### Low Priority States (Priority 1-9)
- **Idle States**: Various idle behaviors and animations
- **Background Activities**: Background activities that can be interrupted
- **Ambient States**: States related to ambient environmental interactions
 
#### State Interaction Rules
 
##### Interrupt Handling
- **Higher Priority Interrupts Lower**: States with higher priority can interrupt lower priority states
- **Same Priority Coordination**: States with the same priority must coordinate or queue
- **Interrupt Permission**: Each state can specify whether it can be interrupted
- **Graceful Transitions**: Interruptions should be handled gracefully with appropriate transitions
 
##### Concurrent State Management
- **Independent States**: Some states can run concurrently if they don't conflict
- **Shared Resource Coordination**: States that share resources must coordinate access
- **Conflict Resolution**: Clear rules for resolving state conflicts
- **Priority Resolution**: Priority-based resolution of conflicting state requests
 
### Event-Driven Architecture
 
The Player System uses an event-driven architecture that promotes loose coupling and flexible system interactions.
 
#### Event Types
 
##### State Change Events
Events fired when player states change:
- **PlayerStateChangedEvent**: Fired when player state changes
- **PlayerModeChangedEvent**: Fired when player mode changes
- **IdleStateChangedEvent**: Fired when idle state changes
 
##### Action Events
Events triggered by player actions:
- **JumpEvent**: Fired when player jumps
- **InteractEvent**: Fired when player interacts with objects
- **UseAbilityEvent**: Fired when player uses abilities
 
##### System Events
Events for system-level communications:
- **SystemEnableEvent**: Fired when systems are enabled/disabled
- **ConfigurationChangeEvent**: Fired when configuration changes
- **ErrorEvent**: Fired when errors occur
 
#### Event Processing
 
##### Event Queues
Events are processed through efficient queue systems:
- **Batch Processing**: Events are processed in batches for efficiency
- **Priority Processing**: Events can be processed based on priority
- **Filtering**: Events can be filtered based on criteria
- **Debouncing**: Rapid events can be debounced to prevent spam
 
##### Event Handling
Event handling follows established patterns:
- **Immediate Processing**: Some events are processed immediately
- **Deferred Processing**: Some events are processed in subsequent frames
- **Error Handling**: Robust error handling for event processing
- **Logging**: Comprehensive logging for debugging and monitoring
 
#### System Communication
 
##### Loose Coupling
Systems communicate through events rather than direct references:
- **Reduced Dependencies**: Systems have minimal direct dependencies
- **Flexible Architecture**: Systems can be added or removed easily
- **Testing Friendly**: Event-driven systems are easier to test
- **Maintainability**: Easier to maintain and modify systems
 
##### Event Documentation
All events are well-documented:
- **Event Purpose**: Clear explanation of why the event exists
- **Event Data**: Detailed description of event data fields
- **Usage Examples**: Examples of how to use the events
- **Integration Notes**: Notes on how to integrate with the events
 
### Animation Integration
 
The Player System provides comprehensive integration with animation systems to ensure smooth and realistic character behavior.
 
#### Animation State Mapping
 
##### Movement Animation Mapping
Each movement state is mapped to appropriate animations:
- **Idle Animations**: Various idle animations for different contexts
- **Walk/Run Animations**: Movement animations with speed variation
- **Jump/Fall Animations**: Aerial animations for jumping and falling
- **Landing Animations**: Landing animations with impact variation
 
##### State Transition Animation
Smooth transitions between animation states:
- **Blend Trees**: Use of blend trees for smooth transitions
- **Transition Duration**: Configurable transition durations
- **Interruption Handling**: Proper handling of animation interruptions
- **Priority Blending**: Priority-based animation blending
 
#### Animation Customization
 
##### Animation Selection
Flexible animation selection based on context:
- **Context-Aware Selection**: Animations selected based on game context
- **Random Variation**: Random variation in animation selection
- **Skill-Based Variation**: Animation variation based on player skills
- **Equipment Variation**: Animation variation based on equipped items
 
##### Animation Modification
Dynamic modification of animations:
- **Speed Modification**: Animation speed based on movement speed
- **Direction Modification**: Animation direction based on movement direction
- **Intensity Modification**: Animation intensity based on action intensity
- **Blend Modification**: Dynamic animation blending based on conditions
 
#### Performance Optimization
 
##### Animation Caching
Efficient animation management:
- **Animation Pooling**: Reuse of animation instances where possible
- **LOD Animation**: Level-of-detail animation systems
- **Streaming Animation**: Streaming of animations for memory efficiency
- **Compression**: Animation compression for storage efficiency
 
##### Update Optimization
Optimized animation updates:
- **Conditional Updates**: Only update animations when necessary
- **Culling**: Cull animations that are not visible or active
- **Batching**: Batch animation updates for efficiency
- **Multithreading**: Multithreaded animation processing where possible
 
 
---
 
## System Integration
 
### Character Controller Integration
 
The Player System integrates seamlessly with the Character Controller system to provide comprehensive character movement and behavior.
 
#### Movement Coordination
 
##### State Coordination
The Player System coordinates with the Character Controller through:
- **Movement State Sharing**: Shared movement state information
- **Priority Coordination**: Priority-based coordination of movement requests
- **Resource Coordination**: Coordination of shared movement resources
- **Timing Coordination**: Synchronized timing of movement operations
 
##### Control Flow
The control flow between systems follows this pattern:
1. **Player System**: Determines desired player state and behavior
2. **Character Controller**: Processes movement based on player state
3. **Physics System**: Applies physics-based movement and collision
4. **Animation System**: Updates animations based on movement
5. **Feedback Systems**: Provide feedback to player about movement state
 
#### Integration Points
 
##### Component Integration
Components from both systems work together:
- **Movement Components**: Player System movement components integrate with Character Controller components
- **State Components**: State management components coordinate between systems
- **Input Components**: Input components provide unified input handling
- **Physics Components**: Physics components provide unified physics interaction
 
##### System Communication
Systems communicate through well-defined interfaces:
- **Event Interfaces**: Event-based communication between systems
- **Resource Interfaces**: Shared resource access patterns
- **Query Interfaces**: Efficient query patterns for data access
- **Update Interfaces**: Coordinated update scheduling
 
### Camera System Integration
 
The Player System integrates with the Camera System to provide camera behavior that follows player state and movement.
 
#### Camera State Coordination
 
##### Movement-Based Camera
Camera behavior changes based on player movement:
- **Idle Camera**: Calm camera behavior when player is idle
- **Movement Camera**: Dynamic camera behavior during movement
- **Combat Camera**: Focused camera behavior during combat
- **Special Camera**: Special camera modes for special activities
 
##### Camera Modes
Different camera modes based on player state:
- **First Person**: First-person camera for certain activities
- **Third Person**: Third-person camera for standard gameplay
- **Overhead**: Overhead camera for certain special situations
- **Cinematic**: Cinematic camera for special sequences
 
#### Camera Control Integration
 
##### Input Integration
Camera input integrates with player input:
- **Shared Input**: Shared input handling for camera and player control
- **Priority Handling**: Priority-based handling of competing input
- **Context Switching**: Automatic switching of input context
- **Smooth Transitions**: Smooth transitions between input modes
 
##### State-Based Camera
Camera behavior based on player state:
- **State Notifications**: Camera system receives notifications of player state changes
- **Automatic Adjustment**: Automatic camera adjustment based on player state
- **Manual Override**: Manual camera override capabilities
- **Smooth Transitions**: Smooth camera transitions between states
 
### Input System Integration
 
The Player System integrates with the Input System to provide comprehensive and responsive input handling.
 
#### Input Event Processing
 
##### Event Handling
Input events are processed through the Player System:
- **Event Filtering**: Filtering of input events based on player state
- **Event Prioritization**: Prioritization of input events
- **Event Combination**: Combination of input events for complex actions
- **Event Translation**: Translation of raw input to game actions
 
##### Input State Management
Input state is managed comprehensively:
- **State Persistence**: Persistence of input state across frames
- **State History**: Maintenance of input history for analysis
- **State Prediction**: Prediction of future input state
- **State Smoothing**: Smoothing of input state for better feel
 
#### Input Customization
 
##### Rebinding Support
Comprehensive input rebinding support:
- **Full Rebinding**: Complete rebinding of all input actions
- **Context Sensitivity**: Context-sensitive input rebinding
- **Preset Management**: Management of input binding presets
- **Validation**: Validation of input bindings
 
##### Sensitivity Settings
Fine-grained sensitivity settings:
- **Per-Action Sensitivity**: Individual sensitivity settings for each action
- **Nonlinear Scaling**: Nonlinear sensitivity scaling options
- **Device-Specific Settings**: Device-specific sensitivity settings
- **Adaptive Sensitivity**: Adaptive sensitivity based on context
 
### Animation System Integration
 
The Player System integrates with animation systems to provide smooth and realistic character animation.
 
#### Animation Coordination
 
##### State-Based Animation
Animation state coordinates with player state:
- **Automatic Triggers**: Automatic animation triggers based on player state
- **State Transitions**: Smooth transitions between animation states
- **Priority Handling**: Priority-based handling of animation requests
- **Blending Coordination**: Coordinated animation blending
 
##### Movement-Based Animation
Animation based on player movement:
- **Speed Matching**: Animation speed matching movement speed
- **Direction Matching**: Animation direction matching movement direction
- **Intensity Matching**: Animation intensity matching action intensity
- **Context Matching**: Animation context matching situation
 
#### Animation Customization
 
##### Animation Selection
Flexible animation selection:
- **Random Variation**: Random variation in animation selection
- **Skill-Based Selection**: Animation selection based on player skills
- **Equipment-Based Selection**: Animation selection based on equipped items
- **Context-Based Selection**: Animation selection based on context
 
##### Animation Modification
Dynamic animation modification:
- **Speed Modification**: Real-time animation speed modification
- **Blend Modification**: Real-time animation blend modification
- **Layer Modification**: Real-time animation layer modification
- **Weight Modification**: Real-time animation weight modification
 
### Physics System Integration
 
The Player System integrates with physics systems to provide realistic physical interaction.
 
#### Physics Coordination
 
##### Movement Physics
Physics-based player movement:
- **Force Application**: Application of forces for movement
- **Velocity Control**: Control of velocity for movement
- **Acceleration Control**: Control of acceleration for movement
- **Deceleration Control**: Control of deceleration for movement
 
##### Collision Physics
Physics-based collision handling:
- **Collision Detection**: Robust collision detection
- **Collision Response**: Realistic collision response
- **Friction Handling**: Proper friction handling
- **Restitution Handling**: Proper restitution handling
 
#### Physics Customization
 
##### Physics Properties
Customizable physics properties:
- **Mass Configuration**: Configurable mass properties
- **Inertia Configuration**: Configurable inertia properties
- **Friction Configuration**: Configurable friction properties
- **Drag Configuration**: Configurable drag properties
 
##### Physics Behavior
Customizable physics behavior:
- **Gravity Configuration**: Configurable gravity behavior
- **Constraint Configuration**: Configurable constraint behavior
- **Joint Configuration**: Configurable joint behavior
- **Material Configuration**: Configurable material behavior
 
### UI System Integration
 
The Player System integrates with UI systems to provide visual feedback about player state and behavior.
 
#### State Visualization
 
##### State Indicators
Visual indicators of player state:
- **State Icons**: Icons representing different player states
- **State Bars**: Progress bars for time-limited states
- **State Colors**: Color coding for different state types
- **State Text**: Text descriptions of current player state
 
##### Status Display
Comprehensive status display:
- **Health Status**: Display of health and vitality
- **Resource Status**: Display of various resources (energy, mana, etc.)
- **Ability Status**: Display of ability states and cooldowns
- **Movement Status**: Display of movement capabilities
 
#### Interaction Feedback
 
##### Action Feedback
Feedback for player actions:
- **Action Confirmation**: Confirmation of successful actions
- **Action Failure**: Feedback for failed actions
- **Action Progress**: Progress indication for ongoing actions
- **Action Timing**: Timing feedback for actions
 
##### Environmental Feedback
Feedback from the environment:
- **Interaction Prompts**: Prompts for possible interactions
- **Environmental Warnings**: Warnings about environmental hazards
- **Opportunity Indicators**: Indicators of opportunities and possibilities
- **Status Updates**: Updates about environmental status
 
---
 
## Usage Patterns
 
### Basic Player Setup
 
The most common usage pattern involves setting up a basic player character with standard movement capabilities.
 
#### Player Entity Creation
 
```rust
// Basic player entity setup
let player_entity = commands.spawn((
    Player,
    PlayerModesSystem::default(),
    PlayerStateSystem::default(),
    PlayerIdleSystem::default(),
    // Add other player components as needed
)).id();
```
 
#### Component Configuration
 
```rust
// Configure player modes
let mut player_modes = PlayerModesSystem::default();
player_modes.player_modes = vec![
    PlayerMode {
        name: "Weapons".to_string(),
        mode_enabled: true,
        is_current_state: true,
    },
    PlayerMode {
        name: "Powers".to_string(),
        mode_enabled: true,
        is_current_state: false,
    },
];
// Configure player states
let mut player_states = PlayerStateSystem::default();
player_states.player_state_list = vec![
    PlayerStateInfo {
        name: "Walking".to_string(),
        state_enabled: true,
        state_active: true,
        state_priority: 10,
        can_be_interrupted: true,
        use_state_duration: false,
        state_duration: 0.0,
        current_duration_timer: 0.0,
    },
    // Add more states as needed
];
```
 
#### Integration Setup
 
```rust
// Setup input system integration
app.add_plugins(InputPlugin)
   .add_plugins(PlayerPlugin);
// Setup animation system integration
app.add_systems(Update, (
    update_player_animation,
    handle_player_state_changes,
));
```
 
### Advanced Movement Setup
 
For games requiring advanced movement capabilities, the Player System can be configured with multiple movement systems.
 
#### Extra Movement Configuration
 
```rust
// Enable multiple movement systems
app.add_plugins((
    ExtraMovementsPlugin,
    fly::FlyPlugin,
    swim::SwimPlugin,
    wall_run::WallRunPlugin,
    jetpack::JetpackPlugin,
));
// Configure player for multiple movement types
let mut player_states = PlayerStateSystem::default();
player_states.player_state_list = vec![
    PlayerStateInfo {
        name: "Flying".to_string(),
        state_enabled: true,
        state_active: false,
        state_priority: 30,
        can_be_interrupted: true,
        use_state_duration: false,
        state_duration: 0.0,
        current_duration_timer: 0.0,
    },
    PlayerStateInfo {
        name: "Swimming".to_string(),
        state_enabled: true,
        state_active: false,
        state_priority: 25,
        can_be_interrupted: true,
        use_state_duration: false,
        state_duration: 0.0,
        current_duration_timer: 0.0,
    },
    // Add more movement states
];
```
 
#### Movement State Management
 
```rust
// System to handle movement state changes
pub fn handle_movement_state_changes(
    mut set_state_queue: ResMut,
    keys: Res<ButtonInput>,
    query: Query,
) {
    let player_entity = /* get player entity */;
    
    // Handle fly mode activation
    if keys.just_pressed(KeyCode::KeyF) {
        set_state_queue.0.push(SetPlayerStateEvent {
            player_entity,
            state_name: "Flying".to_string(),
        });
    }
    
    // Handle swim mode activation
    if keys.just_pressed(KeyCode::KeyS) {
        set_state_queue.0.push(SetPlayerStateEvent {
            player_entity,
            state_name: "Swimming".to_string(),
        });
    }
}
```
 
### Combat Integration Pattern
 
Games with combat systems can integrate the Player System with combat mechanics for enhanced gameplay.
 
#### Combat State Integration
 
```rust
// Integrate with combat system
pub fn handle_combat_states(
    mut set_mode_queue: ResMut,
    combat_events: Res,
    query: Query,
) {
    let player_entity = /* get player entity */;
    
    // Switch to combat mode when combat starts
    if combat_events.combat_started {
        set_mode_queue.0.push(SetPlayerModeEvent {
            player_entity,
            mode_name: "Combat".to_string(),
        });
    }
    
    // Switch back to exploration mode when combat ends
    if combat_events.combat_ended {
        set_mode_queue.0.push(SetPlayerModeEvent {
            player_entity,
            mode_name: "Exploration".to_string(),
        });
    }
}
```
 
#### Combat Animation Integration
 
```rust
// Integrate with combat animations
pub fn handle_combat_animations(
    mut sprite_query: Query,
    combat_events: Res,
) {
    for mut animator in sprite_query.iter_mut() {
        // Trigger combat animations
        if combat_events.attack_initiated {
            animator.current_state = SpriteAnimationState::Attack;
        }
        
        // Return to movement animations
        if combat_events.attack_completed {
            animator.current_state = SpriteAnimationState::Idle;
        }
    }
}
```
 
### Vehicle Integration Pattern
 
For games with vehicles, the Player System can integrate with vehicle systems for seamless transitions.
 
#### Vehicle State Management
 
```rust
// Handle vehicle entry/exit
pub fn handle_vehicle_states(
    mut set_control_queue: ResMut,
    vehicle_events: Res,
    query: Query,
) {
    let player_entity = /* get player entity */;
    
    // Enter vehicle
    if vehicle_events.vehicle_entered {
        set_control_queue.0.push(SetControlStateEvent {
            player_entity,
            state_name: "Driving".to_string(),
        });
    }
    
    // Exit vehicle
    if vehicle_events.vehicle_exited {
        set_control_queue.0.push(SetControlStateEvent {
            player_entity,
            state_name: "Regular".to_string(),
        });
    }
}
```
 
#### Vehicle Animation Integration
 
```rust
// Integrate with vehicle animations
pub fn handle_vehicle_animations(
    mut animator_query: Query,
    vehicle_state: Res,
) {
    for mut animator in animator_query.iter_mut() {
        // Use vehicle-specific animations
        if vehicle_state.in_vehicle {
            animator.current_state = SpriteAnimationState::Driving;
        } else {
            // Return to character animations
            animator.current_state = SpriteAnimationState::Idle;
        }
    }
}
```
 
### AI Integration Pattern
 
For games with AI systems, the Player System can be controlled by AI for scripted sequences.
 
#### NavMesh Override Setup
 
```rust
// Setup AI-controlled player movement
pub fn setup_ai_control(
    mut commands: Commands,
    mut nav_override_queue: ResMut,
    ai_targets: Res,
) {
    let player_entity = /* get player entity */;
    
    // Add NavMesh override component if not present
    if !nav_override_query.get(player_entity).is_ok() {
        commands.entity(player_entity).insert(NavMeshOverride::default());
    }
    
    // Set AI target
    nav_override_queue.0.push(SetNavMeshTargetEvent {
        entity: player_entity,
        target_position: Some(ai_targets.next_target),
        target_entity: None,
    });
}
```
 
#### AI Sequence Management
 
```rust
// Manage AI sequences
pub fn manage_ai_sequences(
    mut enable_override_queue: ResMut,
    mut disable_override_queue: ResMut,
    ai_state: Res,
    query: Query,
) {
    let player_entity = /* get player entity */;
    
    // Enable AI control
    if ai_state.sequence_started && !query.get(player_entity).unwrap().active {
        enable_override_queue.0.push(EnableNavMeshOverrideEvent {
            entity: player_entity,
        });
    }
    
    // Disable AI control
    if ai_state.sequence_completed && query.get(player_entity).unwrap().active {
        disable_override_queue.0.push(DisableNavMeshOverrideEvent {
            entity: player_entity,
        });
    }
}
```
 
### Save System Integration
 
The Player System can integrate with save systems to persist player state.
 
#### State Serialization
 
```rust
// Serialize player state for saving
pub fn serialize_player_state(
    query: Query,
) -> PlayerStateData {
    let (modes, states, idle) = query.single();
    
    PlayerStateData {
        current_mode: modes.current_players_mode_name.clone(),
        current_state: states.current_state_name.clone(),
        current_idle: idle.current_idle_index,
        // Serialize other state data
    }
}
// Deserialize player state from save
pub fn deserialize_player_state(
    state_data: PlayerStateData,
    mut set_mode_queue: ResMut,
    mut set_state_queue: ResMut,
) {
    let player_entity = /* get player entity */;
    
    // Restore player mode
    set_mode_queue.0.push(SetPlayerModeEvent {
        player_entity,
        mode_name: state_data.current_mode,
    });
    
    // Restore player state
    set_state_queue.0.push(SetPlayerStateEvent {
        player_entity,
        state_name: state_data.current_state,
    });
}
```
 
### Multiplayer Integration Pattern
 
For multiplayer games, the Player System needs to be adapted for network synchronization.
 
#### Network State Synchronization
 
```rust
// Synchronize player state over network
pub fn sync_player_state(
    mut query: Query,
    network_events: Res,
) {
    // Apply remote state changes
    for event in network_events.state_changes.iter() {
        if let Ok(mut states) = query.get_mut(event.player_entity) {
            states.current_state_name = event.new_state.clone();
        }
    }
}
```
 
#### Network Event Handling
 
```rust
// Handle network events
pub fn handle_network_events(
    mut set_mode_queue: ResMut,
    mut set_state_queue: ResMut,
    network_events: Res,
) {
    for event in network_events.mode_changes.iter() {
        set_mode_queue.0.push(SetPlayerModeEvent {
            player_entity: event.player_entity,
            mode_name: event.mode_name.clone(),
        });
    }
    
    for event in network_events.state_changes.iter() {
        set_state_queue.0.push(SetPlayerStateEvent {
            player_entity: event.player_entity,
            state_name: event.state_name.clone(),
        });
    }
}
```