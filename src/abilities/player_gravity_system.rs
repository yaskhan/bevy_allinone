use bevy::prelude::*;
use crate::physics::CustomGravity;
use super::ability_info::AbilityInfo;

/// Player gravity ability.
///
///
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct PlayerGravityAbility {
    pub ability_name: String,
    pub gravity_system_enabled: bool,
}

impl Default for PlayerGravityAbility {
    fn default() -> Self {
        Self {
            ability_name: "PlayerGravity".to_string(),
            gravity_system_enabled: true,
        }
    }
}

pub fn update_player_gravity_ability(
    mut query: Query<(&AbilityInfo, &mut PlayerGravityAbility, Option<&mut CustomGravity>)>,
) {
    for (ability, mut gravity_ability, custom_gravity) in query.iter_mut() {
        if ability.name != gravity_ability.ability_name {
            continue;
        }

        if !ability.enabled {
            gravity_ability.gravity_system_enabled = false;
        }

        if let Some(mut gravity) = custom_gravity {
            gravity.multiplier = if gravity_ability.gravity_system_enabled { 1.0 } else { 0.0 };
        }
    }
}
