use bevy::prelude::*;
use avian3d::prelude::*;

#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

/// Main character controller component
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct CharacterController {
    // Speeds
    pub walk_speed: f32,
    pub run_speed: f32,
    pub sprint_speed: f32,
    pub crouch_speed: f32,
    
    // Rotation
    pub turn_speed: f32,
    pub stationary_turn_speed: f32,
    pub moving_turn_speed: f32,
    pub use_tank_controls: bool,
    
    // Jump
    pub jump_power: f32,
    pub jump_hold_bonus: f32,
    pub max_jump_hold_time: f32,
    
    // Feedback/State
    pub can_move: bool,
    pub is_dead: bool,
    pub is_strafing: bool,
    
    // Movement Smoothing
    pub acceleration: f32,
    pub deceleration: f32,

    // Falling Damage
    pub fall_damage_enabled: bool,
    pub min_velocity_for_damage: f32,
    pub falling_damage_multiplier: f32,

    // Crouch Sliding
    pub crouch_sliding_enabled: bool,
    pub crouch_sliding_speed: f32,
    pub crouch_sliding_duration: f32,

    // Obstacle Detection
    pub obstacle_detection_distance: f32,
    
    // Axis Constraints (for 2.5D)
    pub fixed_axis: Option<Vec3>,

    // Root Motion
    pub use_root_motion: bool,

    // Environmental States
    pub zero_gravity_mode: bool,
    pub free_floating_mode: bool,
}

impl Default for CharacterController {
    fn default() -> Self {
        Self {
            walk_speed: 4.0,
            run_speed: 7.0,
            sprint_speed: 10.0,
            crouch_speed: 2.5,
            
            turn_speed: 10.0,
            stationary_turn_speed: 180.0,
            moving_turn_speed: 200.0,
            use_tank_controls: false,
            
            jump_power: 6.0,
            jump_hold_bonus: 2.0,
            max_jump_hold_time: 0.25,
            
            can_move: true,
            is_dead: false,
            is_strafing: false,
            
            acceleration: 10.0,
            deceleration: 15.0,

            fall_damage_enabled: true,
            min_velocity_for_damage: 12.0,
            falling_damage_multiplier: 5.0,

            crouch_sliding_enabled: true,
            crouch_sliding_speed: 12.0,
            crouch_sliding_duration: 1.0,

            obstacle_detection_distance: 0.5,
            
            fixed_axis: None,
            use_root_motion: false,
            zero_gravity_mode: false,
            free_floating_mode: false,
        }
    }
}

/// Character movement state
#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct CharacterMovementState {
    pub raw_move_dir: Vec3,
    pub lerped_move_dir: Vec3,
    pub is_running: bool,
    pub is_sprinting: bool,
    pub is_crouching: bool,
    pub wants_to_jump: bool,
    pub jump_held: bool,
    pub current_speed: f32,
    pub current_normal: Vec3,

    // Internal state
    pub last_vertical_velocity: f32,
    pub air_time: f32,
    pub jump_hold_timer: f32,
    pub crouch_sliding_active: bool,
    pub crouch_sliding_timer: f32,
    pub obstacle_found: bool,
    pub quick_turn_active: bool,
    pub quick_turn_timer: f32,

    // Wall Running
    pub wall_running_active: bool,
    pub wall_side: Option<Vec3>, // Normal of the wall we are running on

    // Root Motion Deltas (to be filled by animation systems)
    pub root_motion_translation: Vec3,
    pub root_motion_rotation: Quat,

    // Vehicle state
    pub is_in_vehicle: bool,
    pub vehicle_entity: Option<Entity>,

    // Slope state
    pub slope_slide_active: bool,
}

/// Character animation modes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
pub enum CharacterAnimationMode {
    #[default]
    Idle,
    Walk,
    Run,
    Sprint,
    CrouchIdle,
    CrouchWalk,
    JumpStart,
    JumpAir,
    Fall,
    Land,
}

/// Character animation state
#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct CharacterAnimationState {
    pub mode: CharacterAnimationMode,
    pub forward: f32,
    pub turn: f32,
}
