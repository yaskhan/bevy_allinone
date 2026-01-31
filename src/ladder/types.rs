use bevy::prelude::*;

/// Ladder direction component for ladder objects
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct LadderDirection {
    pub direction: Vec3,
    pub raycast_direction: Vec3,
}

impl Default for LadderDirection {
    fn default() -> Self {
        Self {
            direction: Vec3::Y,
            raycast_direction: Vec3::Y,
        }
    }
}

/// Ladder end detection component
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct LadderEndDetection {
    pub layer_mask: u32,
    pub check_distance: f32,
    pub check_offset: Vec3,
}

impl Default for LadderEndDetection {
    fn default() -> Self {
        Self {
            layer_mask: 1,
            check_distance: 2.0,
            check_offset: Vec3::new(0.0, 0.1, 0.0),
        }
    }
}

/// Ladder movement state
#[derive(Debug, Clone, Copy, PartialEq, Reflect, Default)]
pub enum LadderMovementState {
    #[default]
    None,
    Approaching,
    Mounting,
    ClimbingUp,
    ClimbingDown,
    ClimbingHorizontal,
    Dismounting,
}

/// Component to track ladder movement state
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct LadderMovementTracker {
    pub current_state: LadderMovementState,
    pub previous_state: LadderMovementState,
    pub state_timer: f32,
    pub mount_duration: f32,
    pub dismount_duration: f32,
    pub climb_speed: f32,
    pub horizontal_climb_speed: f32,
}

impl Default for LadderMovementTracker {
    fn default() -> Self {
        Self {
            current_state: LadderMovementState::None,
            previous_state: LadderMovementState::None,
            state_timer: 0.0,
            mount_duration: 0.3,
            dismount_duration: 0.3,
            climb_speed: 5.0,
            horizontal_climb_speed: 3.0,
        }
    }
}

/// Component for ladder mount/dismount animation
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct LadderAnimation {
    pub is_mounting: bool,
    pub is_dismounting: bool,
    pub mount_progress: f32,
    pub dismount_progress: f32,
    pub mount_duration: f32,
    pub dismount_duration: f32,
    pub mount_start_position: Vec3,
    pub mount_target_position: Vec3,
    pub dismount_start_position: Vec3,
    pub dismount_target_position: Vec3,
}

impl Default for LadderAnimation {
    fn default() -> Self {
        Self {
            is_mounting: false,
            is_dismounting: false,
            mount_progress: 0.0,
            dismount_progress: 0.0,
            mount_duration: 0.3,
            dismount_duration: 0.3,
            mount_start_position: Vec3::ZERO,
            mount_target_position: Vec3::ZERO,
            dismount_start_position: Vec3::ZERO,
            dismount_target_position: Vec3::ZERO,
        }
    }
}

/// Component for ladder movement control
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct LadderMovement {
    pub is_active: bool,
    pub movement_direction: Vec3,
    pub vertical_input: f32,
    pub horizontal_input: f32,
    pub movement_speed: f32,
    pub vertical_movement_amount: f32,
    pub horizontal_movement_amount: f32,
    pub move_in_ladder_center: bool,
    pub use_horizontal_movement: bool,
    pub use_local_direction: bool,
    pub min_angle_vertical: f32,
    pub max_angle_vertical: f32,
    pub min_angle_to_inverse: f32,
}

impl Default for LadderMovement {
    fn default() -> Self {
        Self {
            is_active: false,
            movement_direction: Vec3::ZERO,
            vertical_input: 0.0,
            horizontal_input: 0.0,
            movement_speed: 5.0,
            vertical_movement_amount: 0.3,
            horizontal_movement_amount: 0.1,
            move_in_ladder_center: false,
            use_horizontal_movement: false,
            use_local_direction: false,
            min_angle_vertical: 60.0,
            max_angle_vertical: 120.0,
            min_angle_to_inverse: 100.0,
        }
    }
}

/// Component for ladder exit detection
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct LadderExitDetection {
    pub end_detected: bool,
    pub start_detected: bool,
    pub end_check_distance: f32,
    pub start_check_distance: f32,
    pub end_check_offset: Vec3,
    pub start_check_offset: Vec3,
    pub layer_mask: u32,
}

impl Default for LadderExitDetection {
    fn default() -> Self {
        Self {
            end_detected: false,
            start_detected: false,
            end_check_distance: 2.0,
            start_check_distance: 0.13,
            end_check_offset: Vec3::ZERO,
            start_check_offset: Vec3::new(0.0, 0.1, 0.0),
            layer_mask: 1,
        }
    }
}

/// Component for ladder footstep state
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct LadderFootstep {
    pub climb_state_name: String,
    pub is_climbing: bool,
    pub step_interval: f32,
    pub step_timer: f32,
}

impl Default for LadderFootstep {
    fn default() -> Self {
        Self {
            climb_state_name: "Climb Ladders".to_string(),
            is_climbing: false,
            step_interval: 0.5,
            step_timer: 0.0,
        }
    }
}

/// Event for when player enters a ladder
#[derive(Event, Debug, Reflect)]
pub struct LadderEnterEvent {
    pub player_entity: Entity,
    pub ladder_entity: Entity,
    pub ladder_transform: Transform,
}

/// Event for when player exits a ladder
#[derive(Event, Debug, Reflect)]
pub struct LadderExitEvent {
    pub player_entity: Entity,
    pub ladder_entity: Entity,
}

/// Event for when player starts climbing ladder
#[derive(Event, Debug, Reflect)]
pub struct LadderClimbStartEvent {
    pub player_entity: Entity,
    pub ladder_entity: Entity,
}

/// Event for when player stops climbing ladder
#[derive(Event, Debug, Reflect)]
pub struct LadderClimbStopEvent {
    pub player_entity: Entity,
    pub ladder_entity: Entity,
}
