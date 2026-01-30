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
    Hide,
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

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct AiVisionVisualizer {
    pub active: bool,
    pub normal_color: Color,
    pub alert_color: Color,
}

impl Default for AiVisionVisualizer {
    fn default() -> Self {
        Self {
            active: true,
            normal_color: Color::WHITE,
            alert_color: Color::srgb(1.0, 0.0, 0.0),
        }
    }
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct AiStateVisuals {
    pub show_state_icons: bool,
    pub icon_offset: Vec3,
    // Timers for spawning icons could go here or be stateless based on events
    pub last_icon_spawn_time: f32,
    pub icon_spawn_interval: f32,
}

impl Default for AiStateVisuals {
    fn default() -> Self {
        Self {
            show_state_icons: true,
            icon_offset: Vec3::new(0.0, 2.0, 0.0),
            last_icon_spawn_time: 0.0,
            icon_spawn_interval: 1.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect, Default)]
pub enum FactionRelation {
    #[default]
    Neutral,
    Friend,
    Enemy,
}

#[derive(Resource, Debug, Reflect, Default)]
#[reflect(Resource)]
pub struct FactionSystem {
    pub relations: Vec<(String, String, FactionRelation)>,
}

impl FactionSystem {
    pub fn get_relation(&self, f1: &str, f2: &str) -> FactionRelation {
        if f1 == f2 { return FactionRelation::Friend; }
        for (a, b, rel) in &self.relations {
            if (a == f1 && b == f2) || (a == f2 && b == f1) {
                return *rel;
            }
        }
        FactionRelation::Neutral
    }
}

#[derive(Component, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct CharacterFaction {
    pub name: String,
}

#[derive(Component, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct HidePosition;
