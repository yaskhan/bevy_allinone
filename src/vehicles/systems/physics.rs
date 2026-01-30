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
                // Lean into turns
                let target_lean = -vehicle.steer_input * 25.0; // Max 25 degrees
                vehicle.chassis_lean_y = vehicle.chassis_lean_y.lerp(target_lean, delta * 5.0);
                
                // Stabilization force for two wheels (simplified)
                if vehicle.is_on_ground {
                    let roll_error = angular_vel.x;
                    angular_vel.x -= roll_error * 5.0 * delta;
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
                // Fixed-wing aircraft physics
                if vehicle.is_turned_on {
                    let forward_speed = velocity.dot(*forward).max(0.0);
                    let local_velocity = transform.to_matrix().inverse().transform_point3((*velocity).0);
                    
                    // Throttle & Engine Power
                    let throttle = vehicle.motor_input.clamp(0.0, 1.0);
                    let engine_power = throttle * vehicle.engine_torque + (if vehicle.is_boosting { vehicle.boost_multiplier * 50.0 } else { 0.0 });
                    velocity.0 += *forward * engine_power * delta;

                    // Aerodynamics: align velocity with forward direction
                    if velocity.length() > 1.0 {
                        let aero_factor = forward.dot(velocity.normalize()).powi(2);
                        let target_vel = *forward * forward_speed;
                        velocity.0 = velocity.0.lerp(target_vel, aero_factor * forward_speed * vehicle.aero_dynamic_force * delta);
                        
                        // Stall logic: rotate downward if speed is low
                        let stall_rotation = Quat::from_rotation_arc(*forward, velocity.normalize());
                        angular_vel.0 = angular_vel.0.lerp(stall_rotation.to_scaled_axis(), vehicle.aero_dynamic_force * delta);
                    }

                    // Lift
                    let lift_direction = velocity.cross(*right).normalize();
                    let zero_lift_factor = (1.0 - (forward_speed / 300.0)).clamp(0.0, 1.0);
                    let lift_power = forward_speed * forward_speed * vehicle.lift_amount * zero_lift_factor;
                    velocity.0 += lift_direction * lift_power * delta * 100.0; // Scaled for physics

                    // Control Torque (Pitch, Yaw, Roll)
                    let pitch = -vehicle.steer_input_speed * vehicle.pitch_force; // Using stick vertical or separate vertical axis if needed
                    let yaw = vehicle.steer_input * vehicle.yaw_force;
                    let roll = -vehicle.steer_input * vehicle.roll_force; // Simplified mapping for now

                    let mut torque = Vec3::ZERO;
                    torque += *right * (vehicle.motor_input * vehicle.pitch_force); // Pitch
                    torque += *up * (vehicle.steer_input * vehicle.yaw_force); // Yaw
                    torque += *forward * (vehicle.boost_input * vehicle.roll_force); // Roll (placeholder mapping)

                    angular_vel.0 += torque * forward_speed * 0.01 * delta;
                    
                    // Stability: level out if no input
                    if vehicle.steer_input.abs() < 0.1 {
                        let roll_error = up.dot(*right); // Sin of roll angle
                        angular_vel.0 -= *forward * roll_error * vehicle.stability_force * delta;
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

pub fn update_passenger_stability(
    time: Res<Time>,
    vehicle_query: Query<(&Vehicle, &LinearVelocity, &AngularVelocity, Option<&Children>)>,
    mut stability_query: Query<(&mut VehiclePassengerStability, &mut Transform)>,
) {
    let delta = time.delta_secs();

    for (_vehicle, _lin_vel, ang_vel, children) in vehicle_query.iter() {
        let Some(children) = children else { continue; };
        // Calculate G-forces based on acceleration (simplified via velocity changes)
        // We'll use angular velocity and rotation for leaning
        let local_ang_vel = ang_vel.0; 
        
        for child in children.iter() {
            if let Ok((mut stability, mut transform)) = stability_query.get_mut(child) {
                if !stability.enabled { continue; }

                // Target lean depends on lateral acceleration and turning
                let target_lean_x = local_ang_vel.y * stability.lean_amount; // Leaning back/forward
                let target_lean_z = -local_ang_vel.x * stability.lean_amount; // Leaning side to side

                let target_lean = Vec3::new(target_lean_x, 0.0, target_lean_z);
                stability.current_lean = stability.current_lean.lerp(target_lean, delta * stability.stability_speed);

                // Apply rotation to the passenger entity (usually child of seat)
                transform.rotation = Quat::from_euler(
                    EulerRot::XYZ,
                    stability.current_lean.x,
                    stability.current_lean.y,
                    stability.current_lean.z,
                );
            }
        }
    }
}
