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
        VehicleStats::default(),
        VehicleWeaponSystem {
            weapons: vec![VehicleWeapon::default()],
            aiming_enabled: true,
            weapons_activated: true,
            rotation_speed: 10.0,
            ..default()
        },
        VehicleDamageReceiver { damage_multiplier: 1.0 },
    )).insert((
        SkidManager {
            enabled: true,
            mark_width: 0.3,
            ground_offset: 0.02,
            min_distance: 0.1,
            max_marks: 1000,
            ..default()
        },
        Mesh3d(meshes.add(Cuboid::new(2.0, 1.0, 4.0))),
        MeshMaterial3d(materials.add(Color::from(LinearRgba::new(0.8, 0.2, 0.2, 1.0)))),
        Transform::from_translation(position),
        GlobalTransform::default(),
        RigidBody::Dynamic,
        Collider::cuboid(2.0, 1.0, 4.0),
        LinearVelocity::default(),
        AngularVelocity::default(),
    )).id();

    // Gears
    let gear1 = commands.spawn(VehicleGear {
        gear_name: "Gear 1".to_string(),
        gear_speed: 15.0,
        ..default()
    }).id();
    let gear2 = commands.spawn(VehicleGear {
        gear_name: "Gear 2".to_string(),
        gear_speed: 30.0,
        ..default()
    }).id();
    let gear3 = commands.spawn(VehicleGear {
        gear_name: "Gear 3".to_string(),
        gear_speed: 60.0,
        ..default()
    }).id();

    // Wheels (Simplified for car)
    let fl_wheel = commands.spawn((
        Name::new("Front Left Wheel"),
        VehicleWheel {
            wheel_name: "FL".to_string(),
            is_steerable: true,
            is_left_side: true,
            ..default()
        },
        Transform::from_xyz(-1.0, -0.5, 1.5),
        GlobalTransform::default(),
    )).with_children(|p| {
        p.spawn((
            Mesh3d(meshes.add(Cylinder::new(0.3, 0.2))),
            MeshMaterial3d(materials.add(Color::BLACK)),
            Transform::from_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
        ));
    }).id();

    let fr_wheel = commands.spawn((
        Name::new("Front Right Wheel"),
        VehicleWheel {
            wheel_name: "FR".to_string(),
            is_steerable: true,
            is_right_side: true,
            ..default()
        },
        Transform::from_xyz(1.0, -0.5, 1.5),
        GlobalTransform::default(),
    )).with_children(|p| {
        p.spawn((
            Mesh3d(meshes.add(Cylinder::new(0.3, 0.2))),
            MeshMaterial3d(materials.add(Color::BLACK)),
            Transform::from_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
        ));
    }).id();

    let rl_wheel = commands.spawn((
        Name::new("Rear Left Wheel"),
        VehicleWheel {
            wheel_name: "RL".to_string(),
            is_powered: true,
            is_left_side: true,
            ..default()
        },
        Transform::from_xyz(-1.0, -0.5, -1.5),
        GlobalTransform::default(),
    )).with_children(|p| {
        p.spawn((
            Mesh3d(meshes.add(Cylinder::new(0.3, 0.2))),
            MeshMaterial3d(materials.add(Color::BLACK)),
            Transform::from_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
        ));
    }).id();

    let rr_wheel = commands.spawn((
        Name::new("Rear Right Wheel"),
        VehicleWheel {
            wheel_name: "RR".to_string(),
            is_powered: true,
            is_right_side: true,
            ..default()
        },
        Transform::from_xyz(1.0, -0.5, -1.5),
        GlobalTransform::default(),
    )).with_children(|p| {
        p.spawn((
            Mesh3d(meshes.add(Cylinder::new(0.3, 0.2))),
            MeshMaterial3d(materials.add(Color::BLACK)),
            Transform::from_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
        ));
    }).id();

    // Driver Seat
    let driver_seat = commands.spawn((
        Name::new("Driver Seat"),
        VehicleSeat {
            seat_index: 0,
            is_driver_seat: true,
            offset: Vec3::new(-0.5, 0.5, 0.0),
            occupied_by: None,
            bounce_on_enter: true,
            ..default()
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
            ..default()
        },
        Transform::from_xyz(0.5, 0.5, 0.0),
        GlobalTransform::default(),
    )).id();

    let seats_vec = vec![driver_seat, passenger_seat];
    
    commands.entity(vehicle_entity).insert(VehicleSeatingManager {
        seats: seats_vec,
        eject_on_destroy: true,
        eject_force: 15.0,
        ..default()
    });

    commands.entity(vehicle_entity).add_children(&[
        gear1, gear2, gear3,
        fl_wheel, fr_wheel, rl_wheel, rr_wheel,
        driver_seat, passenger_seat
    ]);

    vehicle_entity
}
