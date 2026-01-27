//! Character controller module
//!
//! Provides 3rd/1st person character control with full body awareness.

use bevy::prelude::*;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                update_character_movement,
                update_character_rotation,
                update_character_animation,
                handle_character_input,
            ).chain())
            .add_systems(FixedUpdate, (
                apply_character_physics,
                check_ground_state,
            ));
    }
}

/// Main character controller component
///
/// This is the core component for player and AI-controlled characters.
/// Handles movement, rotation, physics, and state management.
#[derive(Component, Debug)]
pub struct CharacterController {
    // Movement parameters
    pub walk_speed: f32,
    pub run_speed: f32,
    pub sprint_speed: f32,
    pub crouch_speed: f32,
    
    // Rotation parameters
    pub stationary_turn_speed: f32,
    pub moving_turn_speed: f32,
    pub aim_turn_speed: f32,
    
    // Jump parameters
    pub jump_power: f32,
    pub air_speed: f32,
    pub air_control: f32,
    pub enable_double_jump: bool,
    pub max_jumps_in_air: u32,
    
    // Gravity parameters
    pub gravity_multiplier: f32,
    pub gravity_force: f32,
    
    // State flags
    pub can_move: bool,
    pub is_grounded: bool,
    pub is_crouching: bool,
    pub is_running: bool,
    pub is_sprinting: bool,
    pub is_aiming: bool,
    pub is_dead: bool,
    
    // TODO: Add more fields from playerController.cs
}

impl Default for CharacterController {
    fn default() -> Self {
        Self {
            walk_speed: 1.0,
            run_speed: 2.0,
            sprint_speed: 3.0,
            crouch_speed: 0.5,
            
            stationary_turn_speed: 180.0,
            moving_turn_speed: 200.0,
            aim_turn_speed: 10.0,
            
            jump_power: 12.0,
            air_speed: 6.0,
            air_control: 2.0,
            enable_double_jump: true,
            max_jumps_in_air: 2,
            
            gravity_multiplier: 2.0,
            gravity_force: -9.8,
            
            can_move: true,
            is_grounded: false,
            is_crouching: false,
            is_running: false,
            is_sprinting: false,
            is_aiming: false,
            is_dead: false,
        }
    }
}

/// Character movement state
///
/// Tracks the current movement mode and input
#[derive(Component, Debug, Default)]
pub struct CharacterMovementState {
    pub move_input: Vec3,
    pub velocity: Vec3,
    pub current_speed: f32,
    pub jumps_performed: u32,
    
    // TODO: Add more state tracking
}

/// Character animation state
///
/// Manages animation parameters and transitions
#[derive(Component, Debug, Default)]
pub struct CharacterAnimationState {
    pub forward_amount: f32,
    pub turn_amount: f32,
    pub is_moving: bool,
    pub movement_speed: f32,
    
    // TODO: Add animator integration
}

/// First-person/Third-person mode
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum CameraMode {
    FirstPerson,
    ThirdPerson,
}

impl Default for CameraMode {
    fn default() -> Self {
        Self::ThirdPerson
    }
}

/// Character camera mode component
#[derive(Component, Debug, Default)]
pub struct CharacterCameraMode {
    pub mode: CameraMode,
    pub first_person_active: bool,
}

// ============================================================================
// SYSTEMS
// ============================================================================

/// Update character movement based on input
///
/// TODO: Implement movement logic
fn update_character_movement(
    time: Res<Time>,
    mut query: Query<(
        &CharacterController,
        &mut CharacterMovementState,
        &Transform,
    )>,
) {
    for (controller, mut movement, transform) in query.iter_mut() {
        if !controller.can_move || controller.is_dead {
            continue;
        }
        
        // TODO: Implement movement calculation
        // TODO: Handle walk/run/sprint transitions
        // TODO: Apply movement speed modifiers
        // TODO: Handle strafe movement
        // TODO: Handle locked camera movement (2.5D mode)
        
        let _delta = time.delta_secs();
        let _forward = transform.forward();
        let _right = transform.right();
        
        // Placeholder
        movement.current_speed = controller.walk_speed;
    }
}

/// Update character rotation
///
/// TODO: Implement rotation logic
fn update_character_rotation(
    time: Res<Time>,
    mut query: Query<(
        &CharacterController,
        &CharacterMovementState,
        &mut Transform,
    )>,
) {
    for (controller, movement, mut transform) in query.iter_mut() {
        if !controller.can_move || controller.is_dead {
            continue;
        }
        
        // TODO: Implement rotation towards movement direction
        // TODO: Handle aim rotation
        // TODO: Handle tank controls
        // TODO: Handle look in camera direction
        
        let _delta = time.delta_secs();
        let _move_dir = movement.move_input;
    }
}

/// Update character animation state
///
/// TODO: Implement animation logic
fn update_character_animation(
    mut query: Query<(
        &CharacterController,
        &CharacterMovementState,
        &mut CharacterAnimationState,
    )>,
) {
    for (controller, movement, mut anim_state) in query.iter_mut() {
        // TODO: Calculate forward and turn amounts
        // TODO: Update animator parameters
        // TODO: Handle animation speed multipliers
        // TODO: Handle root motion
        
        anim_state.is_moving = movement.move_input.length() > 0.01;
        anim_state.movement_speed = movement.current_speed;
    }
}

/// Handle character input
///
/// TODO: Integrate with input system
fn handle_character_input(
    mut query: Query<(
        &CharacterController,
        &mut CharacterMovementState,
    )>,
) {
    for (_controller, mut movement) in query.iter_mut() {
        // TODO: Read input from input system
        // TODO: Process jump input
        // TODO: Process crouch input
        // TODO: Process sprint input
        // TODO: Process aim input
        
        // Placeholder
        movement.move_input = Vec3::ZERO;
    }
}

/// Apply physics to character
///
/// TODO: Implement physics logic
fn apply_character_physics(
    time: Res<Time>,
    mut query: Query<(
        &CharacterController,
        &mut CharacterMovementState,
        &mut Transform,
    )>,
) {
    for (controller, mut movement, mut transform) in query.iter_mut() {
        // TODO: Apply gravity
        // TODO: Apply air control
        // TODO: Apply ground adherence
        // TODO: Handle slopes and stairs
        // TODO: Apply external forces
        
        let delta = time.delta_secs();
        
        if !controller.is_grounded {
            // Apply gravity
            let gravity = Vec3::Y * controller.gravity_force * controller.gravity_multiplier * delta;
            movement.velocity += gravity;
        }
        
        // Apply velocity to transform
        transform.translation += movement.velocity * delta;
    }
}

/// Check if character is on ground
///
/// TODO: Implement ground detection
fn check_ground_state(
    mut query: Query<(
        &mut CharacterController,
        &Transform,
    )>,
) {
    for (mut controller, _transform) in query.iter_mut() {
        // TODO: Raycast down to check ground
        // TODO: Check surface angle
        // TODO: Detect slopes and stairs
        // TODO: Handle stair adherence system
        
        // Placeholder
        controller.is_grounded = false;
    }
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Spawn a character controller entity
///
/// TODO: Add more configuration options
pub fn spawn_character(
    commands: &mut Commands,
    position: Vec3,
) -> Entity {
    commands.spawn((
        CharacterController::default(),
        CharacterMovementState::default(),
        CharacterAnimationState::default(),
        CharacterCameraMode::default(),
        Transform::from_translation(position),
        GlobalTransform::default(),
        Visibility::default(),
        InheritedVisibility::default(),
        ViewVisibility::default(),
    ))
    .id()
}
