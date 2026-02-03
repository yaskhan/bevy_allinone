use bevy::prelude::*;

use crate::inventory::{Inventory, types::{InventoryItem, ItemType}};
use crate::weapons::{Weapon, WeaponManager};

use super::events::PickupEventQueue;
use super::melee_weapon_consumable_pickup::MeleeWeaponConsumablePickup;
use super::melee_shield_pickup::MeleeShieldPickup;
use super::melee_weapon_pickup::MeleeWeaponPickup;
use super::weapon_pickup::WeaponPickup;

pub fn process_pickup_events(
    mut commands: Commands,
    mut events: ResMut<PickupEventQueue>,
    mut weapon_manager_query: Query<&mut WeaponManager>,
    mut weapon_query: Query<(Entity, &mut Weapon, Option<&mut Visibility>)>,
    mut inventory_query: Query<&mut Inventory>,
    weapon_pickup_query: Query<&WeaponPickup>,
    melee_weapon_pickup_query: Query<&MeleeWeaponPickup>,
    melee_weapon_consumable_pickup_query: Query<&MeleeWeaponConsumablePickup>,
    melee_shield_pickup_query: Query<&MeleeShieldPickup>,
) {
    for event in events.0.drain(..) {
        let mut picked = false;

        if let Ok(pickup) = weapon_pickup_query.get(event.target) {
            picked = handle_weapon_pickup(
                &mut commands,
                event.source,
                event.target,
                pickup,
                &mut weapon_manager_query,
                &mut weapon_query,
                &mut inventory_query,
            );
        }

        if !picked {
            if let Ok(pickup) = melee_weapon_pickup_query.get(event.target) {
                picked = handle_melee_weapon_pickup(event.source, pickup, &mut inventory_query);
            }
        }

        if !picked {
            if let Ok(pickup) = melee_weapon_consumable_pickup_query.get(event.target) {
                picked = handle_melee_weapon_consumable_pickup(event.source, pickup, &mut inventory_query);
            }
        }

        if !picked {
            if let Ok(pickup) = melee_shield_pickup_query.get(event.target) {
                picked = handle_melee_shield_pickup(event.source, pickup, &mut inventory_query);
            }
        }

        if picked {
            commands.entity(event.target).despawn_recursive();
        }
    }
}

fn handle_weapon_pickup(
    commands: &mut Commands,
    player: Entity,
    target: Entity,
    pickup: &WeaponPickup,
    weapon_manager_query: &mut Query<&mut WeaponManager>,
    weapon_query: &mut Query<(Entity, &mut Weapon, Option<&mut Visibility>)>,
    inventory_query: &mut Query<&mut Inventory>,
) -> bool {
    if pickup.store_picked_weapons_on_inventory {
        return store_weapon_in_inventory(player, pickup, inventory_query);
    }

    let Ok(mut manager) = weapon_manager_query.get_mut(player) else {
        return store_weapon_in_inventory(player, pickup, inventory_query);
    };

    if let Ok((_entity, mut weapon, mut visibility)) = weapon_query.get_mut(target) {
        weapon.enabled = true;
        if let Some(vis) = visibility.as_mut() {
            *vis = Visibility::Hidden;
        }
        if !manager.weapons_list.contains(&target) {
            manager.weapons_list.push(target);
        }
        return true;
    }

    if let Some(found_entity) = find_weapon_by_name(pickup, weapon_query) {
        if !manager.weapons_list.contains(&found_entity) {
            manager.weapons_list.push(found_entity);
        }
        return true;
    }

    store_weapon_in_inventory(player, pickup, inventory_query)
}

fn find_weapon_by_name(
    pickup: &WeaponPickup,
    weapon_query: &mut Query<(Entity, &mut Weapon, Option<&mut Visibility>)>,
) -> Option<Entity> {
    if pickup.weapon_name.is_empty() {
        return None;
    }

    for (entity, mut weapon, _) in weapon_query.iter_mut() {
        if weapon.weapon_name == pickup.weapon_name {
            weapon.enabled = true;
            return Some(entity);
        }
    }

    None
}

fn store_weapon_in_inventory(
    player: Entity,
    pickup: &WeaponPickup,
    inventory_query: &mut Query<&mut Inventory>,
) -> bool {
    let Ok(mut inventory) = inventory_query.get_mut(player) else {
        warn!("Weapon pickup missing inventory on {:?}", player);
        return false;
    };

    let item = InventoryItem {
        item_id: pickup.weapon_id.clone(),
        name: pickup.weapon_name.clone(),
        quantity: 1,
        max_stack: 1,
        weight: 0.0,
        item_type: ItemType::Weapon,
        icon_path: String::new(),
        value: 0.0,
        category: "Weapon".to_string(),
        min_level: 0,
        info: String::new(),
    };

    inventory.add_item(item).is_none()
}

fn handle_melee_weapon_pickup(
    player: Entity,
    pickup: &MeleeWeaponPickup,
    inventory_query: &mut Query<&mut Inventory>,
) -> bool {
    if !pickup.store_picked_weapons_on_inventory {
        warn!("Melee weapon pickup requires inventory storage. Storing by default.");
    }

    let Ok(mut inventory) = inventory_query.get_mut(player) else {
        warn!("Melee weapon pickup missing inventory on {:?}", player);
        return false;
    };

    let item = InventoryItem {
        item_id: pickup.weapon_id.clone(),
        name: pickup.weapon_name.clone(),
        quantity: 1,
        max_stack: 1,
        weight: 0.0,
        item_type: ItemType::Weapon,
        icon_path: String::new(),
        value: 0.0,
        category: "Melee Weapon".to_string(),
        min_level: 0,
        info: String::new(),
    };

    inventory.add_item(item).is_none()
}

fn handle_melee_weapon_consumable_pickup(
    player: Entity,
    pickup: &MeleeWeaponConsumablePickup,
    inventory_query: &mut Query<&mut Inventory>,
) -> bool {
    if !pickup.store_picked_weapons_on_inventory {
        warn!("Melee weapon consumable pickup requires inventory storage. Storing by default.");
    }

    let Ok(mut inventory) = inventory_query.get_mut(player) else {
        warn!("Melee weapon consumable pickup missing inventory on {:?}", player);
        return false;
    };

    let quantity = pickup.amount.max(1);
    let item = InventoryItem {
        item_id: pickup.weapon_consumable_name.clone(),
        name: pickup.weapon_consumable_name.clone(),
        quantity,
        max_stack: 99,
        weight: 0.0,
        item_type: ItemType::Consumable,
        icon_path: String::new(),
        value: 0.0,
        category: "Melee Consumable".to_string(),
        min_level: 0,
        info: String::new(),
    };

    inventory.add_item(item).is_none()
}

fn handle_melee_shield_pickup(
    player: Entity,
    pickup: &MeleeShieldPickup,
    inventory_query: &mut Query<&mut Inventory>,
) -> bool {
    if !pickup.store_picked_shields_on_inventory {
        warn!("Melee shield pickup requires inventory storage. Storing by default.");
    }

    let Ok(mut inventory) = inventory_query.get_mut(player) else {
        warn!("Melee shield pickup missing inventory on {:?}", player);
        return false;
    };

    let item = InventoryItem {
        item_id: pickup.shield_id.clone(),
        name: pickup.shield_name.clone(),
        quantity: 1,
        max_stack: 1,
        weight: 0.0,
        item_type: ItemType::Equipment,
        icon_path: String::new(),
        value: 0.0,
        category: "Shield".to_string(),
        min_level: 0,
        info: String::new(),
    };

    inventory.add_item(item).is_none()
}
