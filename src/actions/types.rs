use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Default, PartialEq, Reflect)]
pub enum ActionState {
    #[default]
    Idle,
    WalkingToTargetBefore,
    AdjustingTransform,
    PlayingAnimation,
    WalkingToTargetAfter,
    Finished,
}

/// Walk-to-target state for NavMesh integration
#[derive(Debug, Clone, Copy, Default, PartialEq, Reflect)]
pub enum WalkToTargetState {
    #[default]
    Idle,
    Walking,
    ReachedTarget,
    TimedOut,
}

/// Camera event types for cinematic effects
#[derive(Debug, Clone, Reflect, PartialEq)]
pub enum CameraEventType {
    None,
    Shake { intensity: f32, duration: f32, frequency: f32 },
    Zoom { target_fov: f32, duration: f32 },
    Focus { target_position: Vec3, duration: f32 },
    Offset { offset: Vec3, duration: f32 },
}

impl Default for CameraEventType {
    fn default() -> Self {
        Self::None
    }
}

/// Physics event types for physical effects
#[derive(Debug, Clone, Reflect, PartialEq)]
pub enum PhysicsEventType {
    None,
    ApplyForce { direction: Vec3, magnitude: f32 },
    ApplyImpulse { direction: Vec3, magnitude: f32 },
    SetVelocity { velocity: Vec3, additive: bool },
    EnableCollider { enabled: bool },
    ApplyTorque { axis: Vec3, magnitude: f32 },
}

impl Default for PhysicsEventType {
    fn default() -> Self {
        Self::None
    }
}

/// State change event types for player state transitions
#[derive(Debug, Clone, Reflect, PartialEq)]
pub enum StateChangeEventType {
    None,
    ChangePlayerState { state_name: String, force: bool },
    ChangePlayerMode { mode_name: String },
    ChangeControlState { control_state: String },
    EnableState { state_name: String, enabled: bool },
}

impl Default for StateChangeEventType {
    fn default() -> Self {
        Self::None
    }
}

/// Weapon event types for triggering weapon actions
#[derive(Debug, Clone, Reflect, PartialEq)]
pub enum WeaponEventType {
    None,
    Fire { burst_count: i32, delay_between_shots: f32 },
    Reload,
    Aim { enable: bool },
    SwitchWeapon { weapon_index: i32 },
    ThrowGrenade { force: f32, direction: Vec3 },
}

impl Default for WeaponEventType {
    fn default() -> Self {
        Self::None
    }
}

/// Power event types for consuming/restoring power
#[derive(Debug, Clone, Reflect, PartialEq)]
pub enum PowerEventType {
    None,
    ConsumePower { amount: f32 },
    RestorePower { amount: f32 },
    DrainOverTime { amount_per_second: f32, duration: f32 },
    RequirePower { minimum_amount: f32 },
}

impl Default for PowerEventType {
    fn default() -> Self {
        Self::None
    }
}

/// Player state control flags for managing player systems during actions
#[derive(Debug, Clone, Reflect, PartialEq)]
pub struct PlayerStateControl {
    pub disable_movement: bool,
    pub disable_rotation: bool,
    pub disable_input: bool,
    pub disable_gravity: bool,
    pub collider_as_trigger: bool,
    pub disable_collider: bool,
    pub hide_hud: bool,
    pub invincible: bool,
    pub preserve_crouch: bool,
    pub preserve_strafe: bool,
    
    // Weapon control
    pub disable_weapon_input: bool,
    pub force_aim: bool,
    pub disable_weapon_switching: bool,
    pub preserve_aim_state: bool,
    
    // Power control
    pub disable_power_usage: bool,
    pub power_drain_multiplier: f32,
}

impl Default for PlayerStateControl {
    fn default() -> Self {
        Self {
            disable_movement: false,
            disable_rotation: false,
            disable_input: false,
            disable_gravity: false,
            collider_as_trigger: false,
            disable_collider: false,
            hide_hud: false,
            invincible: false,
            preserve_crouch: false,
            preserve_strafe: false,
            disable_weapon_input: false,
            force_aim: false,
            disable_weapon_switching: false,
            preserve_aim_state: false,
            disable_power_usage: false,
            power_drain_multiplier: 1.0,
        }
    }
}

/// Condition that must be met for an event to fire
#[derive(Debug, Clone, Reflect, PartialEq)]
pub enum EventCondition {
    None,
    PlayerOnGround,
    PlayerInAir,
    PlayerCrouching,
    PlayerSprinting,
    ActionProgressGreaterThan(f32),
    ActionProgressLessThan(f32),
    ActionProgressBetween(f32, f32),
    HealthGreaterThan(f32),
    HealthLessThan(f32),
    DistanceToTargetLessThan(f32),
    DistanceToTargetGreaterThan(f32),
    CustomCondition(String),
}

impl Default for EventCondition {
    fn default() -> Self {
        Self::None
    }
}

/// Action event that triggers at a specific time during action
#[derive(Debug, Clone, Reflect)]
pub struct ActionEvent {
    pub delay_to_activate: f32,
    pub event_triggered: bool,
    
    // Event types
    pub use_bevy_event: bool,
    pub bevy_event_name: String,
    
    pub use_remote_event: bool,
    pub remote_event_name: String,
    
    pub send_player_entity: bool,
    pub call_if_action_stopped: bool,
    
    // Conditions
    pub condition: EventCondition,
    pub check_condition_continuously: bool,
    
    // Animation timing
    pub use_animation_timing: bool,
    pub animation_normalized_time: f32,  // 0.0 to 1.0
    
    // Camera events
    pub use_camera_event: bool,
    pub camera_event_type: CameraEventType,
    
    // Physics events
    pub use_physics_event: bool,
    pub physics_event_type: PhysicsEventType,
    pub physics_target_self: bool,  // Apply to player (true) or target (false)
    
    // State change events
    pub use_state_change_event: bool,
    pub state_change_event_type: StateChangeEventType,
    
    // Weapon events
    pub use_weapon_event: bool,
    pub weapon_event_type: WeaponEventType,
    pub weapon_target_entity: Option<Entity>,
    
    // Power events
    pub use_power_event: bool,
    pub power_event_type: PowerEventType,
    pub power_event_affects_player: bool,
}

impl Default for ActionEvent {
    fn default() -> Self {
        Self {
            delay_to_activate: 0.0,
            event_triggered: false,
            use_bevy_event: false,
            bevy_event_name: String::new(),
            use_remote_event: false,
            remote_event_name: String::new(),
            send_player_entity: false,
            call_if_action_stopped: false,
            condition: EventCondition::None,
            check_condition_continuously: false,
            use_animation_timing: false,
            animation_normalized_time: 0.0,
            use_camera_event: false,
            camera_event_type: CameraEventType::None,
            use_physics_event: false,
            physics_event_type: PhysicsEventType::None,
            physics_target_self: true,
            use_state_change_event: false,
            state_change_event_type: StateChangeEventType::None,
            use_weapon_event: false,
            weapon_event_type: WeaponEventType::None,
            weapon_target_entity: None,
            use_power_event: false,
            power_event_type: PowerEventType::None,
            power_event_affects_player: true,
        }
    }
}

/// Animator parameters for action system
#[derive(Component, Debug, Reflect, Clone)]
#[reflect(Component)]
pub struct AnimatorParameters {
    pub action_active: bool,
    pub action_id: i32,
    pub action_active_upper_body: bool,
    pub horizontal: f32,
    pub vertical: f32,
    pub raw_horizontal: i32,
    pub raw_vertical: i32,
    pub last_horizontal_direction: i32,
    pub last_vertical_direction: i32,
}

impl Default for AnimatorParameters {
    fn default() -> Self {
        Self {
            action_active: false,
            action_id: 0,
            action_active_upper_body: false,
            horizontal: 0.0,
            vertical: 0.0,
            raw_horizontal: 0,
            raw_vertical: 0,
            last_horizontal_direction: 0,
            last_vertical_direction: 0,
        }
    }
}

/// Match target configuration for precise positioning during animations
#[derive(Component, Debug, Reflect, Clone)]
#[reflect(Component)]
pub struct MatchTargetConfig {
    pub enabled: bool,
    pub target_position: Vec3,
    pub target_rotation: Quat,
    pub position_weight: Vec3, // XYZ weights 0-1
    pub rotation_weight: f32,  // 0-1
    pub start_time: f32,       // normalized 0-1
    pub end_time: f32,         // normalized 0-1
    pub current_normalized_time: f32,
}

impl Default for MatchTargetConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            target_position: Vec3::ZERO,
            target_rotation: Quat::IDENTITY,
            position_weight: Vec3::ONE,
            rotation_weight: 1.0,
            start_time: 0.0,
            end_time: 1.0,
            current_normalized_time: 0.0,
        }
    }
}

/// Action category for grouping and priority management
#[derive(Debug, Clone, Reflect, PartialEq)]
pub struct ActionCategory {
    pub name: String,
    pub priority: i32,
}

impl Default for ActionCategory {
    fn default() -> Self {
        Self {
            name: "Default".to_string(),
            priority: 0,
        }
    }
}

/// Custom action information for named actions with advanced features
#[derive(Component, Debug, Reflect, Clone)]
#[reflect(Component)]
pub struct CustomActionInfo {
    pub name: String,
    pub category: ActionCategory,
    pub enabled: bool,
    pub action_system_entity: Option<Entity>,
    
    // Interruption settings
    pub can_interrupt_other_actions: bool,
    pub use_category_to_check_interrupt: bool,
    pub action_categories_to_interrupt: Vec<String>,
    pub action_names_to_interrupt: Vec<String>,
    pub can_force_interrupt: bool,
    
    // Probability
    pub use_probability: bool,
    pub probability: f32, // 0.0 - 1.0
    
    // Random actions
    pub use_random_action_list: bool,
    pub random_action_entities: Vec<Entity>,
    pub follow_actions_order: bool,
    pub current_action_index: usize,
    
    // Conditions
    pub check_locked_camera_state: bool,
    pub required_locked_camera_state: bool,
    pub check_aiming_state: bool,
    pub required_aiming_state: bool,
    pub check_on_ground: bool,
    
    // Action on air
    pub use_action_on_air: bool,
    pub action_system_on_air_entity: Option<Entity>,
}

impl Default for CustomActionInfo {
    fn default() -> Self {
        Self {
            name: String::new(),
            category: ActionCategory::default(),
            enabled: true,
            action_system_entity: None,
            can_interrupt_other_actions: false,
            use_category_to_check_interrupt: false,
            action_categories_to_interrupt: Vec::new(),
            action_names_to_interrupt: Vec::new(),
            can_force_interrupt: false,
            use_probability: false,
            probability: 1.0,
            use_random_action_list: false,
            random_action_entities: Vec::new(),
            follow_actions_order: false,
            current_action_index: 0,
            check_locked_camera_state: false,
            required_locked_camera_state: false,
            check_aiming_state: false,
            required_aiming_state: false,
            check_on_ground: false,
            use_action_on_air: false,
            action_system_on_air_entity: None,
        }
    }
}

/// Main component for an interactive action (e.g., sitting, vaulting)
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct ActionSystem {
    pub action_name: String,
    pub is_active: bool,
    pub category_name: String,
    
    // Activation Conditions
    pub use_min_distance: bool,
    pub min_distance: f32,
    pub use_min_angle: bool,
    pub min_angle: f32,
    
    // Player Adjustment
    pub use_position_to_adjust_player: bool,
    pub match_target_transform: Option<Transform>, 
    pub adjust_player_position_speed: f32,
    pub rotate_player_to_face_target: bool,

    pub duration: f32,
    pub animation_speed: f32,
    pub animation_clip: Option<Handle<AnimationClip>>,
    
    // Animation settings
    pub use_action_id: bool,
    pub action_id: i32,
    pub remove_action_id_immediately: bool,
    pub animation_used_on_upper_body: bool,
    pub disable_regular_action_active_state: bool,
    
    // Root motion
    pub use_root_motion: bool,
    pub apply_root_position: bool,
    pub apply_root_rotation: bool,
    
    // Match target
    pub use_match_target: bool,
    pub match_target_config: Option<MatchTargetConfig>,
    
    // State Overrides
    pub disable_physics: bool,
    pub disable_gravity: bool,
    pub disable_input: bool,
    
    // Walk-to-target settings
    pub use_walk_to_target_before_action: bool,
    pub use_walk_to_target_after_action: bool,
    pub walk_target_position: Option<Vec3>,
    pub walk_target_entity: Option<Entity>,
    pub max_walk_speed: f32,
    pub min_distance_to_target: f32,
    pub walk_timeout: f32,
    pub use_raycast_to_adjust_target: bool,
    pub raycast_layer_mask: u32,
    pub activate_dynamic_obstacle_detection: bool,
    pub dynamic_obstacle_distance_threshold: f32,
    
    // Event system
    pub use_event_list: bool,
    pub use_accumulative_delay: bool,
    pub event_list: Vec<ActionEvent>,
    
    // Chaining
    pub activate_custom_action_after_complete: bool,
    pub custom_action_name_after_complete: String,
    
    // Interruption
    pub can_stop_previous_action: bool,
    pub can_interrupt_other_action_active: bool,
    pub use_event_on_interrupted_action: bool,
    
    // Internal state
    pub player_detected: bool,
    
    // Player state management
    pub player_state_control: PlayerStateControl,
}

impl Default for ActionSystem {
    fn default() -> Self {
        Self {
            action_name: "Action".to_string(),
            is_active: true,
            category_name: String::new(),
            use_min_distance: true,
            min_distance: 2.0,
            use_min_angle: true,
            min_angle: 45.0,
            use_position_to_adjust_player: false,
            match_target_transform: None,
            adjust_player_position_speed: 5.0,
            rotate_player_to_face_target: true,
            duration: 1.0,
            animation_speed: 1.0,
            animation_clip: None,
            use_action_id: false,
            action_id: 0,
            remove_action_id_immediately: false,
            animation_used_on_upper_body: false,
            disable_regular_action_active_state: false,
            use_root_motion: false,
            apply_root_position: true,
            apply_root_rotation: true,
            use_match_target: false,
            match_target_config: None,
            disable_physics: true,
            disable_gravity: true,
            disable_input: true,
            use_walk_to_target_before_action: false,
            use_walk_to_target_after_action: false,
            walk_target_position: None,
            walk_target_entity: None,
            max_walk_speed: 2.0,
            min_distance_to_target: 0.5,
            walk_timeout: 10.0,
            use_raycast_to_adjust_target: false,
            raycast_layer_mask: 0,
            activate_dynamic_obstacle_detection: false,
            dynamic_obstacle_distance_threshold: 2.0,
            use_event_list: false,
            use_accumulative_delay: false,
            event_list: Vec::new(),
            activate_custom_action_after_complete: false,
            custom_action_name_after_complete: String::new(),
            can_stop_previous_action: true,
            can_interrupt_other_action_active: false,
            use_event_on_interrupted_action: false,
            player_detected: false,
            player_state_control: PlayerStateControl::default(),
        }
    }
}

/// Component on the player to manage active actions
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PlayerActionSystem {
    pub current_action: Option<Entity>,
    pub is_action_active: bool,
    
    pub state: ActionState,
    pub action_timer: f32,
    
    // Current action info
    pub current_action_category: Option<String>,
    pub action_waiting_to_resume: Option<Entity>,
    
    // Walk-to-target state
    pub walk_state: WalkToTargetState,
    pub walk_timer: f32,
    pub previous_navmesh_active: bool,
    
    // Event tracking
    pub events_active: bool,
    pub event_start_time: f32,
    
    // State backup to restore after action
    pub previous_gravity_state: bool,
    pub previous_physics_state: bool,
    pub saved_crouch_state: bool,
    pub saved_strafe_state: bool,
    
    // Saved weapon state
    pub saved_aim_state: bool,
    pub saved_weapon_index: usize,
    pub saved_change_keys: bool,
    pub saved_change_wheel: bool,
    pub saved_change_number: bool,
}

impl Default for PlayerActionSystem {
    fn default() -> Self {
        Self {
            current_action: None,
            is_action_active: false,
            state: ActionState::Idle,
            action_timer: 0.0,
            current_action_category: None,
            action_waiting_to_resume: None,
            walk_state: WalkToTargetState::Idle,
            walk_timer: 0.0,
            previous_navmesh_active: false,
            events_active: false,
            event_start_time: 0.0,
            previous_gravity_state: true,
            previous_physics_state: true,
            saved_crouch_state: false,
            saved_strafe_state: false,
            saved_aim_state: false,
            saved_weapon_index: 0,
            saved_change_keys: true,
            saved_change_wheel: true,
            saved_change_number: true,
        }
    }
}

/// Resource for managing custom actions
#[derive(Resource, Default)]
pub struct CustomActionManager {
    // Name -> Entity lookup (lowercase)
    pub action_lookup: HashMap<String, Entity>,
    
    // Category -> Actions lookup
    pub category_lookup: HashMap<String, Vec<Entity>>,
    
    // Queue of actions waiting to be played
    pub stored_action_queue: Vec<Entity>,
}

/// Event to trigger an action
#[derive(Debug, Clone, Copy, Event)]
pub struct StartActionEvent {
    pub action_entity: Entity,
    pub player_entity: Entity,
}

#[derive(Resource, Default)]
pub struct StartActionEventQueue(pub Vec<StartActionEvent>);

/// Event when an action ends
#[derive(Debug, Clone, Copy, Event)]
pub struct EndActionEvent {
    pub action_entity: Entity,
    pub player_entity: Entity,
}

#[derive(Resource, Default)]
pub struct EndActionEventQueue(pub Vec<EndActionEvent>);

/// Event to activate a custom action by name
#[derive(Debug, Clone)]
pub struct ActivateCustomActionEvent {
    pub player_entity: Entity,
    pub action_name: String,
}

#[derive(Resource, Default)]
pub struct ActivateCustomActionEventQueue(pub Vec<ActivateCustomActionEvent>);

/// Event to stop a custom action by name
#[derive(Debug, Clone)]
pub struct StopCustomActionEvent {
    pub player_entity: Entity,
    pub action_name: String,
}

#[derive(Resource, Default)]
pub struct StopCustomActionEventQueue(pub Vec<StopCustomActionEvent>);

/// Event when an action is interrupted
#[derive(Debug, Clone, Copy)]
pub struct ActionInterruptedEvent {
    pub player_entity: Entity,
    pub interrupted_action_entity: Entity,
    pub new_action_entity: Entity,
}

#[derive(Resource, Default)]
pub struct ActionInterruptedEventQueue(pub Vec<ActionInterruptedEvent>);

// ============================================================================
// Event System
// ============================================================================

/// Event triggered during action execution
#[derive(Debug, Clone)]
pub struct ActionEventTriggered {
    pub action_entity: Entity,
    pub player_entity: Entity,
    pub event_name: String,
}

#[derive(Resource, Default)]
pub struct ActionEventTriggeredQueue(pub Vec<ActionEventTriggered>);

/// Remote event for cross-system communication
#[derive(Debug, Clone)]
pub struct RemoteActionEvent {
    pub event_name: String,
    pub player_entity: Option<Entity>,
}

#[derive(Resource, Default)]
pub struct RemoteActionEventQueue(pub Vec<RemoteActionEvent>);

/// Camera event triggered during action
#[derive(Debug, Clone)]
pub struct CameraEventTriggered {
    pub event_type: CameraEventType,
    pub player_entity: Entity,
    pub action_entity: Entity,
}

#[derive(Resource, Default)]
pub struct CameraEventQueue(pub Vec<CameraEventTriggered>);

/// Physics event triggered during action
#[derive(Debug, Clone)]
pub struct PhysicsEventTriggered {
    pub event_type: PhysicsEventType,
    pub target_entity: Entity,
    pub source_entity: Entity,
}

#[derive(Resource, Default)]
pub struct PhysicsEventQueue(pub Vec<PhysicsEventTriggered>);

/// State change event triggered during action
#[derive(Debug, Clone)]
pub struct StateChangeEventTriggered {
    pub event_type: StateChangeEventType,
    pub player_entity: Entity,
}

#[derive(Resource, Default)]
pub struct StateChangeEventQueue(pub Vec<StateChangeEventTriggered>);

/// Weapon event triggered during an action
#[derive(Debug, Clone)]
pub struct WeaponEventTriggered {
    pub event_type: WeaponEventType,
    pub player_entity: Entity,
    pub target_entity: Option<Entity>,
}

/// Queue for weapon events triggered during actions
#[derive(Resource, Default)]
pub struct WeaponEventQueue(pub Vec<WeaponEventTriggered>);

/// Power event triggered during an action
#[derive(Debug, Clone)]
pub struct PowerEventTriggered {
    pub event_type: PowerEventType,
    pub player_entity: Entity,
    pub amount: f32,
}

/// Queue for power events triggered during actions
#[derive(Resource, Default)]
pub struct PowerEventQueue(pub Vec<PowerEventTriggered>);
