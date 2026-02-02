use bevy::prelude::*;
use super::ability_info::AbilityInfo;

#[derive(Debug, Clone)]
pub struct PlayerStealthEvent {
    pub entity: Entity,
    pub active: bool,
}

#[derive(Resource, Default)]
pub struct PlayerStealthEventQueue(pub Vec<PlayerStealthEvent>);

/// Player stealth ability system.
///
/// GKC reference: `playerStealthSystem.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PlayerStealthSystem {
    pub ability_name: String,
    pub enabled: bool,
    pub active: bool,
    pub time_limit_timer: f32,
}

impl Default for PlayerStealthSystem {
    fn default() -> Self {
        Self {
            ability_name: "Stealth".to_string(),
            enabled: true,
            active: false,
            time_limit_timer: 0.0,
        }
    }
}

pub fn update_player_stealth_system(
    time: Res<Time>,
    mut events: ResMut<PlayerStealthEventQueue>,
    mut query: Query<(Entity, &AbilityInfo, &mut PlayerStealthSystem)>,
) {
    for (entity, ability, mut stealth) in query.iter_mut() {
        if ability.name != stealth.ability_name {
            continue;
        }

        if !ability.enabled {
            if stealth.active {
                stealth.active = false;
                events.0.push(PlayerStealthEvent { entity, active: false });
            }
            stealth.enabled = false;
            continue;
        }

        if ability.active_from_press_down && !stealth.active {
            stealth.active = true;
            stealth.time_limit_timer = if ability.use_time_limit { ability.time_limit } else { 0.0 };
            events.0.push(PlayerStealthEvent { entity, active: true });
        }

        if !ability.active_from_press_down && stealth.active {
            stealth.active = false;
            events.0.push(PlayerStealthEvent { entity, active: false });
        }

        if stealth.active && ability.use_time_limit {
            stealth.time_limit_timer -= time.delta_secs();
            if stealth.time_limit_timer <= 0.0 {
                stealth.active = false;
                events.0.push(PlayerStealthEvent { entity, active: false });
            }
        }
    }
}
