use bevy::prelude::*;

/// Player ladder system component (attached to player)
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PlayerLadderSystem {
    // Main Settings
    pub ladder_found: bool,
    pub ladder_movement_speed: f32,
    pub ladder_vertical_movement_amount: f32,
    pub ladder_horizontal_movement_amount: f32,

    pub min_angle_to_inverse_direction: f32,

    pub use_always_horizontal_movement_on_ladder: bool,
    pub use_always_local_movement_direction: bool,

    pub min_angle_vertical_direction: f32,
    pub max_angle_vertical_direction: f32,

    pub climb_ladder_foot_step_state_name: String,

    // Debug
    pub ladder_end_detected: bool,
    pub ladder_start_detected: bool,

    pub movement_direction: i32,
    pub ladder_vertical_input: f32,
    pub ladder_horizontal_input: f32,
    pub ladder_angle: f32,
    pub ladder_signed_angle: f32,

    pub current_vertical_input: f32,
    pub current_horizontal_input: f32,

    pub ladder_movement_direction: Vec3,

    pub moving_on_ladder: bool,
    pub moving_on_ladder_previously: bool,

    pub ladder_direction_transform: Option<Entity>,
    pub ladder_raycast_direction_transform: Option<Entity>,

    // Events Settings
    pub use_events_on_third_person: bool,

    // Internal state
    pub use_ladder_horizontal_movement: bool,
    pub move_in_ladder_center: bool,
    pub use_local_movement_direction: bool,

    pub current_ladder_system: Option<Entity>,
    pub previous_ladder_system: Option<Entity>,
}

impl Default for PlayerLadderSystem {
    fn default() -> Self {
        Self {
            ladder_found: false,
            ladder_movement_speed: 5.0,
            ladder_vertical_movement_amount: 0.3,
            ladder_horizontal_movement_amount: 0.1,
            min_angle_to_inverse_direction: 100.0,
            use_always_horizontal_movement_on_ladder: false,
            use_always_local_movement_direction: false,
            min_angle_vertical_direction: 60.0,
            max_angle_vertical_direction: 120.0,
            climb_ladder_foot_step_state_name: "Climb Ladders".to_string(),
            ladder_end_detected: false,
            ladder_start_detected: false,
            movement_direction: 1,
            ladder_vertical_input: 0.0,
            ladder_horizontal_input: 0.0,
            ladder_angle: 0.0,
            ladder_signed_angle: 0.0,
            current_vertical_input: 0.0,
            current_horizontal_input: 0.0,
            ladder_movement_direction: Vec3::ZERO,
            moving_on_ladder: false,
            moving_on_ladder_previously: false,
            ladder_direction_transform: None,
            ladder_raycast_direction_transform: None,
            use_events_on_third_person: false,
            use_ladder_horizontal_movement: false,
            move_in_ladder_center: false,
            use_local_movement_direction: false,
            current_ladder_system: None,
            previous_ladder_system: None,
        }
    }
}
