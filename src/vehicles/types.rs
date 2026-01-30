use bevy::prelude::*;

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
    pub min_rpm: f32,
    pub max_rpm: f32,
    pub gear_shift_rate: f32,
    pub is_turned_on: bool,
    pub is_driving: bool,
    pub is_reversing: bool,
    pub is_braking: bool,
    pub is_on_ground: bool,
    pub is_boosting: bool,
    pub is_jumping: bool,
    pub changing_gear: bool,

    // Input
    pub motor_input: f32,
    pub steer_input: f32,
    pub steer_input_speed: f32,
    pub boost_input: f32,

    // Internal
    pub current_steering: f32,
    pub chassis_lean_x: f32,
    pub chassis_lean_y: f32,
    pub time_to_stabilize: f32,
    pub reset_timer: f32,
    pub using_gravity_control: bool,
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
            min_rpm: 1000.0,
            max_rpm: 6000.0,
            gear_shift_rate: 10.0,
            is_turned_on: true,
            is_driving: false,
            is_reversing: false,
            is_braking: false,
            is_on_ground: true,
            is_boosting: false,
            is_jumping: false,
            changing_gear: false,
            motor_input: 0.0,
            steer_input: 0.0,
            steer_input_speed: 5.0,
            boost_input: 1.0,
            current_steering: 0.0,
            chassis_lean_x: 0.0,
            chassis_lean_y: 0.0,
            time_to_stabilize: 0.0,
            reset_timer: 0.0,
            using_gravity_control: false,
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

    pub mudguard_offset: Vec3,
    pub suspension_offset: Vec3,
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
            mudguard_offset: Vec3::ZERO,
            suspension_offset: Vec3::ZERO,
        }
    }
}

/// Vehicle gear system
#[derive(Component, Debug, Reflect, Clone)]
#[reflect(Component)]
pub struct VehicleGear {
    pub gear_name: String,
    pub gear_speed: f32,
    pub torque_curve: Vec<f32>,
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
