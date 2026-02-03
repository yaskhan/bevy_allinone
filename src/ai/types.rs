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
    pub suspicion_timer: f32,
    pub max_suspicion_time: f32,
    pub wander_radius: f32,
    pub wander_center: Vec3,
    pub target_last_position: Option<Vec3>,
    pub is_paused: bool,
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
            suspicion_timer: 0.0,
            max_suspicion_time: 5.0,
            wander_radius: 10.0,
            wander_center: Vec3::ZERO,
            target_last_position: None,
            is_paused: false,
        }
    }
}

#[derive(Component, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct AiMovement {
    pub destination: Option<Vec3>,
    pub speed: f32,
    pub acceleration: f32,
    pub stop_distance: f32,
    pub move_type: AiMovementType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect, Default)]
pub enum AiMovementType {
    #[default]
    Walk,
    Run,
    Sprint,
    Crouch,
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
    Combat,
    Turret,
    Dead,
    Wander,
    Suspect,
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
    pub follow_distance: f32,
    pub stop_distance: f32,
}

impl Default for FriendManager {
    fn default() -> Self {
        Self {
            friends: Vec::new(),
            current_command: AiCommand::Follow,
            follow_distance: 5.0,
            stop_distance: 2.0,
        }
    }
}

#[derive(Resource, Debug, Reflect, Default)]
#[reflect(Resource)]
pub struct FriendSystem {
    pub friend_entities: Vec<Entity>,
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

#[derive(Component, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct AiCombatBrain {
    pub strategy: AiCombatStrategy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub enum AiCombatStrategy {
    CloseCombat,
    MeleeAdvanced,
    Ranged,
    Powers,
}

impl Default for AiCombatStrategy {
    fn default() -> Self {
        AiCombatStrategy::Ranged
    }
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct AiRangedCombatSettings {
    pub burst_size: u8,
    pub burst_interval: f32,
    pub burst_cooldown: f32,
    pub fire_timer: f32,
    pub burst_remaining: u8,
    pub reload_time: f32,
    pub reload_timer: f32,
    pub clip_size: i32,
    pub ammo_in_clip: i32,
    pub accuracy: f32,
    pub aim_time: f32,
    pub aim_timer: f32,
}

impl Default for AiRangedCombatSettings {
    fn default() -> Self {
        Self {
            burst_size: 3,
            burst_interval: 0.12,
            burst_cooldown: 0.6,
            fire_timer: 0.0,
            burst_remaining: 0,
            reload_time: 1.6,
            reload_timer: 0.0,
            clip_size: 30,
            ammo_in_clip: 30,
            accuracy: 0.85,
            aim_time: 0.15,
            aim_timer: 0.0,
        }
    }
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct AiMeleeCombatSettings {
    pub min_time_between_attacks: f32,
    pub last_attack_time: f32,
    pub block_probability: f32,
    pub parry_probability: f32,
    pub combo_probability: f32,
}

impl Default for AiMeleeCombatSettings {
    fn default() -> Self {
        Self {
            min_time_between_attacks: 0.6,
            last_attack_time: 0.0,
            block_probability: 0.2,
            parry_probability: 0.1,
            combo_probability: 0.25,
        }
    }
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct AiCloseCombatSettings {
    pub min_time_between_attacks: f32,
    pub last_attack_time: f32,
}

impl Default for AiCloseCombatSettings {
    fn default() -> Self {
        Self {
            min_time_between_attacks: 0.8,
            last_attack_time: 0.0,
        }
    }
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct AiPowersCombatSettings {
    pub ability_name: String,
    pub cooldown: f32,
    pub last_cast_time: f32,
    pub min_range: f32,
    pub max_range: f32,
}

impl Default for AiPowersCombatSettings {
    fn default() -> Self {
        Self {
            ability_name: "Ability".to_string(),
            cooldown: 2.0,
            last_cast_time: 0.0,
            min_range: 2.0,
            max_range: 20.0,
        }
    }
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
    pub factions: Vec<FactionInfo>,
    pub relations: Vec<FactionRelationInfo>,
}

#[derive(Debug, Clone, Reflect, Default)]
pub struct FactionRelationInfo {
    pub faction_a: String,
    pub faction_b: String,
    pub relation: FactionRelation,
}

#[derive(Debug, Clone, Reflect, Default)]
pub struct FactionInfo {
    pub name: String,
    pub turn_to_enemy_if_attacked: bool,
    pub turn_faction_to_enemy: bool,
    pub friendly_fire_turn_into_enemies: bool,
}

impl FactionSystem {
    pub fn get_relation(&self, f1: &str, f2: &str) -> FactionRelation {
        if f1 == f2 { return FactionRelation::Friend; }
        for rel_info in &self.relations {
            if (rel_info.faction_a == f1 && rel_info.faction_b == f2) || (rel_info.faction_a == f2 && rel_info.faction_b == f1) {
                return rel_info.relation;
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

#[derive(Component, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct PatrolPath {
    pub waypoints: Vec<Vec3>,
    pub loop_path: bool,
}

#[derive(Component, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct AIPerceptionSettings {
    pub fov: f32,
    pub range: f32,
    pub hearing_range: f32,
    pub layer_mask: u32,
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct AiHearingSettings {
    pub enabled: bool,
    pub min_decibels: f32,
    pub investigate_only_if_idle: bool,
}

impl Default for AiHearingSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            min_decibels: 0.1,
            investigate_only_if_idle: true,
        }
    }
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct AiCombatRangeSettings {
    pub min_distance_to_draw: f32,
    pub min_distance_to_shoot: f32,
    pub melee_range: f32,
    pub ranged_range: f32,
}

impl Default for AiCombatRangeSettings {
    fn default() -> Self {
        Self {
            min_distance_to_draw: 3.0,
            min_distance_to_shoot: 6.0,
            melee_range: 2.0,
            ranged_range: 15.0,
        }
    }
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct AiAlertSettings {
    pub enabled: bool,
    pub radius: f32,
    pub cooldown: f32,
    pub last_alert_time: f32,
}

impl Default for AiAlertSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            radius: 12.0,
            cooldown: 1.5,
            last_alert_time: -999.0,
        }
    }
}

#[derive(Debug, Reflect, Clone)]
pub struct NoiseEvent {
    pub position: Vec3,
    pub volume: f32,
    pub source: Entity,
}

#[derive(Resource, Default)]
pub struct NoiseEventQueue(pub Vec<NoiseEvent>);
