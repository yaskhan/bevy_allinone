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
            ));
    }
}

/// Main character controller component
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct CharacterController {
    pub walk_speed: f32,
    pub run_speed: f32,
    pub sprint_speed: f32,
    pub crouch_speed: f32,
    
    pub turn_speed: f32,
    pub jump_power: f32,
    
    pub can_move: bool,
    pub is_dead: bool,
}

impl Default for CharacterController {
    fn default() -> Self {
        Self {
            walk_speed: 4.0,
            run_speed: 7.0,
            sprint_speed: 10.0,
            crouch_speed: 2.5,
            
            turn_speed: 10.0,
            jump_power: 6.0,
            
            can_move: true,
            is_dead: false,
        }
    }
}

/// Character movement state
#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct CharacterMovementState {
    pub move_dir: Vec3,
    pub is_running: bool,
    pub is_sprinting: bool,
    pub is_crouching: bool,
    pub wants_to_jump: bool,
    pub current_speed: f32,
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
    mut query: Query<(&CharacterController, &mut CharacterMovementState)>,
) {
    for (controller, mut state) in query.iter_mut() {
        if !controller.can_move || controller.is_dead {
            state.move_dir = Vec3::ZERO;
            continue;
        }

        state.move_dir = Vec3::new(input.movement.x, 0.0, -input.movement.y);
        state.is_running = true; // Default to run
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
    mut query: Query<(&CharacterController, &CharacterMovementState, &mut Transform)>,
) {
    for (controller, state, mut transform) in query.iter_mut() {
        if state.move_dir.length_squared() > 0.001 {
            let target_rotation = Quat::from_rotation_arc(Vec3::NEG_Z, state.move_dir.normalize());
            transform.rotation = transform.rotation.slerp(target_rotation, controller.turn_speed * time.delta_secs());
        }
    }
}

fn update_character_animation(
    mut query: Query<(&CharacterMovementState, &mut CharacterAnimationState)>,
) {
    for (movement, mut anim) in query.iter_mut() {
        anim.forward = movement.move_dir.length() * movement.current_speed;
        // Simple turn logic for now
        anim.turn = 0.0; 
    }
}

fn check_ground_state(
    mut query: Query<(&GroundDetection, &mut CharacterMovementState)>,
) {
    for (_detection, _state) in query.iter_mut() {
        // react to it here if needed for physics behavior
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
        // Horizontal movement
        let target_vel = movement.move_dir * movement.current_speed;
        velocity.x = target_vel.x;
        velocity.z = target_vel.z;

        // Jump logic
        if movement.wants_to_jump && ground.is_grounded {
            impulse.apply_impulse(Vec3::Y * controller.jump_power);
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
