use bevy::prelude::*;

use crate::weapons::{Weapon, WeaponManager};
use super::components::Inventory;
use super::types::ItemType;

pub fn sync_weapon_ammo_with_inventory(
    inventory_query: Query<&Inventory>,
    mut manager_query: Query<(Entity, &mut WeaponManager)>,
    mut weapon_query: Query<&mut Weapon>,
) {
    for (owner, mut manager) in manager_query.iter_mut() {
        let Ok(inventory) = inventory_query.get(owner) else { continue };

        for weapon_entity in manager.weapons_list.iter() {
            let Ok(mut weapon) = weapon_query.get_mut(*weapon_entity) else { continue };
            if !manager.use_ammo_from_inventory {
                continue;
            }

            let reserve = inventory
                .items
                .iter()
                .flatten()
                .filter(|item| item.item_type == ItemType::Ammo)
                .filter(|item| item.name == weapon.ammo_name || item.item_id == weapon.ammo_name)
                .map(|item| item.quantity)
                .sum::<i32>();

            weapon.reserve_ammo = reserve.max(0);
        }
    }
}
