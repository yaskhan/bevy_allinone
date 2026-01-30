use bevy::prelude::*;
use crate::character::types::*;
use crate::input::InputState;
use crate::physics::{GroundDetection, GroundDetectionSettings};
use avian3d::prelude::*;
use crate::input::InputBuffer;
use crate::input::InputAction;

pub fn update_character_movement(
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

pub fn apply_character_physics(
    time: Res<Time>,
    spatial_query: SpatialQuery,
    mut input_buffer: ResMut<InputBuffer>,
    mut query: Query<(
        Entity,
        &mut CharacterMovementState, 
        &mut GroundDetection, 
        &GroundDetectionSettings,
        &mut LinearVelocity, 
        &CharacterController,
        &mut Transform,
        &InputState,
    )>,
) {
    for (entity, mut movement, mut ground, settings, mut velocity, controller, mut transform, input) in query.iter_mut() {
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

        if movement.wall_running_active {
            // Counteract gravity and maintain forward momentum
            velocity.y = 0.0;
            if let Some(normal) = movement.wall_side {
                // Stick to wall by applying slight force towards it
                velocity.0 -= normal * 2.0;
            }
        }

        if controller.zero_gravity_mode || controller.free_floating_mode {
            // In zero gravity or free float, movement is 3D
            let mut free_vel = transform.rotation * Vec3::new(input.movement.x, 0.0, -input.movement.y) * movement.current_speed;
            
            // Vertical propulsion
            if input.jump_pressed {
                free_vel.y += movement.current_speed;
            }
            if input.crouch_pressed {
                free_vel.y -= movement.current_speed;
            }

            velocity.x = free_vel.x;
            velocity.y = free_vel.y;
            velocity.z = free_vel.z;
            continue;
        }

        // --- ROOT MOTION ---
        if controller.use_root_motion {
            // Convert translation delta to velocity
            let dt = time.delta_secs();
            if dt > 0.0 {
                let rm_velocity = movement.root_motion_translation / dt;
                velocity.x = rm_velocity.x;
                velocity.z = rm_velocity.z;
                if rm_velocity.y.abs() > 0.001 {
                    velocity.y = rm_velocity.y;
                }
            }
            // Apply rotation delta
            transform.rotation *= movement.root_motion_rotation;
            
            // Reset deltas after consumption
            movement.root_motion_translation = Vec3::ZERO;
            movement.root_motion_rotation = Quat::IDENTITY;
        }

        // --- STEP UP LOGIC ---
        if move_dir.length_squared() > 0.01 && ground.is_grounded {
            let filter = SpatialQueryFilter::from_excluded_entities([entity]);
            let ray_dir = Dir3::new(move_dir.normalize()).unwrap_or(Dir3::NEG_Z);
            
            // Raycast at feet level to detect step base
            let feet_ray_pos = transform.translation + Vec3::Y * 0.05;
            let step_base_hit = spatial_query.cast_ray(feet_ray_pos, ray_dir, settings.step_check_distance, true, &filter);
            
            // Raycast at max_step_height to see if it's climbable
            let knee_ray_pos = transform.translation + Vec3::Y * (settings.max_step_height + 0.05);
            let step_top_hit = spatial_query.cast_ray(knee_ray_pos, ray_dir, settings.step_check_distance, true, &filter);

            // If we hit something at the feet but not at the knee, it's a step
            if step_base_hit.is_some() && step_top_hit.is_none() {
                // Perform a downward raycast from ahead to find the exact top of the step
                let ahead_pos = transform.translation + (*ray_dir * settings.step_check_distance) + Vec3::Y * settings.max_step_height;
                if let Some(hit) = spatial_query.cast_ray(ahead_pos, Dir3::NEG_Y, settings.max_step_height + 0.1, true, &filter.clone()) {
                    let step_height = settings.max_step_height - hit.distance;
                    if step_height > 0.01 && step_height <= settings.max_step_height {
                        // Smoothly adjust position up
                        transform.translation.y += step_height + 0.01;
                        velocity.y = 0.0; // Prevent upward bounce
                    }
                }
            }
        }

        // --- STEP DOWN (SNAPPING) LOGIC ---
        // If we were grounded last frame, but not this frame, and not jumping
        if !ground.is_grounded && ground.last_is_grounded && !movement.wants_to_jump && velocity.y <= 0.0 {
            let filter = SpatialQueryFilter::from_excluded_entities([entity]);
            // Search for ground slightly ahead and below
            let snap_pos = transform.translation;
            if let Some(snap_hit) = spatial_query.cast_ray(snap_pos, Dir3::NEG_Y, settings.max_step_height + settings.ray_length + 0.1, true, &filter) {
                // If ground is within snapping distance
                if snap_hit.distance <= settings.max_step_height + settings.ray_length + 0.05 {
                    transform.translation.y -= snap_hit.distance - settings.ray_length;
                    velocity.y = 0.0;
                    ground.is_grounded = true; // Force grounded state
                }
            }
        }

        // Jump logic with buffering
        let jump_requested = movement.wants_to_jump || input_buffer.consume(InputAction::Jump);
        
        if jump_requested && ground.is_grounded {
            // Apply jump impulse directly to velocity
            velocity.y = controller.jump_power;
            movement.jump_hold_timer = controller.max_jump_hold_time;
            movement.wants_to_jump = false;
        }

        // Variable Jump Bonus
        if movement.jump_held && movement.jump_hold_timer > 0.0 && !ground.is_grounded {
            movement.jump_hold_timer -= time.delta_secs();
            // Apply hover acceleration directly
            let accel = controller.jump_hold_bonus * 100.0 * time.delta_secs();
            velocity.y += accel;
        }

        // Axis Constraints (2.5D)
        if let Some(axis) = controller.fixed_axis {
            let offset = transform.translation - axis;
            transform.translation -= offset * Vec3::new(1.0, 0.0, 1.0); // Simple projection
            velocity.x *= 1.0 - axis.x.abs();
            velocity.z *= 1.0 - axis.z.abs();
        }
    }
}

pub fn handle_crouch_sliding(
    time: Res<Time>,
    mut query: Query<(&CharacterController, &mut CharacterMovementState)>,
) {
    for (_controller, mut state) in query.iter_mut() {
        if state.crouch_sliding_active {
            state.crouch_sliding_timer -= time.delta_secs();
            if state.crouch_sliding_timer <= 0.0 || !state.is_crouching {
                state.crouch_sliding_active = false;
            }
        }
    }
}

pub fn update_friction_material(
    mut query: Query<(&CharacterMovementState, &mut Friction)>,
) {
    for (state, mut friction) in query.iter_mut() {
        if state.raw_move_dir.length_squared() < 0.01 {
            friction.static_coefficient = 1.0;
            friction.dynamic_coefficient = 1.0;
        } else {
            friction.static_coefficient = 0.0;
            friction.dynamic_coefficient = 0.0;
        }
    }
}
