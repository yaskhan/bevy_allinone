use bevy::prelude::*;
use avian3d::prelude::*;
use crate::physics::{GroundDetection, CustomGravity};
use crate::input::{InputState, InputAction, InputBuffer};
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
                handle_obstacle_detection,
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
        let move_dir = Vec3::new(input.movement.x, 0.0, -input.movement.y);
        state.raw_move_dir = move_dir;
        
        // Smooth input transition with separate Accel/Decel
        let lerp_speed = if move_dir.length_squared() > 0.01 {
            controller.acceleration
        } else {
            controller.deceleration
        };

        state.lerped_move_dir = state.lerped_move_dir.lerp(move_dir, lerp_speed * time.delta_secs());

        state.is_running = true; 
        state.is_sprinting = input.sprint_pressed;
        
        // Crouch sliding check
        if input.crouch_pressed && !state.is_crouching && state.is_sprinting && controller.crouch_sliding_enabled {
            state.crouch_sliding_active = true;
            state.crouch_sliding_timer = controller.crouch_sliding_duration;
        }
        
        state.is_crouching = input.crouch_pressed;
        state.wants_to_jump = input.jump_pressed;
        state.jump_held = input.jump_pressed; // Simple hold tracking
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

        if state.crouch_sliding_active {
            base_speed = controller.crouch_sliding_speed;
        }

        state.current_speed = base_speed;
    }
}

fn update_character_rotation(
    time: Res<Time>,
    mut query: Query<(Entity, &CharacterController, &mut CharacterMovementState, &mut Transform)>,
) {
    for (_entity, controller, mut state, mut transform) in query.iter_mut() {
        if state.lerped_move_dir.length_squared() > 0.001 {
            if controller.use_tank_controls {
                let rotation = Quat::from_rotation_y(-state.lerped_move_dir.x * controller.stationary_turn_speed.to_radians() * time.delta_secs());
                transform.rotation *= rotation;
            } else if controller.is_strafing {
                // Strafe mode
            } else {
                // Quick Turn Logic
                if !state.quick_turn_active && state.lerped_move_dir.dot(transform.forward().into()) < -0.8 {
                    state.quick_turn_active = true;
                    state.quick_turn_timer = 0.15;
                }

                if state.quick_turn_active {
                    state.quick_turn_timer -= time.delta_secs();
                    if state.quick_turn_timer <= 0.0 {
                        state.quick_turn_active = false;
                    }
                    // Snap or fast slerp for quick turn
                    let target_rotation = Quat::from_rotation_arc(Vec3::NEG_Z, state.lerped_move_dir.normalize());
                    transform.rotation = transform.rotation.slerp(target_rotation, 20.0 * time.delta_secs());
                } else {
                    let target_rotation = Quat::from_rotation_arc(Vec3::NEG_Z, state.lerped_move_dir.normalize());
                    transform.rotation = transform.rotation.slerp(target_rotation, controller.turn_speed * time.delta_secs());
                }
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
            state.air_time = 0.0;
        } else {
            state.current_normal = Vec3::Y;
        }
    }
}

fn apply_character_physics(
    time: Res<Time>,
    mut input_buffer: ResMut<InputBuffer>,
    mut query: Query<(
        &mut CharacterMovementState, 
        &GroundDetection, 
        &mut LinearVelocity, 
        &mut ExternalImpulse,
        &mut ExternalForce,
        &CharacterController,
        &mut Transform,
    )>,
) {
    for (mut movement, ground, mut velocity, mut impulse, mut force, controller, mut transform) in query.iter_mut() {
        // Horizontal movement
        let move_dir = if controller.use_tank_controls {
             Vec3::new(0.0, 0.0, movement.lerped_move_dir.z)
        } else {
            movement.lerped_move_dir
        };

        // Obstacle detection affects movement
        let final_move_dir = if movement.obstacle_found { Vec3::ZERO } else { move_dir };
        let target_vel = final_move_dir * movement.current_speed;
        
        velocity.x = target_vel.x;
        velocity.z = target_vel.z;

        // Jump logic with buffering
        let jump_requested = movement.wants_to_jump || input_buffer.consume(InputAction::Jump);
        
        if jump_requested && ground.is_grounded {
            impulse.apply_impulse(Vec3::Y * controller.jump_power);
            movement.jump_hold_timer = controller.max_jump_hold_time;
            movement.wants_to_jump = false;
        }

        // Variable Jump Bonus
        if movement.jump_held && movement.jump_hold_timer > 0.0 && !ground.is_grounded {
            movement.jump_hold_timer -= time.delta_secs();
            force.apply_force(Vec3::Y * controller.jump_hold_bonus * 100.0);
        }

        // Axis Constraints (2.5D)
        if let Some(axis) = controller.fixed_axis {
            let offset = transform.translation - axis;
            transform.translation -= offset * Vec3::new(1.0, 0.0, 1.0); // Simple projection
            velocity.x *= (1.0 - axis.x.abs());
            velocity.z *= (1.0 - axis.z.abs());
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

fn handle_falling_damage(
    time: Res<Time>,
    mut damage_events: EventWriter<DamageEvent>,
    mut query: Query<(Entity, &CharacterController, &mut CharacterMovementState, &LinearVelocity, &GroundDetection)>,
) {
    for (entity, controller, mut state, velocity, ground) in query.iter_mut() {
        if !controller.fall_damage_enabled { continue; }

        if !ground.is_grounded {
            state.last_vertical_velocity = velocity.y;
            state.air_time += time.delta_secs();
        } else if state.last_vertical_velocity < -controller.min_velocity_for_damage {
            let impact_speed = state.last_vertical_velocity.abs();
            // Damage formula: (impact + duration) * multiplier
            let damage = (impact_speed - controller.min_velocity_for_damage + state.air_time * 2.0) * controller.falling_damage_multiplier;
            
            damage_events.send(DamageEvent {
                target: entity,
                amount: damage,
                damage_type: DamageType::Fall,
                source: None,
            });
            
            state.last_vertical_velocity = 0.0;
            state.air_time = 0.0;
        } else {
            state.last_vertical_velocity = 0.0;
            state.air_time = 0.0;
        }
    }
}

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

fn handle_obstacle_detection(
    spatial_query: SpatialQuery,
    mut query: Query<(Entity, &Transform, &CharacterController, &mut CharacterMovementState)>,
) {
    for (entity, transform, controller, mut state) in query.iter_mut() {
        if state.raw_move_dir.length_squared() < 0.01 {
            state.obstacle_found = false;
            continue;
        }

        let ray_pos = transform.translation + Vec3::Y * 0.5;
        let ray_dir = Dir3::new(state.raw_move_dir.normalize()).unwrap_or(Dir3::NEG_Z);
        let filter = SpatialQueryFilter::from_excluded_entities([entity]);

        // Dual raycasts for feet level
        let left_ray = ray_pos + transform.left() * 0.3;
        let right_ray = ray_pos + transform.right() * 0.3;

        let hit_left = spatial_query.cast_ray(left_ray, ray_dir, controller.obstacle_detection_distance, true, filter.clone());
        let hit_right = spatial_query.cast_ray(right_ray, ray_dir, controller.obstacle_detection_distance, true, filter);

        state.obstacle_found = hit_left.is_some() || hit_right.is_some();
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
