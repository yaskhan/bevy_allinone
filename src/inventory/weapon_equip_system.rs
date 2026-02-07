use bevy::prelude::*;
use std::collections::HashMap;

use crate::weapons::{WeaponBundle, WeaponManager, Weapon, WeaponType};

#[derive(Event, Debug, Clone)]
pub struct RequestEquipWeaponEvent {
    pub owner: Entity,
    pub weapon_id: String,
    pub hand_preference: Option<crate::inventory::types::HandType>,
}

#[derive(Debug, Clone)]
pub struct WeaponSpawnInfo {
    pub weapon_name: String,
    pub weapon_type: WeaponType,
    pub damage: f32,
    pub fire_rate: f32,
    pub ammo_capacity: i32,
    pub current_ammo: i32,
    pub is_automatic: bool,
    pub ammo_name: String,
}

impl Default for WeaponSpawnInfo {
    fn default() -> Self {
        Self {
            weapon_name: "Generic Gun".to_string(),
            weapon_type: WeaponType::Pistol,
            damage: 10.0,
            fire_rate: 0.1,
            ammo_capacity: 30,
            current_ammo: 30,
            is_automatic: false,
            ammo_name: "Generic Ammo".to_string(),
        }
    }
}

#[derive(Resource, Default)]
pub struct WeaponSpawnRegistry {
    pub weapons: HashMap<String, WeaponSpawnInfo>,
}

pub fn handle_request_equip_weapon(
    mut commands: Commands,
    mut events: EventReader<RequestEquipWeaponEvent>,
    mut manager_query: Query<&mut WeaponManager>,
    mut weapon_query: Query<&mut Weapon>,
    registry: Res<WeaponSpawnRegistry>,
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

        let index = if let Some(index) = found_index {
            index
        } else {
            let weapon_entity = spawn_weapon_for_owner(
                &mut commands,
                event.owner,
                &event.weapon_id,
                &registry,
            );
            manager.weapons_list.push(weapon_entity);
            manager.weapons_list.len().saturating_sub(1)
        };

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

fn spawn_weapon_for_owner(
    commands: &mut Commands,
    owner: Entity,
    weapon_id: &str,
    registry: &WeaponSpawnRegistry,
) -> Entity {
    let info = registry
        .weapons
        .get(weapon_id)
        .cloned()
        .unwrap_or_else(|| {
            let mut fallback = WeaponSpawnInfo::default();
            fallback.weapon_name = weapon_id.to_string();
            fallback
        });

    let mut weapon = Weapon::default();
    weapon.weapon_name = info.weapon_name.clone();
    weapon.weapon_type = info.weapon_type;
    weapon.damage = info.damage;
    weapon.fire_rate = info.fire_rate;
    weapon.ammo_capacity = info.ammo_capacity;
    weapon.current_ammo = info.current_ammo;
    weapon.is_automatic = info.is_automatic;
    weapon.ammo_name = info.ammo_name.clone();

    let entity = commands
        .spawn(WeaponBundle {
            weapon,
            name: Name::new(info.weapon_name),
            ..default()
        })
        .id();

    commands.entity(entity).set_parent(owner);
    entity
}
