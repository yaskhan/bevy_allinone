use bevy::prelude::*;
use crate::vehicles::types::*;
use avian3d::prelude::*;

pub fn update_vehicles_physics(
    time: Res<Time>,
    mut query: Query<(Entity, &mut Vehicle, &mut LinearVelocity, &mut AngularVelocity, &Transform, &Children)>,
    wheel_query: Query<&VehicleWheel>,
    spatial_query: SpatialQuery,
) {
    let delta = time.delta_secs();

    for (entity, mut vehicle, mut velocity, mut angular_vel, transform, children) in query.iter_mut() {
        let forward = transform.forward();
        let right = transform.right();
        let up = transform.up();

        // Check if any wheel is on ground
        let mut any_on_ground = false;
        let mut total_travel_left = 0.0;
        let mut total_travel_right = 0.0;
        let mut powered_wheels_count = 0;
        let mut total_rpm = 0.0;

        for child in children.iter() {
            if let Ok(wheel) = wheel_query.get(child.clone()) {
                if wheel.suspension_spring_pos > -wheel.suspension_distance {
                    any_on_ground = true;
                }
                
                let travel = (wheel.suspension_spring_pos + wheel.suspension_distance) / wheel.suspension_distance;
                if wheel.is_left_side { total_travel_left += travel; }
                if wheel.is_right_side { total_travel_right += travel; }

                if wheel.is_powered {
                    total_rpm += wheel.current_rpm;
                    powered_wheels_count += 1;
                }
            }
        }
        vehicle.is_on_ground = any_on_ground;

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
        if vehicle.is_turned_on && !vehicle.is_braking && !vehicle.changing_gear {
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
            // Only apply standard steering if it's a ground vehicle
            if matches!(vehicle.vehicle_type, VehicleType::Car | VehicleType::Truck | VehicleType::Motorcycle | VehicleType::Hovercraft) {
                angular_vel.y += steer_torque * delta * 2.0;
                // Extra torque for better handling
                angular_vel.y += -vehicle.steer_input * 2.0 * delta;
            }
        }

        // Apply jump force
        if vehicle.is_jumping && vehicle.is_on_ground {
            velocity.0 += *up * vehicle.jump_power;
            vehicle.is_jumping = false;
        }

        // Anti-roll system (Advanced)
        if vehicle.is_on_ground && vehicle.anti_roll > 0.0 {
            let travel_diff = total_travel_left - total_travel_right;
            let anti_roll_force = travel_diff * vehicle.anti_roll * delta;
            angular_vel.x += anti_roll_force * 0.1; // Apply roll torque
            velocity.0 += *up * anti_roll_force; // Apply vertical stability
        }

        // Chassis lean (Visual sync with physics)
        let lean_amount = vehicle.chassis_lean.x * current_right_speed * 0.05;
        vehicle.chassis_lean_x = vehicle.chassis_lean_x.lerp(lean_amount, delta * 3.0);
        vehicle.chassis_lean_x = vehicle.chassis_lean_x.clamp(-vehicle.chassis_lean_limit, vehicle.chassis_lean_limit);

        // Preserve direction in air (Advanced)
        if !vehicle.is_on_ground && vehicle.preserve_direction_in_air && vehicle.current_speed > 5.0 {
            vehicle.time_to_stabilize += delta;
            if vehicle.time_to_stabilize > 0.6 {
                if velocity.length() > 0.1 {
                    let target_forward = velocity.normalize();
                    let current_forward = *forward;
                    let cross = current_forward.cross(target_forward);
                    
                    if cross.length_squared() > 0.0001 {
                        let rotation_axis = cross.normalize();
                        let angle = current_forward.angle_between(target_forward);
                        let stabilization_torque = rotation_axis * angle * 5.0 * delta;
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
        let max_speed = vehicle.max_forward_speed * vehicle.boost_input;
        if vehicle.current_speed > max_speed {
            let excess_speed = vehicle.current_speed - max_speed;
            let velocity_normalized = velocity.normalize();
            velocity.0 -= velocity_normalized * excess_speed * delta * 2.0;
        }

        // --- Specialized Physics ---
        match vehicle.vehicle_type {
            VehicleType::Motorcycle => {
                // Progressive lean into turns based on speed and steering
                let speed_factor = (vehicle.current_speed / vehicle.max_forward_speed).clamp(0.0, 1.0);
                let target_lean = -vehicle.steer_input * 35.0 * speed_factor; // Max 35 degrees at max speed
                vehicle.chassis_lean_y = vehicle.chassis_lean_y.lerp(target_lean, delta * 4.0);

                // Advanced stabilization for two-wheeled vehicles
                if vehicle.is_on_ground {
                    // Calculate desired up vector based on lean
                    let lean_rad = vehicle.chassis_lean_y.to_radians();
                    let desired_up = Vec3::new(lean_rad.sin(), lean_rad.cos(), 0.0);

                    // Current up vector
                    let current_up = transform.up();

                    // Calculate angular error
                    let roll_error = current_up.cross(desired_up);
                    let pitch_error = angular_vel.z; // Forward tilt

                    // Apply corrective torques with damping
                    let stabilizing_torque_roll = roll_error * vehicle.stability_force * 10.0;
                    let damping_torque_roll = -angular_vel.x * vehicle.stability_force * 2.0;

                    angular_vel.x += (stabilizing_torque_roll.x + damping_torque_roll) * delta;
                    angular_vel.z += pitch_error * vehicle.stability_force * delta;

                    // Counter-steering effect at low speeds
                    if vehicle.current_speed < 5.0 {
                        let counter_steer = vehicle.steer_input * vehicle.current_speed * 0.5;
                        angular_vel.y += counter_steer * delta;
                    }
                }
            }
            VehicleType::Hovercraft => {
                // Hovering physics: Apply upward force if close to ground
                // (Already handled by raycast suspension if configured properly, 
                // but we can add gliding feel)
                if vehicle.is_on_ground {
                    // Reduce friction/drag for gliding
                    velocity.0 *= 1.0 + 0.005 * delta; 
                    
                    // Surface following: Smoothly align with ground normal
                    // ... implementation ...
                }
            }
            VehicleType::Aircraft => {
                // Fixed-wing aircraft physics with coordinated turn mechanics
                if vehicle.is_turned_on {
                    let forward_speed = velocity.dot(*forward).max(0.0);
                    let speed_factor = (forward_speed / vehicle.max_forward_speed).clamp(0.0, 1.0);

                    // Throttle & Engine Power with speed-dependent thrust
                    let throttle = vehicle.motor_input.clamp(0.0, 1.0);
                    let thrust_efficiency = 1.0 - (speed_factor * 0.3); // Less efficient at high speeds
                    let engine_power = throttle * vehicle.engine_torque * thrust_efficiency
                        + if vehicle.is_boosting { vehicle.boost_multiplier * 50.0 } else { 0.0 };
                    velocity.0 += *forward * engine_power * delta;

                    // Aerodynamics: induced drag and parasitic drag
                    if velocity.length() > 1.0 {
                        let aero_factor = forward.dot(velocity.normalize());
                        let induced_drag = (1.0 - aero_factor.abs()) * forward_speed * vehicle.aero_dynamic_force;
                        let parasitic_drag = forward_speed * forward_speed * 0.001;

                        velocity.0 *= 1.0 - (induced_drag + parasitic_drag) * delta;

                        // Stall logic: progressive loss of control at low speeds
                        let stall_speed = vehicle.max_forward_speed * 0.25;
                        if forward_speed < stall_speed {
                            let stall_factor = 1.0 - (forward_speed / stall_speed).clamp(0.0, 1.0);
                            // Nose drops and control surfaces lose effectiveness
                            angular_vel.x += stall_factor * 2.0 * delta; // Pitch down
                        }
                    }

                    // Lift with realistic lift curve (stalls at high angle of attack)
                    let lift_direction = velocity.cross(*right).normalize_or_zero();
                    let dynamic_pressure = forward_speed * forward_speed;
                    let angle_of_attack = forward.dot(velocity.normalize()).acos().min(std::f32::consts::FRAC_PI_2);
                    let lift_coefficient = (angle_of_attack.sin() * 2.0).clamp(0.0, 1.5); // Simplified lift curve
                    let lift_power = dynamic_pressure * vehicle.lift_amount * lift_coefficient;
                    velocity.0 += lift_direction * lift_power * delta * 50.0;

                    // Control inputs with speed-dependent authority
                    let control_authority = speed_factor.clamp(0.2, 1.0);

                    // Pitch: controlled by vertical input (stick pitch)
                    let pitch_input = -vehicle.steer_input_speed; // Negative for pull-up
                    let pitch_torque = pitch_input * vehicle.pitch_force * control_authority;

                    // Yaw: coordinated with roll for turns
                    let yaw_input = vehicle.steer_input;
                    let yaw_torque = yaw_input * vehicle.yaw_force * control_authority;

                    // Roll: bank into turns using coordinated turn calculation
                    // In a coordinated turn: tan(roll) = v^2 / (r * g) where r is turn radius
                    // Simplified: roll proportional to yaw input and speed squared
                    let target_roll = -vehicle.steer_input * 45.0 * speed_factor; // Max 45 degree bank
                    let current_roll = up.dot(*right).asin().to_degrees(); // Current bank angle
                    let roll_error = target_roll - current_roll;
                    let roll_torque = roll_error.signum() * vehicle.roll_force * control_authority
                        + roll_error * 0.1; // Add proportional correction

                    let mut torque = Vec3::ZERO;
                    torque += *right * (pitch_torque * 0.5 + vehicle.motor_input * vehicle.pitch_force * 0.3);
                    torque += *up * yaw_torque;
                    torque += *forward * roll_torque;

                    angular_vel.0 += torque * forward_speed * 0.02 * delta;

                    // Advanced stability: artificial horizon and dampening
                    let roll_damping = -angular_vel.dot(*forward) * 0.5;
                    let pitch_damping = -angular_vel.dot(*right) * 0.5;
                    let yaw_damping = -angular_vel.dot(*up) * 0.3;

                    angular_vel.0 += *forward * roll_damping * delta;
                    angular_vel.0 += *right * pitch_damping * delta;
                    angular_vel.0 += *up * yaw_damping * delta;

                    // Auto-leveling when no input
                    if vehicle.steer_input.abs() < 0.1 && vehicle.steer_input_speed.abs() < 0.1 {
                        // Level roll
                        let roll_error = up.dot(*right);
                        angular_vel.0 -= *forward * roll_error * vehicle.stability_force * delta;
                        // Level pitch (slight nose-up trim)
                        let pitch_error = up.dot(*forward) - 0.1;
                        angular_vel.0 -= *right * pitch_error * vehicle.stability_force * 0.5 * delta;
                    }
                }
            }
            VehicleType::Flying => {
                // VTOL / Drone logic
                if vehicle.is_turned_on {
                    let mut target_vel = *forward * (vehicle.motor_input * vehicle.max_forward_speed) + 
                                       *right * (vehicle.steer_input * 15.0);
                    
                    // Hover force
                    let hover = *up * vehicle.hover_force;
                    target_vel += hover;

                    if vehicle.is_boosting { target_vel *= vehicle.boost_multiplier; }

                    velocity.0 = velocity.0.lerp(target_vel, delta * 2.0);

                    // Stability
                    let torque_vector = up.cross(Vec3::Y);
                    angular_vel.0 += torque_vector * vehicle.stability_force * delta;
                    
                    // Rotation
                    angular_vel.y += -vehicle.steer_input * vehicle.roll_rotation_speed * delta;
                }
            }
            VehicleType::Sphere => {
                // Sphere rolling physics
                if vehicle.is_turned_on {
                    let move_input = *forward * vehicle.motor_input + *right * vehicle.steer_input;
                    let force = move_input * vehicle.engine_torque * vehicle.move_speed_multiplier;
                    
                    if velocity.length() < vehicle.max_forward_speed {
                        velocity.0 += force * delta;
                    }
                }
            }
            VehicleType::Turret => {
                // Turret stabilization (base shouldn't move much)
                velocity.0 = Vec3::ZERO;
                angular_vel.0 = Vec3::ZERO;
            }
            VehicleType::Hoverboard => {
                // Hoverboard physics
                if vehicle.is_turned_on {
                    // Forward/Backward movement
                    let throttle = vehicle.motor_input * vehicle.engine_torque * delta;
                    velocity.0 += *forward * throttle;

                    // Steering (Torque based)
                    let steer = -vehicle.steer_input * vehicle.steering_angle.to_radians() * 20.0 * delta;
                    angular_vel.y += steer;

                    // Stability (Apply roll torque during turns)
                    let target_roll = -vehicle.steer_input * 0.5;
                    let roll_error = target_roll - transform.rotation.to_euler(EulerRot::YXZ).2;
                    angular_vel.z += roll_error * vehicle.hover_stability * delta;

                    // Hovering: If not on ground enough, apply extra hover force
                    if !vehicle.is_on_ground {
                        velocity.0 += *up * vehicle.hover_engine_force * delta;
                    }
                    
                    // Friction/Drag override
                    velocity.0 *= 1.0 - (vehicle.hover_damping * delta);
                }
            }
            _ => {}
        }

        // Update RPM based on gear and speed
        if powered_wheels_count > 0 {
            let avg_rpm = total_rpm / powered_wheels_count as f32;
            vehicle.current_rpm = (avg_rpm * vehicle.gear_shift_rate + vehicle.min_rpm) / (vehicle.current_gear as f32 + 1.0);
        } else {
            let speed_ratio = vehicle.current_speed / vehicle.max_forward_speed;
            vehicle.current_rpm = vehicle.min_rpm + (vehicle.max_rpm - vehicle.min_rpm) * speed_ratio;
        }
        vehicle.current_rpm = vehicle.current_rpm.clamp(vehicle.min_rpm, vehicle.max_rpm);
    }
}

/// Resource to track previous frame velocities for acceleration calculation
#[derive(Resource, Default)]
pub struct PassengerAccelerationTracker {
    pub velocities: std::collections::HashMap<Entity, Vec3>,
}

pub fn update_passenger_stability(
    time: Res<Time>,
    mut tracker: ResMut<PassengerAccelerationTracker>,
    vehicle_query: Query<(Entity, &Vehicle, &LinearVelocity, &AngularVelocity, &Transform, Option<&Children>)>,
    mut stability_query: Query<(&mut VehiclePassengerStability, &mut Transform)>,
) {
    let delta = time.delta_secs();
    const GRAVITY: f32 = 9.81;

    for (vehicle_entity, _vehicle, lin_vel, ang_vel, transform, children) in vehicle_query.iter() {
        let Some(children) = children else { continue };

        // Calculate linear acceleration (delta-v / delta-t)
        let prev_vel = tracker.velocities.get(&vehicle_entity).copied().unwrap_or(lin_vel.0);
        let linear_accel = (lin_vel.0 - prev_vel) / delta.max(0.001);

        // Update stored velocity
        tracker.velocities.insert(vehicle_entity, lin_vel.0);

        // Transform acceleration to local space
        let local_accel = transform.rotation.inverse() * linear_accel;

        // Calculate centrifugal/centripetal acceleration from rotation
        // a_c = ω × (ω × r) where r is offset from center of rotation
        // Simplified: lateral accel from yaw, vertical from pitch, longitudinal from roll
        let centripetal_accel = Vec3::new(
            -ang_vel.y.powi(2) * 0.5, // Lateral from yaw (turning)
            ang_vel.x.powi(2) * 0.3,  // Vertical from pitch
            -ang_vel.z.powi(2) * 0.5, // Longitudinal from roll
        );

        // Total G-force in local space (1.0 = normal gravity)
        let total_g_force = Vec3::new(
            (-local_accel.x / GRAVITY) + centripetal_accel.x,
            1.0 + (-local_accel.y / GRAVITY) + centripetal_accel.y,
            (-local_accel.z / GRAVITY) + centripetal_accel.z,
        );

        for child in children.iter() {
            if let Ok((mut stability, mut transform)) = stability_query.get_mut(child) {
                if !stability.enabled { continue; }

                // Calculate target lean based on G-forces
                // Passengers lean INTO the acceleration (opposite to G-force direction)
                // Scale by lean_amount for sensitivity tuning

                // Lateral lean (rolling into turns)
                let target_lean_z = total_g_force.x * stability.lean_amount;

                // Longitudinal lean (pitching under braking/acceleration)
                let target_lean_x = -total_g_force.z * stability.lean_amount * 0.7;

                // Vertical compression/stretch (subtle)
                let target_scale_y = 1.0 - (total_g_force.y - 1.0).abs() * 0.05;

                let target_lean = Vec3::new(target_lean_x, 0.0, target_lean_z);

                // Smooth interpolation
                stability.current_lean = stability.current_lean.lerp(
                    target_lean,
                    delta * stability.stability_speed
                );

                // Apply rotation to the passenger entity (usually child of seat)
                transform.rotation = Quat::from_euler(
                    EulerRot::XYZ,
                    stability.current_lean.x,
                    stability.current_lean.y,
                    stability.current_lean.z,
                );

                // Optional: Apply subtle vertical scaling under high G
                transform.scale.y = transform.scale.y.lerp(
                    target_scale_y.clamp(0.9, 1.1),
                    delta * stability.stability_speed
                );
            }
        }
    }
}
