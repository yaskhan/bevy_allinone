use bevy::prelude::*;

use crate::weapons::{WeaponManager, Weapon};

#[derive(Event, Debug, Clone)]
pub struct RequestEquipWeaponEvent {
    pub owner: Entity,
    pub weapon_id: String,
}

pub fn handle_request_equip_weapon(
    mut events: EventReader<RequestEquipWeaponEvent>,
    mut manager_query: Query<&mut WeaponManager>,
    mut weapon_query: Query<&mut Weapon>,
) {
    for event in events.read() {
        let Ok(mut manager) = manager_query.get_mut(event.owner) else { continue };

        let mut found_index = None;
        for (index, weapon_entity) in manager.weapons_list.iter().enumerate() {
            let Ok(weapon) = weapon_query.get(*weapon_entity) else { continue };
            if weapon.weapon_name == event.weapon_id {
                found_index = Some(index);
                break;
            }
        }

        let Some(index) = found_index else { continue };

        for weapon_entity in manager.weapons_list.iter() {
            if let Ok(mut weapon) = weapon_query.get_mut(*weapon_entity) {
                weapon.equipped = false;
            }
        }

        if let Some(weapon_entity) = manager.weapons_list.get(index) {
            if let Ok(mut weapon) = weapon_query.get_mut(*weapon_entity) {
                weapon.equipped = true;
            }
            manager.current_index = index;
            manager.weapons_mode_active = true;
        }
    }
}
