use bevy::prelude::*;

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct AiController {
    pub state: AiBehaviorState,
    pub target: Option<Entity>,
    pub patrol_path: Vec<Vec3>,
    pub current_waypoint_index: usize,
    pub detection_range: f32,
    pub attack_range: f32,
    pub patrol_speed_mult: f32,
    pub chase_speed_mult: f32,
    pub wait_timer: f32,
    pub wait_time_between_waypoints: f32,
}

impl Default for AiController {
    fn default() -> Self {
        Self {
            state: AiBehaviorState::Idle,
            target: None,
            patrol_path: Vec::new(),
            current_waypoint_index: 0,
            detection_range: 15.0,
            attack_range: 2.5,
            patrol_speed_mult: 0.5,
            chase_speed_mult: 1.0,
            wait_timer: 0.0,
            wait_time_between_waypoints: 2.0,
        }
    }
}

/// AI behavior state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub enum AiBehaviorState {
    Idle,
    Patrol,
    Chase,
    Attack,
    Flee,
    Follow,
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct AiPerception {
    pub fov: f32,
    pub vision_range: f32,
    pub visible_targets: Vec<Entity>,
}

impl Default for AiPerception {
    fn default() -> Self {
        Self {
            fov: 90.0,
            vision_range: 20.0,
            visible_targets: Vec::new(),
        }
    }
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct FriendManager {
    pub friends: Vec<Entity>,
    pub current_command: AiCommand,
}

impl Default for FriendManager {
    fn default() -> Self {
        Self {
            friends: Vec::new(),
            current_command: AiCommand::Follow,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect, Default)]
pub enum AiCommand {
    #[default]
    Follow,
    Wait,
    Attack, // Target implies closest enemy for now
    Hide,
}
