use bevy::prelude::*;
use crate::vehicles::types::*;
use crate::input::InputState;
use avian3d::prelude::*;

#[derive(Default)]
pub struct VehicleConfig {
    pub vehicle_type: VehicleType,
    pub position: Vec3,
    pub name: String,
    pub mesh_size: Vec3,
    pub color: Color,
    pub seats: Vec<(String, Vec3, bool)>, // Name, Offset, IsDriver
    pub wheels: Vec<(String, Vec3, bool, bool, bool)>, // Name, Offset, Steer, Power, LeftSide
}

impl VehicleConfig {
    pub fn new(vehicle_type: VehicleType, position: Vec3) -> Self {
        Self {
            vehicle_type: vehicle_type.clone(),
            position,
            name: format!("{:?}", vehicle_type),
            mesh_size: Vec3::new(2.0, 1.0, 4.0),
            color: Color::from(LinearRgba::new(0.8, 0.2, 0.2, 1.0)),
            seats: Vec::new(),
            wheels: Vec::new(),
        }
    }

    pub fn build(
        self,
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) -> Entity {
        let mut vehicle = Vehicle::default();
        vehicle.vehicle_type = self.vehicle_type.clone();
        vehicle.vehicle_name = self.name.clone();

        match self.vehicle_type {
            VehicleType::Aircraft => {
                vehicle.max_forward_speed = 60.0;
                vehicle.engine_torque = 5000.0;
                vehicle.lift_amount = 0.002;
            }
            VehicleType::Sphere => {
                vehicle.max_forward_speed = 20.0;
                vehicle.engine_torque = 3000.0;
                vehicle.move_speed_multiplier = 15.0;
            }
            _ => {}
        }

        let vehicle_entity = commands.spawn((
            Name::new(self.name.clone()),
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
            Mesh3d(meshes.add(Cuboid::new(self.mesh_size.x, self.mesh_size.y, self.mesh_size.z))),
            MeshMaterial3d(materials.add(self.color)),
            Transform::from_translation(self.position),
            GlobalTransform::default(),
            RigidBody::Dynamic,
            Collider::cuboid(self.mesh_size.x, self.mesh_size.y, self.mesh_size.z),
            LinearVelocity::default(),
            AngularVelocity::default(),
        )).id();

        let mut child_entities = Vec::new();

        for (w_name, w_offset, w_steer, w_power, w_left) in self.wheels {
            let wheel = commands.spawn((
                Name::new(w_name.clone()),
                VehicleWheel {
                    wheel_name: w_name,
                    is_steerable: w_steer,
                    is_powered: w_power,
                    is_left_side: w_left,
                    is_right_side: !w_left,
                    ..default()
                },
                Transform::from_translation(w_offset),
                GlobalTransform::default(),
            )).with_children(|p| {
                p.spawn((
                    Mesh3d(meshes.add(Cylinder::new(0.3, 0.2))),
                    MeshMaterial3d(materials.add(Color::BLACK)),
                    Transform::from_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
                ));
            }).id();
            child_entities.push(wheel);
        }

        let mut seat_entities = Vec::new();
        for (s_name, s_offset, s_is_driver) in self.seats {
            let seat = commands.spawn((
                Name::new(s_name),
                VehicleSeat {
                    seat_index: seat_entities.len(),
                    is_driver_seat: s_is_driver,
                    offset: s_offset,
                    ..default()
                },
                Transform::from_translation(s_offset),
                GlobalTransform::default(),
            )).id();
            seat_entities.push(seat);
            child_entities.push(seat);
        }

        commands.entity(vehicle_entity).insert(VehicleSeatingManager {
            seats: seat_entities,
            eject_on_destroy: true,
            eject_force: 15.0,
            ..default()
        });

        commands.entity(vehicle_entity).add_children(&child_entities);

        vehicle_entity
    }
}

pub fn spawn_vehicle(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    position: Vec3,
    vehicle_type: VehicleType,
) -> Entity {
    let mut config = VehicleConfig::new(vehicle_type.clone(), position);
    
    match vehicle_type {
        VehicleType::Car => {
            config.name = "Sports Car".to_string();
            config.wheels = vec![
                ("FL".to_string(), Vec3::new(-1.0, -0.5, 1.5), true, false, true),
                ("FR".to_string(), Vec3::new(1.0, -0.5, 1.5), true, false, false),
                ("RL".to_string(), Vec3::new(-1.0, -0.5, -1.5), false, true, true),
                ("RR".to_string(), Vec3::new(1.0, -0.5, -1.5), false, true, false),
            ];
            config.seats = vec![
                ("Driver".to_string(), Vec3::new(-0.5, 0.5, 0.0), true),
                ("Passenger".to_string(), Vec3::new(0.5, 0.5, 0.0), false),
            ];
        }
        VehicleType::Motorcycle => {
            config.name = "Motorcycle".to_string();
            config.mesh_size = Vec3::new(0.5, 1.0, 2.0);
            config.wheels = vec![
                ("Front".to_string(), Vec3::new(0.0, -0.5, 0.8), true, false, false),
                ("Back".to_string(), Vec3::new(0.0, -0.5, -0.8), false, true, false),
            ];
            config.seats = vec![
                ("Driver".to_string(), Vec3::new(0.0, 0.5, 0.0), true),
            ];
        }
        VehicleType::Sphere => {
            config.name = "Battle Sphere".to_string();
            config.mesh_size = Vec3::splat(2.0);
            config.seats = vec![
                ("Driver".to_string(), Vec3::ZERO, true),
            ];
        }
        VehicleType::Aircraft => {
            config.name = "Combat Plane".to_string();
            config.mesh_size = Vec3::new(4.0, 1.5, 6.0);
            config.seats = vec![
                ("Pilot".to_string(), Vec3::new(0.0, 0.5, 1.0), true),
                ("Gunner".to_string(), Vec3::new(0.0, 0.5, -1.0), false),
            ];
        }
        _ => {
            config.wheels = vec![
                ("FL".to_string(), Vec3::new(-1.0, -0.5, 1.5), true, false, true),
                ("FR".to_string(), Vec3::new(1.0, -0.5, 1.5), true, false, false),
                ("RL".to_string(), Vec3::new(-1.0, -0.5, -1.5), false, true, true),
                ("RR".to_string(), Vec3::new(1.0, -0.5, -1.5), false, true, false),
            ];
            config.seats = vec![
                ("Driver".to_string(), Vec3::new(-0.5, 0.5, 0.0), true),
            ];
        }
    }

    config.build(commands, &mut meshes, &mut materials)
}
