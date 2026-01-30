use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Debug, Reflect, Clone, Default)]
#[reflect(Component)]
pub struct PlayerExperience {
    pub current_level: u32,
    pub current_xp: u32,
    pub total_xp: u32,
    pub skill_points: u32,
    pub xp_multiplier: f32,
    pub xp_multiplier_timer: f32,
}

#[derive(Debug, Clone, Reflect, Serialize, Deserialize)]
pub struct ExperienceLevel {
    pub level_number: u32,
    pub xp_required: u32,
    pub skill_points_reward: u32,
    pub stat_rewards: Vec<StatReward>,
}

#[derive(Debug, Clone, Reflect, Serialize, Deserialize)]
pub struct StatReward {
    pub stat_name: String,
    pub amount: f32,
    pub is_bool: bool,
    pub bool_value: bool,
}

#[derive(Resource, Debug, Reflect, Clone, Default)]
#[reflect(Resource)]
pub struct ExperienceSettings {
    pub levels: Vec<ExperienceLevel>,
    pub max_level: Option<u32>,
    pub xp_multiplier_enabled: bool,
}

#[derive(Component, Debug, Reflect, Clone, Default)]
#[reflect(Component)]
pub struct ObjectExperience {
    pub xp_amount: u32,
    pub xp_range: Option<(u32, u32)>,
    pub skill_points: u32,
    pub skill_points_range: Option<(u32, u32)>,
}

#[derive(Event, Debug, Clone)]
pub struct ExperienceObtainedEvent {
    pub entity: Entity,
    pub amount: u32,
    pub source_position: Option<Vec3>,
}

#[derive(Resource, Default)]
pub struct ExperienceObtainedQueue(pub Vec<ExperienceObtainedEvent>);

#[derive(Event, Debug, Clone)]
pub struct LevelUpEvent {
    pub entity: Entity,
    pub new_level: u32,
}

#[derive(Resource, Default)]
pub struct LevelUpQueue(pub Vec<LevelUpEvent>);
