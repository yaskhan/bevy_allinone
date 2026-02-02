use bevy::prelude::*;
use crate::weapons::WeaponManager;
use super::ability_info::AbilityInfo;
use super::player_abilities::PlayerAbilitiesSystem;

/// Integration hooks between weapons and abilities.
///
/// GKC reference: `playerWeaponSystem.cs`, `playerWeaponsManager.cs`
#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct AbilityWeaponIntegration {
    pub enable_on_weapon: Vec<String>,
    pub disable_on_weapon: Vec<String>,
    pub last_weapon_equipped: bool,
}

/// Apply ability enable/disable lists when weapon equip state changes.
pub fn update_weapon_ability_hooks(
    mut player_query: Query<(&mut PlayerAbilitiesSystem, &WeaponManager, &mut AbilityWeaponIntegration)>,
    mut abilities: Query<&mut AbilityInfo>,
) {
    for (mut system, weapons, mut integration) in player_query.iter_mut() {
        let weapon_equipped = weapons.carrying_weapon_in_third_person || weapons.carrying_weapon_in_first_person;
        if weapon_equipped == integration.last_weapon_equipped {
            continue;
        }

        integration.last_weapon_equipped = weapon_equipped;

        if weapon_equipped {
            for name in &integration.enable_on_weapon {
                system.enable_ability_by_name(name, &mut abilities);
            }
            for name in &integration.disable_on_weapon {
                system.disable_ability_by_name(name, &mut abilities);
            }
        } else {
            for name in &integration.enable_on_weapon {
                system.disable_ability_by_name(name, &mut abilities);
            }
            for name in &integration.disable_on_weapon {
                system.enable_ability_by_name(name, &mut abilities);
            }
        }
    }
}
