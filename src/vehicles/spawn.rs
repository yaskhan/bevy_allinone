use bevy::prelude::*;
use crate::vehicles::types::*;
use crate::input::InputState;
use avian3d::prelude::*;

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
        MeshMaterial3d(materials.add(Color::from(LinearRgba::new(0.8, 0.2, 0.2, 1.0)))),
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
