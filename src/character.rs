use bevy::prelude::*;
use avian3d::prelude::*;
use crate::physics::{GroundDetection, CustomGravity};
use crate::input::InputState;
use crate::combat::{DamageEvent, DamageType};

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
                handle_falling_damage,
                handle_crouch_sliding,
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
    pub use_tank_controls: bool,
    
    // Jump
    pub jump_power: f32,
    
    // Feedback/State
    pub can_move: bool,
    pub is_dead: bool,
    pub is_strafing: bool,
    
    // Movement Smoothing
    pub input_horizontal_lerp_speed: f32,
    pub input_vertical_lerp_speed: f32,

    // Falling Damage
    pub fall_damage_enabled: bool,
    pub min_velocity_for_damage: f32,
    pub falling_damage_multiplier: f32,

    // Crouch Sliding
    pub crouch_sliding_enabled: bool,
    pub crouch_sliding_speed: f32,
    pub crouch_sliding_duration: f32,
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
            
            can_move: true,
            is_dead: false,
            is_strafing: false,
            
            input_horizontal_lerp_speed: 10.0,
            input_vertical_lerp_speed: 10.0,

            fall_damage_enabled: true,
            min_velocity_for_damage: 12.0,
            falling_damage_multiplier: 5.0,

            crouch_sliding_enabled: true,
            crouch_sliding_speed: 12.0,
            crouch_sliding_duration: 1.0,
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

    // Internal state for advanced features
    pub last_vertical_velocity: f32,
    pub crouch_sliding_active: bool,
    pub crouch_sliding_timer: f32,
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
        
        // Smooth input transition (Lerping)
        let target_dir = state.raw_move_dir;
        state.lerped_move_dir.x = state.lerped_move_dir.x + (target_dir.x - state.lerped_move_dir.x) * controller.input_horizontal_lerp_speed * time.delta_secs();
        state.lerped_move_dir.z = state.lerped_move_dir.z + (target_dir.z - state.lerped_move_dir.z) * controller.input_vertical_lerp_speed * time.delta_secs();

        state.is_running = true; 
        state.is_sprinting = input.sprint_pressed;
        
        // Crouch input triggers sliding if sprinting
        if input.crouch_pressed && !state.is_crouching && state.is_sprinting && controller.crouch_sliding_enabled {
            state.crouch_sliding_active = true;
            state.crouch_sliding_timer = controller.crouch_sliding_duration;
        }
        
        state.is_crouching = input.crouch_pressed;
        state.wants_to_jump = input.jump_pressed;
    }
}

fn update_character_movement(
    mut query: Query<(&CharacterController, &mut CharacterMovementState)>,
) {
    for (controller, mut state) in query.iter_mut() {
        let mut base_speed = if state.is_crouching {
            controller.crouch_speed
        } else if state.is_sprinting {
            controller.sprint_speed
        } else {
            controller.run_speed
        };

        // Crouch Sliding speed overwrite
        if state.crouch_sliding_active {
            base_speed = controller.crouch_sliding_speed;
        }

        state.current_speed = base_speed;
    }
}

fn update_character_rotation(
    time: Res<Time>,
    mut query: Query<(Entity, &CharacterController, &CharacterMovementState, &mut Transform)>,
) {
    for (_entity, controller, state, mut transform) in query.iter_mut() {
        if state.lerped_move_dir.length_squared() > 0.001 {
            // Tank Controls
            if controller.use_tank_controls {
                let rotation = Quat::from_rotation_y(-state.lerped_move_dir.x * controller.stationary_turn_speed.to_radians() * time.delta_secs());
                transform.rotation *= rotation;
            } else if controller.is_strafing {
                // Strafe mode handled elsewhere or with camera alignment
            } else {
                // Free movement rotation
                let target_rotation = Quat::from_rotation_arc(Vec3::NEG_Z, state.lerped_move_dir.normalize());
                transform.rotation = transform.rotation.slerp(target_rotation, controller.turn_speed * time.delta_secs());
            }
        }
        
        // Surface Alignment
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
        anim.turn = 0.0; 
    }
}

fn check_ground_state(
    mut query: Query<(&GroundDetection, &mut CharacterMovementState)>,
) {
    for (detection, mut state) in query.iter_mut() {
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
        // Horizontal movement
        let move_dir = if controller.use_tank_controls {
             // In tank controls, move dir is forward/backward
             Vec3::new(0.0, 0.0, movement.lerped_move_dir.z)
        } else {
            movement.lerped_move_dir
        };

        let target_vel = move_dir * movement.current_speed;
        velocity.x = target_vel.x;
        velocity.z = target_vel.z;

        // Jump logic
        if movement.wants_to_jump && ground.is_grounded {
            impulse.apply_impulse(Vec3::Y * controller.jump_power);
        }
    }
}

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

/// Falling Damage System
fn handle_falling_damage(
    mut damage_events: EventWriter<DamageEvent>,
    mut query: Query<(Entity, &CharacterController, &mut CharacterMovementState, &LinearVelocity, &GroundDetection)>,
) {
    for (entity, controller, mut state, velocity, ground) in query.iter_mut() {
        if !controller.fall_damage_enabled { continue; }

        if !ground.is_grounded {
            // Track negative vertical velocity (downward)
            state.last_vertical_velocity = velocity.y;
        } else if state.last_vertical_velocity < -controller.min_velocity_for_damage {
            // Landed after a hard fall
            let impact_speed = state.last_vertical_velocity.abs();
            let damage = (impact_speed - controller.min_velocity_for_damage) * controller.falling_damage_multiplier;
            
            damage_events.send(DamageEvent {
                target: entity,
                amount: damage,
                damage_type: DamageType::Fall,
                source: None,
            });
            
            // Reset
            state.last_vertical_velocity = 0.0;
        } else {
            state.last_vertical_velocity = 0.0;
        }
    }
}

/// Crouch Sliding Logic
fn handle_crouch_sliding(
    time: Res<Time>,
    mut query: Query<(&CharacterController, &mut CharacterMovementState)>,
) {
    for (controller, mut state) in query.iter_mut() {
        if state.crouch_sliding_active {
            state.crouch_sliding_timer -= time.delta_secs();
            if state.crouch_sliding_timer <= 0.0 || !state.is_crouching {
                state.crouch_sliding_active = false;
            }
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
        crate::combat::Health::default(),
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
