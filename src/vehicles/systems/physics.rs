use bevy::prelude::*;
use crate::vehicles::types::*;
use avian3d::prelude::*;

pub fn update_vehicles_physics(
    time: Res<Time>,
    mut query: Query<(&mut Vehicle, &mut LinearVelocity, &mut AngularVelocity, &Transform)>,
    spatial_query: SpatialQuery,
) {
    let delta = time.delta_secs();

    for (mut vehicle, mut velocity, mut angular_vel, transform) in query.iter_mut() {
        let forward = transform.forward();
        let right = transform.right();
        let up = transform.up();

        // Check if vehicle is on ground
        let ray_origin = transform.translation + up * 0.5;
        let ray_direction = Dir3::NEG_Y;
        let ray_distance = 1.0;

        let hit = spatial_query.cast_ray(ray_origin, ray_direction, ray_distance, false, &SpatialQueryFilter::default());
        vehicle.is_on_ground = hit.is_some();

        // Calculate current speed
        let current_forward_speed = velocity.dot(*forward);
        let current_right_speed = velocity.dot(*right);
        vehicle.current_speed = velocity.length();

        // Determine if reversing
        vehicle.is_reversing = vehicle.motor_input < 0.0 && current_forward_speed < 1.0;

        // Calculate target speed
        let target_speed = if vehicle.motor_input > 0.0 {
            vehicle.motor_input * vehicle.max_forward_speed
        } else {
            vehicle.motor_input * vehicle.max_backward_speed
        };

        // Calculate acceleration
        let speed_diff = target_speed - current_forward_speed;
        let acceleration = if vehicle.is_boosting {
            vehicle.engine_torque * vehicle.boost_multiplier * delta
        } else {
            vehicle.engine_torque * delta
        };

        // Apply motor torque
        if vehicle.is_turned_on && !vehicle.is_braking {
            let motor_torque = speed_diff.abs() * acceleration;
            velocity.0 += *forward * motor_torque * speed_diff.signum();
        }

        // Apply braking
        if vehicle.is_braking || (vehicle.is_reversing && vehicle.motor_input > 0.0) {
            let brake_force = vehicle.brake_power * delta;
            velocity.0 -= *forward * brake_force * current_forward_speed.signum();
            velocity.0 -= *right * brake_force * current_right_speed.signum();
        }

        // Calculate steering
        let steer_angle = if vehicle.current_speed > vehicle.high_speed_steering_at_speed {
            vehicle.high_speed_steering_angle
        } else {
            vehicle.steering_angle
        };

        // Apply steering torque
        let steer_effectiveness = (vehicle.current_speed / 10.0).clamp(0.0, 1.0);
        let steer_torque = -vehicle.steer_input * steer_angle.to_radians() * steer_effectiveness;

        if vehicle.is_turned_on && vehicle.is_on_ground {
            angular_vel.y += steer_torque * delta * 2.0;
        }

        // Apply jump force
        if vehicle.is_jumping && vehicle.is_on_ground {
            velocity.0 += *up * vehicle.jump_power;
            vehicle.is_jumping = false;
        }

        // Apply impulse (if holding jump)
        if vehicle.can_impulse && vehicle.is_boosting && vehicle.is_turned_on {
            velocity.0 += *up * vehicle.impulse_force * delta;
        }

        // Anti-roll system
        if vehicle.is_on_ground && vehicle.anti_roll > 0.0 {
            let roll = angular_vel.x * vehicle.anti_roll * delta;
            velocity.0 += *up * roll;
        }

        // Chassis lean
        let lean_amount = vehicle.chassis_lean.x * current_right_speed.abs() * 0.1;
        vehicle.chassis_lean_x = vehicle.chassis_lean_x.lerp(lean_amount, delta * 3.0);
        vehicle.chassis_lean_x = vehicle.chassis_lean_x.clamp(-vehicle.chassis_lean_limit, vehicle.chassis_lean_limit);

        // Preserve direction in air
        if !vehicle.is_on_ground && vehicle.preserve_direction_in_air && vehicle.current_speed > 5.0 {
            vehicle.time_to_stabilize += delta;
            if vehicle.time_to_stabilize > 0.6 {
                if velocity.length() > 0.1 {
                    let target_forward = velocity.normalize();
                    let current_forward = *forward;
                    let rotation_axis = current_forward.cross(target_forward).normalize();
                    let angle = current_forward.angle_between(target_forward);

                    if angle > 0.01 {
                        let stabilization_torque = rotation_axis * angle * 10.0 * delta;
                        angular_vel.0 += stabilization_torque;
                    }
                }
            }
        } else {
            vehicle.time_to_stabilize = 0.0;
        }

        // Apply drag
        let drag = 0.01 + (vehicle.current_speed * 0.001);
        velocity.0 *= 1.0 - drag * delta;

        // Apply angular drag
        angular_vel.0 *= 0.95;

        // Limit max speed
        if vehicle.current_speed > vehicle.max_forward_speed * vehicle.boost_multiplier {
            let excess_speed = vehicle.current_speed - vehicle.max_forward_speed * vehicle.boost_multiplier;
            let velocity_normalized = velocity.normalize();
            velocity.0 -= velocity_normalized * excess_speed * delta;
        }

        // Update RPM based on speed and gear
        let base_rpm = 1000.0;
        let max_rpm = 6000.0;
        let speed_ratio = vehicle.current_speed / vehicle.max_forward_speed;
        vehicle.current_rpm = base_rpm + (max_rpm - base_rpm) * speed_ratio;
        vehicle.current_rpm = vehicle.current_rpm.clamp(base_rpm, max_rpm);
    }
}
