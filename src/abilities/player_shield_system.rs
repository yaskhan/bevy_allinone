use bevy::prelude::*;
use super::ability_info::AbilityInfo;

/// Player shield ability.
///
/// GKC reference: `playerShieldSystem.cs`
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PlayerShieldSystem {
    pub ability_name: String,
    pub shield_enabled: bool,
    pub shield_active: bool,
    pub laser_active: bool,
    pub laser_ability_enabled: bool,
    pub shield_entity: Option<Entity>,
    pub laser_entity: Option<Entity>,
}

impl Default for PlayerShieldSystem {
    fn default() -> Self {
        Self {
            ability_name: "Shield".to_string(),
            shield_enabled: true,
            shield_active: false,
            laser_active: false,
            laser_ability_enabled: true,
            shield_entity: None,
            laser_entity: None,
        }
    }
}

pub fn update_player_shield_system(
    mut query: Query<(&AbilityInfo, &mut PlayerShieldSystem)>,
    mut visibility_query: Query<&mut Visibility>,
) {
    for (ability, mut shield) in query.iter_mut() {
        if ability.name != shield.ability_name {
            continue;
        }

        if !ability.enabled {
            shield.shield_enabled = false;
            shield.shield_active = false;
            shield.laser_active = false;
        }

        if shield.shield_enabled {
            shield.shield_active = ability.active_from_press_down;
        }

        if let Some(shield_entity) = shield.shield_entity {
            if let Ok(mut visibility) = visibility_query.get_mut(shield_entity) {
                *visibility = if shield.shield_active || shield.laser_active {
                    Visibility::Visible
                } else {
                    Visibility::Hidden
                };
            }
        }

        if let Some(laser_entity) = shield.laser_entity {
            if let Ok(mut visibility) = visibility_query.get_mut(laser_entity) {
                *visibility = if shield.laser_active && shield.laser_ability_enabled {
                    Visibility::Visible
                } else {
                    Visibility::Hidden
                };
            }
        }
    }
}
