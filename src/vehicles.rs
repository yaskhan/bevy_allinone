//! Vehicles system module
//!
//! Vehicle physics, controls, and passenger management.
//!
//! This module provides a comprehensive vehicle system.
//! It supports:
//! - Vehicle physics with acceleration, steering, and braking
//! - Multiple vehicle types (cars, trucks, etc.)
//! - Passenger management (entry/exit)
//! - Gear system with automatic shifting
//! - Boost/turbo system
//! - Jump system
//! - Vehicle chassis lean and stabilization
//! - Anti-roll system for better handling
//! - Wheel physics with suspension simulation

use bevy::prelude::*;
use crate::input::InputState;
use crate::character::{CharacterController, CharacterMovementState};
use avian3d::prelude::*;
use avian3d::spatial_query::SpatialQuery;

pub struct VehiclesPlugin;

impl Plugin for VehiclesPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Vehicle>()
            .register_type::<VehicleSeat>()
            .register_type::<VehicleDriver>()
            .register_type::<VehicleWheel>()
            .register_type::<VehicleGear>()
            .add_systems(Update, (
                vehicle_input_system,
                character_vehicle_sync_system,
                update_vehicles_physics,
                handle_vehicle_interaction,
                update_vehicle_wheels,
                update_vehicle_chassis,
                update_vehicle_audio,
            ));
    }
}

/// Vehicle component
/// Main component for vehicle physics and settings
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct Vehicle {
    pub vehicle_name: String,
    pub vehicle_type: VehicleType,

    // Physics settings
    pub max_forward_speed: f32,
    pub max_backward_speed: f32,
    pub engine_torque: f32,
    pub rear_engine_torque: f32,
    pub brake_power: f32,
    pub steering_angle: f32,
    pub high_speed_steering_angle: f32,
    pub high_speed_steering_at_speed: f32,

    // Boost settings
    pub can_use_boost: bool,
    pub boost_multiplier: f32,
    pub boost_energy_cost: f32,
    pub boost_energy_rate: f32,

    // Jump settings
    pub can_jump: bool,
    pub jump_power: f32,
    pub can_impulse: bool,
    pub impulse_force: f32,
    pub impulse_energy_cost: f32,

    // Chassis settings
    pub chassis_lean: Vec2,
    pub chassis_lean_limit: f32,
    pub anti_roll: f32,
    pub preserve_direction_in_air: bool,

    // State
    pub current_gear: usize,
    pub current_speed: f32,
    pub current_rpm: f32,
    pub is_turned_on: bool,
    pub is_driving: bool,
    pub is_reversing: bool,
    pub is_braking: bool,
    pub is_boosting: bool,
    pub is_jumping: bool,
    pub is_on_ground: bool,

    // Input
    pub motor_input: f32,
    pub steer_input: f32,
    pub boost_input: f32,

    // Internal
    pub current_steering: f32,
    pub chassis_lean_x: f32,
    pub chassis_lean_y: f32,
    pub time_to_stabilize: f32,
    pub is_rotating: f32,
}

#[derive(Debug, Clone, Reflect)]
pub enum VehicleType {
    Car,
    Truck,
    Motorcycle,
    Boat,
    Plane,
    Hovercraft,
}

impl Default for Vehicle {
    fn default() -> Self {
        Self {
            vehicle_name: "Generic Car".to_string(),
            vehicle_type: VehicleType::Car,
            max_forward_speed: 25.0,
            max_backward_speed: 10.0,
            engine_torque: 2500.0,
            rear_engine_torque: 2500.0,
            brake_power: 4000.0,
            steering_angle: 35.0,
            high_speed_steering_angle: 10.0,
            high_speed_steering_at_speed: 100.0,
            can_use_boost: true,
            boost_multiplier: 2.0,
            boost_energy_cost: 1.0,
            boost_energy_rate: 0.5,
            can_jump: false,
            jump_power: 0.0,
            can_impulse: false,
            impulse_force: 0.0,
            impulse_energy_cost: 0.0,
            chassis_lean: Vec2::new(0.5, 0.5),
            chassis_lean_limit: 10.0,
            anti_roll: 10000.0,
            preserve_direction_in_air: true,
            current_gear: 0,
            current_speed: 0.0,
            current_rpm: 0.0,
            is_turned_on: false,
            is_driving: false,
            is_reversing: false,
            is_braking: false,
            is_boosting: false,
            is_jumping: false,
            is_on_ground: true,
            motor_input: 0.0,
            steer_input: 0.0,
            boost_input: 1.0,
            current_steering: 0.0,
            chassis_lean_x: 0.0,
            chassis_lean_y: 0.0,
            time_to_stabilize: 0.0,
            is_rotating: 0.0,
        }
    }
}

/// Marker for the current driver of a vehicle
#[derive(Component, Debug, Reflect)]
pub struct VehicleDriver;

/// Vehicle seat component
#[derive(Component, Debug, Reflect)]
pub struct VehicleSeat {
    pub seat_index: usize,
    pub is_driver_seat: bool,
    pub offset: Vec3,
    pub occupied_by: Option<Entity>,
    pub bounce_on_enter: bool,
}

impl Default for VehicleSeat {
    fn default() -> Self {
        Self {
            seat_index: 0,
            is_driver_seat: false,
            offset: Vec3::ZERO,
            occupied_by: None,
            bounce_on_enter: true,
        }
    }
}

/// Vehicle wheel component
/// Represents a single wheel with physics properties
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct VehicleWheel {
    pub wheel_name: String,
    pub radius: f32,
    pub suspension_distance: f32,
    pub is_steerable: bool,
    pub is_powered: bool,
    pub is_left_side: bool,
    pub is_right_side: bool,
    pub reverse_steer: bool,

    // Physics state
    pub current_rpm: f32,
    pub rotation_value: f32,
    pub slip_amount_sideways: f32,
    pub slip_amount_forward: f32,
    pub suspension_spring_pos: f32,

    // Visual state
    pub wheel_mesh: Option<Entity>,
    pub mudguard: Option<Entity>,
    pub suspension: Option<Entity>,
}

impl Default for VehicleWheel {
    fn default() -> Self {
        Self {
            wheel_name: "Wheel".to_string(),
            radius: 0.3,
            suspension_distance: 0.2,
            is_steerable: false,
            is_powered: false,
            is_left_side: false,
            is_right_side: false,
            reverse_steer: false,
            current_rpm: 0.0,
            rotation_value: 0.0,
            slip_amount_sideways: 0.0,
            slip_amount_forward: 0.0,
            suspension_spring_pos: 0.0,
            wheel_mesh: None,
            mudguard: None,
            suspension: None,
        }
    }
}

/// Vehicle gear system
#[derive(Component, Debug, Reflect, Clone)]
#[reflect(Component)]
pub struct VehicleGear {
    pub gear_name: String,
    pub gear_speed: f32,
    pub torque_curve: Vec<f32>, // Simple curve representation
}

impl Default for VehicleGear {
    fn default() -> Self {
        Self {
            gear_name: "Gear 1".to_string(),
            gear_speed: 10.0,
            torque_curve: vec![0.0, 0.5, 1.0, 0.8, 0.5],
        }
    }
}

/// Vehicle audio component
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct VehicleAudio {
    pub engine_pitch: f32,
    pub engine_volume: f32,
    pub skid_volume: f32,
    pub is_engine_playing: bool,
    pub is_skid_playing: bool,
}

impl Default for VehicleAudio {
    fn default() -> Self {
        Self {
            engine_pitch: 1.0,
            engine_volume: 0.0,
            skid_volume: 0.0,
            is_engine_playing: false,
            is_skid_playing: false,
        }
    }
}

/// System to sync driver's input to the vehicle
fn vehicle_input_system(
    time: Res<Time>,
    mut vehicle_query: Query<(&mut Vehicle, &mut InputState, &Children)>,
    driver_query: Query<&InputState, (With<VehicleDriver>, Without<Vehicle>)>,
) {
    let delta = time.delta_secs();

    for (mut vehicle, mut v_input, children) in vehicle_query.iter_mut() {
        let mut found_driver = false;
        for child in children.iter() {
            if let Ok(input) = driver_query.get(child) {
                v_input.movement = input.movement;
                v_input.jump_pressed = input.jump_pressed;
                v_input.interact_pressed = input.interact_pressed;
                found_driver = true;
                break;
            }
        }

        if !found_driver {
            v_input.movement = Vec2::ZERO;
            v_input.jump_pressed = false;
            vehicle.is_boosting = false;
            vehicle.is_jumping = false;
        }

        // Update vehicle input state
        vehicle.is_driving = found_driver;

        // Handle boost input
        if vehicle.can_use_boost && vehicle.is_driving && vehicle.is_turned_on {
            if v_input.jump_pressed && !vehicle.is_jumping {
                vehicle.is_boosting = true;
                vehicle.boost_input = vehicle.boost_multiplier;
            } else {
                vehicle.is_boosting = false;
                vehicle.boost_input = 1.0;
            }
        } else {
            vehicle.is_boosting = false;
            vehicle.boost_input = 1.0;
        }

        // Handle jump input
        if vehicle.can_jump && vehicle.is_driving && vehicle.is_turned_on && v_input.jump_pressed && !vehicle.is_jumping {
            vehicle.is_jumping = true;
        }

        // Update motor and steer inputs with smoothing
        let target_motor = v_input.movement.y;
        let target_steer = v_input.movement.x;

        // Smooth steering
        vehicle.steer_input += (target_steer - vehicle.steer_input) * 10.0 * delta;
        vehicle.steer_input = vehicle.steer_input.clamp(-1.0, 1.0);

        // Smooth motor input
        if !vehicle.is_reversing {
            vehicle.motor_input += (target_motor - vehicle.motor_input) * 5.0 * delta;
        } else {
            vehicle.motor_input += (target_motor - vehicle.motor_input) * 3.0 * delta;
        }
        vehicle.motor_input = vehicle.motor_input.clamp(-1.0, 1.0);

        // Handle braking
        vehicle.is_braking = vehicle.motor_input.abs() < 0.05 && vehicle.current_speed > 0.5;
    }
}

/// System to apply physics forces to the vehicle
fn update_vehicles_physics(
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
        let ray_direction = -up;
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
            // Calculate roll based on angular velocity
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
                // Stabilize rotation towards velocity direction
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

/// System to keep passengers/drivers attached to their seats
fn character_vehicle_sync_system(
    vehicle_query: Query<(&GlobalTransform, &Children, &Vehicle)>,
    seat_query: Query<&VehicleSeat>,
    mut occupant_query: Query<(&mut Transform, &mut LinearVelocity, &mut CharacterMovementState)>,
) {
    for (v_gt, children, vehicle) in vehicle_query.iter() {
        for child in children.iter() {
            if let Ok(seat) = seat_query.get(child) {
                if let Some(occupant_entity) = seat.occupied_by {
                    if let Ok((mut trans, mut vel, mut state)) = occupant_query.get_mut(occupant_entity) {
                        // Position character in seat with offset
                        let seat_world_pos = v_gt.translation() + v_gt.rotation() * seat.offset;
                        trans.translation = seat_world_pos;

                        // Rotate character to face forward
                        trans.rotation = v_gt.rotation();

                        // Disable character movement
                        vel.0 = Vec3::ZERO;

                        // Update character state to indicate in vehicle
                        state.is_in_vehicle = true;
                        state.vehicle_entity = Some(occupant_entity);
                    }
                }
            }
        }
    }
}

/// System to handle entering/exiting vehicles
fn handle_vehicle_interaction(
    time: Res<Time>,
    mut commands: Commands,
    mut character_query: Query<(Entity, &GlobalTransform, &InputState, &mut CharacterMovementState, Has<ChildOf>), With<CharacterController>>,
    mut seat_query: Query<(Entity, &mut VehicleSeat, &GlobalTransform)>,
    mut vehicle_query: Query<&mut Vehicle>,
) {
    let delta = time.delta_secs();

    for (entity, gt, input, mut state, is_in_vehicle) in character_query.iter_mut() {
        if input.interact_pressed {
            if is_in_vehicle {
                // If already in a vehicle/seat, try to exit
                for (seat_entity, mut seat, seat_gt) in seat_query.iter_mut() {
                    if seat.occupied_by == Some(entity) {
                        // Exit
                        info!("Exiting vehicle...");
                        seat.occupied_by = None;
                        commands.entity(entity).remove_parent_in_place();
                        commands.entity(entity).remove::<VehicleDriver>();

                        // Update character state
                        state.is_in_vehicle = false;
                        state.vehicle_entity = None;

                        // Teleport slightly outside
                        let exit_pos = seat_gt.translation() + seat_gt.right() * 2.0 + Vec3::Y * 0.5;
                        commands.entity(entity).insert(Transform::from_translation(exit_pos));

                        // Update vehicle state - skip if no parent
                        // Note: In Bevy, GlobalTransform doesn't have a parent() method
                        // We need to track the vehicle entity separately

                        break;
                    }
                }
                continue;
            }

            // If not in a vehicle, look for one to enter
            let char_pos: Vec3 = gt.translation();
            let mut closest_seat_entity = None;
            let mut min_dist = 4.0;

            for (seat_entity, seat, seat_gt) in seat_query.iter() {
                if seat.occupied_by.is_none() {
                    let dist = seat_gt.translation().distance(char_pos);
                    if dist < min_dist {
                        min_dist = dist;
                        closest_seat_entity = Some(seat_entity);
                    }
                }
            }

            if let Some(seat_entity) = closest_seat_entity {
                info!("Entering vehicle seat...");
                if let Ok((_, mut seat, seat_gt)) = seat_query.get_mut(seat_entity) {
                    seat.occupied_by = Some(entity);
                    let is_driver = seat.is_driver_seat;

                    commands.entity(entity).set_parent_in_place(seat_entity);
                    if is_driver {
                        commands.entity(entity).insert(VehicleDriver);
                    }

                    // Update character state
                    state.is_in_vehicle = true;
                    state.vehicle_entity = Some(seat_entity);

                    // Update vehicle state - skip if no parent
                    // Note: In Bevy, GlobalTransform doesn't have a parent() method
                    // The vehicle state will be updated by the input system
                }
            }
        }
    }
}

pub fn spawn_vehicle(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
    vehicle_type: VehicleType,
) -> Entity {
    let mut vehicle = Vehicle::default();
    vehicle.vehicle_type = vehicle_type.clone();

    // Configure vehicle based on type
    match vehicle_type {
        VehicleType::Car => {
            vehicle.vehicle_name = "Sports Car".to_string();
            vehicle.max_forward_speed = 30.0;
            vehicle.max_backward_speed = 15.0;
            vehicle.engine_torque = 3000.0;
            vehicle.steering_angle = 35.0;
            vehicle.can_jump = false;
            vehicle.can_use_boost = true;
            vehicle.boost_multiplier = 2.5;
        }
        VehicleType::Truck => {
            vehicle.vehicle_name = "Delivery Truck".to_string();
            vehicle.max_forward_speed = 20.0;
            vehicle.max_backward_speed = 10.0;
            vehicle.engine_torque = 4000.0;
            vehicle.steering_angle = 25.0;
            vehicle.can_jump = false;
            vehicle.can_use_boost = false;
        }
        VehicleType::Motorcycle => {
            vehicle.vehicle_name = "Motorcycle".to_string();
            vehicle.max_forward_speed = 35.0;
            vehicle.max_backward_speed = 10.0;
            vehicle.engine_torque = 2000.0;
            vehicle.steering_angle = 45.0;
            vehicle.can_jump = true;
            vehicle.jump_power = 5.0;
            vehicle.can_use_boost = true;
            vehicle.boost_multiplier = 2.0;
        }
        VehicleType::Boat => {
            vehicle.vehicle_name = "Boat".to_string();
            vehicle.max_forward_speed = 25.0;
            vehicle.max_backward_speed = 10.0;
            vehicle.engine_torque = 2500.0;
            vehicle.steering_angle = 30.0;
            vehicle.can_jump = false;
            vehicle.can_use_boost = false;
        }
        VehicleType::Plane => {
            vehicle.vehicle_name = "Plane".to_string();
            vehicle.max_forward_speed = 50.0;
            vehicle.max_backward_speed = 10.0;
            vehicle.engine_torque = 5000.0;
            vehicle.steering_angle = 20.0;
            vehicle.can_jump = false;
            vehicle.can_use_boost = true;
            vehicle.boost_multiplier = 3.0;
        }
        VehicleType::Hovercraft => {
            vehicle.vehicle_name = "Hovercraft".to_string();
            vehicle.max_forward_speed = 25.0;
            vehicle.max_backward_speed = 15.0;
            vehicle.engine_torque = 2800.0;
            vehicle.steering_angle = 40.0;
            vehicle.can_jump = true;
            vehicle.jump_power = 8.0;
            vehicle.can_use_boost = true;
            vehicle.boost_multiplier = 2.0;
        }
    }

    let vehicle_entity = commands.spawn((
        Name::new(vehicle.vehicle_name.clone()),
        vehicle,
        InputState::default(),
        VehicleAudio::default(),
        Mesh3d(meshes.add(Cuboid::new(2.0, 1.0, 4.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.2, 0.2))),
        Transform::from_translation(position),
        GlobalTransform::default(),
        RigidBody::Dynamic,
        Collider::cuboid(2.0, 1.0, 4.0),
        LinearVelocity::default(),
        AngularVelocity::default(),
    )).id();

    // Driver Seat
    let driver_seat = commands.spawn((
        Name::new("Driver Seat"),
        VehicleSeat {
            seat_index: 0,
            is_driver_seat: true,
            offset: Vec3::new(-0.5, 0.5, 0.0),
            occupied_by: None,
            bounce_on_enter: true,
        },
        Transform::from_xyz(-0.5, 0.5, 0.0),
        GlobalTransform::default(),
    )).id();

    // Passenger Seat
    let passenger_seat = commands.spawn((
        Name::new("Passenger Seat"),
        VehicleSeat {
            seat_index: 1,
            is_driver_seat: false,
            offset: Vec3::new(0.5, 0.5, 0.0),
            occupied_by: None,
            bounce_on_enter: true,
        },
        Transform::from_xyz(0.5, 0.5, 0.0),
        GlobalTransform::default(),
    )).id();

    commands.entity(vehicle_entity).add_children(&[driver_seat, passenger_seat]);

    vehicle_entity
}

/// System to update vehicle wheels
fn update_vehicle_wheels(
    time: Res<Time>,
    mut vehicle_query: Query<(&mut Vehicle, &Children)>,
    mut wheel_query: Query<&mut VehicleWheel>,
    transform_query: Query<&Transform>,
) {
    let delta = time.delta_secs();

    for (mut vehicle, children) in vehicle_query.iter_mut() {
        let mut total_rpm = 0.0;
        let mut powered_wheels = 0;

        for child in children.iter() {
            if let Ok(mut wheel) = wheel_query.get_mut(child) {
                if let Ok(wheel_transform) = transform_query.get(child) {
                    // Update wheel rotation based on vehicle speed
                    if wheel.is_powered {
                        let wheel_rpm = vehicle.current_speed * 10.0;
                        wheel.current_rpm = wheel_rpm;
                        wheel.rotation_value += wheel_rpm * delta * 6.0;
                        total_rpm += wheel_rpm;
                        powered_wheels += 1;
                    }

                    // Calculate slip
                    let forward_speed = vehicle.current_speed;
                    wheel.slip_amount_forward = (forward_speed * 0.1).clamp(0.0, 1.0);
                    wheel.slip_amount_sideways = (vehicle.steer_input.abs() * forward_speed * 0.05).clamp(0.0, 1.0);
                }
            }
        }

        // Average RPM for powered wheels
        if powered_wheels > 0 {
            vehicle.current_rpm = total_rpm / powered_wheels as f32;
        }
    }
}

/// System to update vehicle chassis (lean and rotation)
fn update_vehicle_chassis(
    time: Res<Time>,
    mut vehicle_query: Query<(&mut Vehicle, &mut Transform)>,
) {
    let delta = time.delta_secs();

    for (mut vehicle, mut transform) in vehicle_query.iter_mut() {
        // Apply chassis lean
        let lean_x = vehicle.chassis_lean_x;
        let lean_y = vehicle.chassis_lean_y;

        // Decay lean over time
        vehicle.chassis_lean_x *= 0.95;
        vehicle.chassis_lean_y *= 0.95;

        // Apply rotation to chassis (visual only)
        let current_euler = transform.rotation.to_euler(EulerRot::XYZ);
        let new_euler = (
            current_euler.0 + lean_x.to_radians() * delta,
            current_euler.1,
            current_euler.2 + lean_y.to_radians() * delta,
        );
        transform.rotation = Quat::from_euler(EulerRot::XYZ, new_euler.0, new_euler.1, new_euler.2);
    }
}

/// System to update vehicle audio
fn update_vehicle_audio(
    time: Res<Time>,
    mut vehicle_query: Query<(&mut Vehicle, &mut VehicleAudio)>,
) {
    let delta = time.delta_secs();

    for (mut vehicle, mut audio) in vehicle_query.iter_mut() {
        if vehicle.is_turned_on && vehicle.is_driving {
            // Update engine audio
            let target_pitch = 1.0 + (vehicle.current_rpm / 6000.0);
            audio.engine_pitch += (target_pitch - audio.engine_pitch) * 5.0 * delta;
            audio.engine_volume = 0.5 + (vehicle.motor_input.abs() * 0.5);
            audio.is_engine_playing = true;

            // Update skid audio
            let target_skid = (vehicle.current_speed * 0.02).clamp(0.0, 1.0);
            audio.skid_volume += (target_skid - audio.skid_volume) * 10.0 * delta;
            audio.is_skid_playing = audio.skid_volume > 0.1;
        } else {
            // Turn off audio
            audio.engine_volume *= 0.9;
            audio.skid_volume *= 0.9;
            if audio.engine_volume < 0.01 {
                audio.is_engine_playing = false;
            }
            if audio.skid_volume < 0.01 {
                audio.is_skid_playing = false;
            }
        }
    }
}
