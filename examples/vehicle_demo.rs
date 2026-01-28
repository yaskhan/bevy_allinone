//! Vehicle Demo Example
//!
//! This example demonstrates the vehicle system with:
//! - Multiple vehicle types (car, truck, motorcycle, etc.)
//! - Vehicle physics and controls
//! - Passenger management (entry/exit)
//! - Boost system
//! - Jump system
//! - Camera following

use bevy::prelude::*;
use avian3d::prelude::*;
use bevy_allinone::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            VehiclesPlugin,
            CharacterPlugin,
            InputPlugin,
            CameraPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (
            spawn_vehicle_system,
            vehicle_controls_system,
            vehicle_camera_system,
            debug_vehicle_info_system,
        ))
        .run();
}

/// Setup the scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn camera
    commands.spawn((
        Name::new("Camera"),
        Camera3d::default(),
        Transform::from_xyz(0.0, 10.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Spawn ground
    commands.spawn((
        Name::new("Ground"),
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::new(100.0, 100.0)))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Transform::from_xyz(0.0, 0.0, 0.0),
        RigidBody::Static,
        Collider::cuboid(100.0, 0.1, 100.0),
    ));

    // Spawn some obstacles
    for i in 0..5 {
        commands.spawn((
            Name::new(format!("Obstacle {}", i)),
            Mesh3d(meshes.add(Cuboid::new(2.0, 2.0, 2.0))),
            MeshMaterial3d(materials.add(Color::srgb(0.8, 0.2, 0.2))),
            Transform::from_xyz(i as f32 * 10.0 - 20.0, 1.0, 5.0),
            RigidBody::Static,
            Collider::cuboid(1.0, 1.0, 1.0),
        ));
    }

    // Spawn character
    let _character = commands.spawn((
        Name::new("Player"),
        CharacterController::default(),
        CharacterMovementState::default(),
        InputState::default(),
        Mesh3d(meshes.add(Capsule3d::new(0.3, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.2, 0.4, 0.8))),
        Transform::from_xyz(0.0, 1.0, 10.0),
        GlobalTransform::default(),
        RigidBody::Dynamic,
        Collider::capsule(0.3, 1.0),
        LinearVelocity::default(),
        AngularVelocity::default(),
        Player,
    )).id();

    info!("Vehicle Demo Started!");
    info!("Controls:");
    info!("  1-6: Spawn different vehicle types");
    info!("  W/A/S/D: Drive vehicle");
    info!("  Space: Boost/Jump");
    info!("  E: Enter/Exit vehicle");
    info!("  R: Reset camera");
    info!("  ESC: Exit");
}

/// System to spawn vehicles
fn spawn_vehicle_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    vehicle_query: Query<(Entity, &Vehicle)>,
) {
    if keyboard_input.just_pressed(KeyCode::Digit1) {
        spawn_vehicle(&mut commands, &mut meshes, &mut materials, Vec3::new(0.0, 1.0, 0.0), VehicleType::Car);
        info!("Spawned Sports Car");
    }
    if keyboard_input.just_pressed(KeyCode::Digit2) {
        spawn_vehicle(&mut commands, &mut meshes, &mut materials, Vec3::new(5.0, 1.0, 0.0), VehicleType::Truck);
        info!("Spawned Delivery Truck");
    }
    if keyboard_input.just_pressed(KeyCode::Digit3) {
        spawn_vehicle(&mut commands, &mut meshes, &mut materials, Vec3::new(10.0, 1.0, 0.0), VehicleType::Motorcycle);
        info!("Spawned Motorcycle");
    }
    if keyboard_input.just_pressed(KeyCode::Digit4) {
        spawn_vehicle(&mut commands, &mut meshes, &mut materials, Vec3::new(15.0, 1.0, 0.0), VehicleType::Boat);
        info!("Spawned Boat");
    }
    if keyboard_input.just_pressed(KeyCode::Digit5) {
        spawn_vehicle(&mut commands, &mut meshes, &mut materials, Vec3::new(20.0, 1.0, 0.0), VehicleType::Plane);
        info!("Spawned Plane");
    }
    if keyboard_input.just_pressed(KeyCode::Digit6) {
        spawn_vehicle(&mut commands, &mut meshes, &mut materials, Vec3::new(25.0, 1.0, 0.0), VehicleType::Hovercraft);
        info!("Spawned Hovercraft");
    }

    // Remove all vehicles
    if keyboard_input.just_pressed(KeyCode::Delete) {
        for (vehicle_entity, _) in vehicle_query.iter() {
            commands.entity(vehicle_entity).despawn(); // Use simple despawn if recursive is troublesome
        }
        info!("Removed all vehicles");
    }
}

/// System to control vehicles
fn vehicle_controls_system(
    mut vehicle_query: Query<(&mut Vehicle, &mut InputState)>,
    character_query: Query<(Entity, &InputState, &CharacterMovementState), With<Player>>,
) {
    for (_vehicle, mut v_input) in vehicle_query.iter_mut() {
        for (_entity, input, state) in character_query.iter() {
            if state.is_in_vehicle && state.vehicle_entity.is_some() {
                // Character is in a vehicle, pass input to vehicle
                v_input.movement = input.movement;
                v_input.jump_pressed = input.jump_pressed;
                v_input.interact_pressed = input.interact_pressed;
            }
        }
    }
}

/// System to follow vehicle with camera
fn vehicle_camera_system(
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
    vehicle_query: Query<(&Transform, &Vehicle)>,
    character_query: Query<(&Transform, &CharacterMovementState), With<Player>>,
) {
    if let Some(mut camera_transform) = camera_query.iter_mut().next() {
        // Follow vehicle if character is in one
        for (_character_transform, state) in character_query.iter() {
            if state.is_in_vehicle && state.vehicle_entity.is_some() {
                if let Ok((vehicle_transform, _)) = vehicle_query.get(state.vehicle_entity.unwrap()) {
                    // Smooth camera follow
                    let target_pos = vehicle_transform.translation + *vehicle_transform.forward() * 10.0 + Vec3::Y * 5.0;
                    camera_transform.translation = camera_transform.translation.lerp(target_pos, 0.1);
                    camera_transform.look_at(vehicle_transform.translation, Vec3::Y);
                    return;
                }
            }
        }
        
        // If not in vehicle, follow character (basic)
        for (character_transform, _state) in character_query.iter() {
            let target_pos = character_transform.translation + Vec3::new(0.0, 5.0, 10.0);
            camera_transform.translation = camera_transform.translation.lerp(target_pos, 0.1);
            camera_transform.look_at(character_transform.translation, Vec3::Y);
        }
    }
}

/// System to display vehicle debug info
fn debug_vehicle_info_system(
    vehicle_query: Query<(&Vehicle, &Name)>,
    character_query: Query<(&CharacterMovementState, &Name), With<Player>>,
) {
    for (vehicle, name) in vehicle_query.iter() {
        println!("\n=== Vehicle: {} ===", name);
        println!("Type: {:?}", vehicle.vehicle_type);
        println!("Speed: {:.1} m/s", vehicle.current_speed);
        println!("RPM: {:.0}", vehicle.current_rpm);
        println!("Gear: {}", vehicle.current_gear);
        println!("Boosting: {}", vehicle.is_boosting);
        println!("Jumping: {}", vehicle.is_jumping);
        println!("Driving: {}", vehicle.is_driving);
        println!("Turned On: {}", vehicle.is_turned_on);
    }

    for (state, name) in character_query.iter() {
        println!("\n=== Character: {} ===", name);
        println!("In Vehicle: {}", state.is_in_vehicle);
        if let Some(vehicle_entity) = state.vehicle_entity {
            println!("Vehicle Entity: {:?}", vehicle_entity);
        }
    }
}

// Commented out due to resolution issues in this environment
/*
/// System to exit on ESC
fn exit_on_esc_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit_events: ResMut<Events<AppExit>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_events.send(AppExit::Success);
    }
}
*/

/// Helper function to spawn a vehicle
fn spawn_vehicle(
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
