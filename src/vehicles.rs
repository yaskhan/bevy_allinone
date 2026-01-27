//! Vehicles system module
//!
//! Vehicle physics, controls, and passenger management.

use bevy::prelude::*;
use crate::input::InputState;
use crate::character::{CharacterController, CharacterMovementState};
use avian3d::prelude::*;

pub struct VehiclesPlugin;

impl Plugin for VehiclesPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Vehicle>()
            .register_type::<VehicleSeat>()
            .register_type::<VehicleDriver>()
            .add_systems(Update, (
                vehicle_input_system,
                character_vehicle_sync_system,
                update_vehicles_physics,
                handle_vehicle_interaction,
            ));
    }
}

/// Vehicle component
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct Vehicle {
    pub vehicle_name: String,
    pub max_forward_speed: f32,
    pub max_backward_speed: f32,
    pub acceleration: f32,
    pub steering_angle: f32, // Degrees
    pub current_steering: f32, // -1.0 to 1.0
}

impl Default for Vehicle {
    fn default() -> Self {
        Self {
            vehicle_name: "Generic Car".to_string(),
            max_forward_speed: 25.0,
            max_backward_speed: 10.0,
            acceleration: 15.0,
            steering_angle: 35.0,
            current_steering: 0.0,
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
}

/// System to sync diver's input to the vehicle
fn vehicle_input_system(
    mut vehicle_query: Query<(&mut Vehicle, &mut InputState, &Children)>,
    driver_query: Query<&InputState, (With<VehicleDriver>, Without<Vehicle>)>,
) {
    for (_vehicle, mut v_input, children) in vehicle_query.iter_mut() {
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
        }
    }
}

/// System to apply physics forces to the vehicle
fn update_vehicles_physics(
    time: Res<Time>,
    mut query: Query<(&Vehicle, &InputState, &mut LinearVelocity, &mut AngularVelocity, &Transform)>,
) {
    let delta = time.delta_secs();

    for (vehicle, input, mut velocity, mut angular_vel, transform) in query.iter_mut() {
        let forward = transform.forward();

        // Driving
        let target_speed = if input.movement.y > 0.0 {
            input.movement.y * vehicle.max_forward_speed
        } else {
            input.movement.y * vehicle.max_backward_speed
        };

        // Simple linear acceleration towards target speed in forward direction
        let current_forward_speed = velocity.dot(*forward);
        let speed_diff = target_speed - current_forward_speed;
        let accel = speed_diff * vehicle.acceleration * delta;
        
        velocity.0 += *forward * accel;

        // Steering
        // Steering is more effective when moving
        let steering_input = input.movement.x;
        let steer_speed = current_forward_speed.abs().min(10.0) / 10.0;
        let turn_amount = -steering_input * vehicle.steering_angle.to_radians() * steer_speed;
        
        angular_vel.y = turn_amount * 2.0;

        // Apply friction/drag to stop drifting forever
        velocity.0 *= 0.99;
        angular_vel.0 *= 0.9;
    }
}

/// System to keep passengers/drivers attached to their seats
fn character_vehicle_sync_system(
    vehicle_query: Query<(&GlobalTransform, &Children)>,
    seat_query: Query<&VehicleSeat>,
    mut occupant_query: Query<(&mut Transform, &mut LinearVelocity, &mut CharacterMovementState)>,
) {
    for (v_gt, children) in vehicle_query.iter() {
        for child in children.iter() {
            if let Ok(seat) = seat_query.get(child) {
                if let Some(occupant_entity) = seat.occupied_by {
                    if let Ok((mut trans, mut vel, _state)) = occupant_query.get_mut(occupant_entity) {
                        // Position character in seat
                        trans.translation = v_gt.translation();
                        trans.rotation = v_gt.rotation();
                        
                        // Disable movement
                        vel.0 = Vec3::ZERO;
                        // We might need to handle character state to stop animations
                    }
                }
            }
        }
    }
}

/// System to handle entering/exiting vehicles
fn handle_vehicle_interaction(
    mut commands: Commands,
    mut character_query: Query<(Entity, &GlobalTransform, &InputState, Has<ChildOf>), With<CharacterController>>,
    mut seat_query: Query<(Entity, &mut VehicleSeat, &GlobalTransform)>,
) {
    for (entity, gt, input, is_in_vehicle) in character_query.iter_mut() {
        if input.interact_pressed {
            if is_in_vehicle {
                // If already in a vehicle/seat, try to exit
                for (_seat_entity, mut seat, seat_gt) in seat_query.iter_mut() {
                    if seat.occupied_by == Some(entity) {
                        // Exit
                        info!("Exiting vehicle...");
                        seat.occupied_by = None;
                        commands.entity(entity).remove_parent_in_place();
                        commands.entity(entity).remove::<VehicleDriver>();
                        
                        // Teleport slightly outside
                        let exit_pos = seat_gt.translation() + seat_gt.right() * 2.0 + Vec3::Y * 0.5;
                        commands.entity(entity).insert(Transform::from_translation(exit_pos));
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
                if let Ok((_, mut seat, _)) = seat_query.get_mut(seat_entity) {
                    seat.occupied_by = Some(entity);
                    let is_driver = seat.is_driver_seat;
                    
                    commands.entity(entity).set_parent_in_place(seat_entity);
                    if is_driver {
                        commands.entity(entity).insert(VehicleDriver);
                    }
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
) -> Entity {
    let vehicle_entity = commands.spawn((
        Name::new("Vehicle"),
        Vehicle::default(),
        InputState::default(),
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
        },
        Transform::from_xyz(0.5, 0.5, 0.0),
        GlobalTransform::default(),
    )).id();

    commands.entity(vehicle_entity).add_children(&[driver_seat, passenger_seat]);

    vehicle_entity
}
