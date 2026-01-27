use bevy::prelude::*;
use avian3d::prelude::*;
use crate::physics::{GroundDetection, CustomGravity};
use crate::input::InputState;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                handle_character_input,
                update_character_movement,
                update_character_rotation,
                update_character_animation,
            ).chain())
            .add_systems(FixedUpdate, (
                apply_character_physics,
                check_ground_state,
                update_friction_material,
            ));
    }
}

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
    
    // Jump
    pub jump_power: f32,
    
    // Feedback/State
    pub can_move: bool,
    pub is_dead: bool,
    pub is_strafing: bool,
    
    // GKC Alignment: Input Smoothing
    pub input_horizontal_lerp_speed: f32,
    pub input_vertical_lerp_speed: f32,
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
            
            jump_power: 6.0,
            
            can_move: true,
            is_dead: false,
            is_strafing: false,
            
            input_horizontal_lerp_speed: 10.0,
            input_vertical_lerp_speed: 10.0,
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
    pub current_speed: f32,
    pub current_normal: Vec3,
}

/// Character animation state
#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct CharacterAnimationState {
    pub forward: f32,
    pub turn: f32,
}

// ============================================================================
// SYSTEMS
// ============================================================================

fn handle_character_input(
    input: Res<InputState>,
    time: Res<Time>,
    mut query: Query<(&CharacterController, &mut CharacterMovementState)>,
) {
    for (controller, mut state) in query.iter_mut() {
        if !controller.can_move || controller.is_dead {
            state.raw_move_dir = Vec3::ZERO;
            state.lerped_move_dir = Vec3::ZERO;
            continue;
        }

        // Horizontal input mapping
        state.raw_move_dir = Vec3::new(input.movement.x, 0.0, -input.movement.y);
        
        // GKC Alignment: Smooth input transition (Lerping)
        let target_dir = state.raw_move_dir;
        state.lerped_move_dir.x = state.lerped_move_dir.x + (target_dir.x - state.lerped_move_dir.x) * controller.input_horizontal_lerp_speed * time.delta_secs();
        state.lerped_move_dir.z = state.lerped_move_dir.z + (target_dir.z - state.lerped_move_dir.z) * controller.input_vertical_lerp_speed * time.delta_secs();

        state.is_running = true; 
        state.is_sprinting = input.sprint_pressed;
        state.is_crouching = input.crouch_pressed;
        state.wants_to_jump = input.jump_pressed;
    }
}

fn update_character_movement(
    mut query: Query<(&CharacterController, &mut CharacterMovementState)>,
) {
    for (controller, mut state) in query.iter_mut() {
        let base_speed = if state.is_crouching {
            controller.crouch_speed
        } else if state.is_sprinting {
            controller.sprint_speed
        } else {
            controller.run_speed
        };

        state.current_speed = base_speed;
    }
}

fn update_character_rotation(
    time: Res<Time>,
    mut query: Query<(Entity, &CharacterController, &CharacterMovementState, &mut Transform)>,
    // Assuming we might need camera look direction for strafing
) {
    for (_entity, controller, state, mut transform) in query.iter_mut() {
        if state.lerped_move_dir.length_squared() > 0.001 {
            // GKC Alignment: Strafe Mode Rotation
            if controller.is_strafing {
                // TODO: Rotate to face camera direction
                // For now, keep facing forward but allow side movement logic
            } else {
                // Free movement rotation
                let target_rotation = Quat::from_rotation_arc(Vec3::NEG_Z, state.lerped_move_dir.normalize());
                transform.rotation = transform.rotation.slerp(target_rotation, controller.turn_speed * time.delta_secs());
            }
        }
        
        // GKC Alignment: Surface Alignment (Rotating Up to Normal)
        if state.current_normal.length_squared() > 0.0 {
            let target_up = state.current_normal;
            let current_up = transform.up();
            let rotation_to_align = Quat::from_rotation_arc(current_up, target_up);
            transform.rotation = rotation_to_align * transform.rotation;
        }
    }
}

fn update_character_animation(
    mut query: Query<(&CharacterMovementState, &mut CharacterAnimationState)>,
) {
    for (movement, mut anim) in query.iter_mut() {
        anim.forward = movement.lerped_move_dir.length() * movement.current_speed;
        // Turn amount calculation for animation blending
        anim.turn = 0.0; 
    }
}

fn check_ground_state(
    mut query: Query<(&GroundDetection, &mut CharacterMovementState)>,
) {
    for (detection, mut state) in query.iter_mut() {
        // GKC Alignment: Track current surface normal
        if detection.is_grounded {
            state.current_normal = detection.ground_normal;
        } else {
            state.current_normal = Vec3::Y;
        }
    }
}

fn apply_character_physics(
    mut query: Query<(
        &CharacterMovementState, 
        &GroundDetection, 
        &mut LinearVelocity, 
        &mut ExternalImpulse,
        &CharacterController
    )>,
) {
    for (movement, ground, mut velocity, mut impulse, controller) in query.iter_mut() {
        // Horizontal movement using lerped input
        let target_vel = movement.lerped_move_dir * movement.current_speed;
        velocity.x = target_vel.x;
        velocity.z = target_vel.z;

        // Jump logic
        if movement.wants_to_jump && ground.is_grounded {
            impulse.apply_impulse(Vec3::Y * controller.jump_power);
        }
    }
}

/// GKC Alignment: Dynamic friction material management
fn update_friction_material(
    mut query: Query<(&CharacterMovementState, &mut Friction)>,
) {
    for (state, mut friction) in query.iter_mut() {
        if state.raw_move_dir.length_squared() < 0.01 {
            friction.combined_static_coefficient = 1.0;
            friction.combined_dynamic_coefficient = 1.0;
        } else {
            friction.combined_static_coefficient = 0.1;
            friction.combined_dynamic_coefficient = 0.1;
        }
    }
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

pub fn spawn_character(
    commands: &mut Commands,
    position: Vec3,
) -> Entity {
    commands.spawn((
        Name::new("Player"),
        CharacterController::default(),
        CharacterMovementState::default(),
        CharacterAnimationState::default(),
        Transform::from_translation(position),
        GlobalTransform::default(),
        // Physics
        RigidBody::Dynamic,
        Collider::capsule(0.4, 1.0),
        LockedAxes::ROTATION_LOCKED,
        LinearVelocity::ZERO,
        ExternalForce::ZERO,
        ExternalImpulse::ZERO,
        Friction::new(1.0),
        Restitution::new(0.0),
        CustomGravity::default(),
        GroundDetection::default(),
        crate::physics::GroundDetectionSettings::default(),
        // Visibility
        Visibility::default(),
        InheritedVisibility::default(),
        ViewVisibility::default(),
    ))
    .id()
}
