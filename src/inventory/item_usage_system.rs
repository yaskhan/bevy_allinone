use bevy::prelude::*;

use crate::combat::{Health, Shield};
use crate::stats::{DerivedStat, StatsSystem};
use crate::abilities::OxygenSystem;
use crate::player::extra_movements::jetpack::Jetpack;
use crate::vehicles::{VehicleStats};
use crate::weapons::{WeaponManager, Weapon};

use super::components::Inventory;
use super::item_effects::{ItemEffectRegistry, ItemEffect};
use super::use_inventory_object::{UseInventoryObjectEvent, InventoryObjectUsedEvent};
use super::melee_weapon_equipment_system::EquipMeleeWeaponEvent;
use super::weapon_equip_system::RequestEquipWeaponEvent;
use crate::character::CharacterMovementState;

pub fn apply_inventory_item_effects(
    mut use_events: EventReader<UseInventoryObjectEvent>,
    mut used_events: EventWriter<InventoryObjectUsedEvent>,
    mut equip_events: EventWriter<EquipMeleeWeaponEvent>,
    mut request_weapon_equip: EventWriter<RequestEquipWeaponEvent>,
    registry: Res<ItemEffectRegistry>,
    mut inventories: Query<&mut Inventory>,
    mut health_query: Query<&mut Health>,
    mut shield_query: Query<&mut Shield>,
    mut stats_query: Query<&mut StatsSystem>,
    mut oxygen_query: Query<&mut OxygenSystem>,
    mut jetpack_query: Query<&mut Jetpack>,
    mut vehicle_stats_query: Query<&mut VehicleStats>,
    mut movement_query: Query<&CharacterMovementState>,
    mut weapon_manager_query: Query<&mut WeaponManager>,
    mut weapon_query: Query<&mut Weapon>,
) {
    for event in use_events.read() {
        let Ok(mut inventory) = inventories.get_mut(event.owner) else { continue };

        let (slot_index, mut item) = match inventory.items.iter_mut().enumerate().find_map(|(i, slot)| {
            slot.as_mut().and_then(|item| {
                if item.item_id == event.item_id { Some((i, item.clone())) } else { None }
            })
        }) {
            Some(found) => found,
            None => continue,
        };

        if item.quantity <= 0 {
            continue;
        }

        let mut desired = event.quantity;
        if desired <= 0 {
            desired = 1;
        }
        let quantity = desired.min(item.quantity);
        let effects = match registry.effects.get(&item.item_id) {
            Some(effects) => effects.clone(),
            None => Vec::new(),
        };

        apply_effects(
            event.owner,
            &effects,
            quantity,
            &mut health_query,
            &mut shield_query,
            &mut stats_query,
            &mut oxygen_query,
            &mut jetpack_query,
            &mut vehicle_stats_query,
            &mut movement_query,
            &mut weapon_manager_query,
            &mut weapon_query,
            &mut equip_events,
            &mut request_weapon_equip,
        );

        if let Some(slot) = inventory.items.get_mut(slot_index) {
            if let Some(stored) = slot {
                stored.quantity -= quantity;
                if stored.quantity <= 0 {
                    *slot = None;
                }
            }
        }

        inventory.recalculate_weight();
        item.quantity = quantity;
        used_events.send(InventoryObjectUsedEvent {
            owner: event.owner,
            item,
        });
    }
}

#[allow(clippy::too_many_arguments)]
fn apply_effects(
    owner: Entity,
    effects: &[ItemEffect],
    quantity: i32,
    health_query: &mut Query<&mut Health>,
    shield_query: &mut Query<&mut Shield>,
    stats_query: &mut Query<&mut StatsSystem>,
    oxygen_query: &mut Query<&mut OxygenSystem>,
    jetpack_query: &mut Query<&mut Jetpack>,
    vehicle_stats_query: &mut Query<&mut VehicleStats>,
    movement_query: &mut Query<&CharacterMovementState>,
    weapon_manager_query: &mut Query<&mut WeaponManager>,
    weapon_query: &mut Query<&mut Weapon>,
    equip_events: &mut EventWriter<EquipMeleeWeaponEvent>,
    request_weapon_equip: &mut EventWriter<RequestEquipWeaponEvent>,
) {
    let amount_mult = quantity as f32;
    for effect in effects {
        match effect {
            ItemEffect::Heal { amount } => {
                if let Ok(mut health) = health_query.get_mut(owner) {
                    health.current = (health.current + amount * amount_mult).min(health.maximum);
                }
                if let Ok(mut stats) = stats_query.get_mut(owner) {
                    stats.increase_derived_stat(DerivedStat::CurrentHealth, amount * amount_mult);
                }
            }
            ItemEffect::RestoreStamina { amount } => {
                if let Ok(mut stats) = stats_query.get_mut(owner) {
                    stats.increase_derived_stat(DerivedStat::CurrentStamina, amount * amount_mult);
                }
            }
            ItemEffect::RestoreEnergy { amount } => {
                if let Ok(mut stats) = stats_query.get_mut(owner) {
                    stats.increase_derived_stat(DerivedStat::CurrentMana, amount * amount_mult);
                }
            }
            ItemEffect::RestoreShield { amount } => {
                if let Ok(mut shield) = shield_query.get_mut(owner) {
                    shield.current = (shield.current + amount * amount_mult).min(shield.maximum);
                }
            }
            ItemEffect::RestoreOxygen { amount } => {
                if let Ok(mut oxygen) = oxygen_query.get_mut(owner) {
                    oxygen.current_oxygen = (oxygen.current_oxygen + amount * amount_mult).min(oxygen.max_oxygen);
                }
            }
            ItemEffect::RestoreJetpackFuel { amount } => {
                if let Ok(mut jetpack) = jetpack_query.get_mut(owner) {
                    jetpack.current_fuel = (jetpack.current_fuel + amount * amount_mult).min(jetpack.max_fuel);
                }
            }
            ItemEffect::RestoreVehicleFuel { amount } => {
                if let Ok(movement) = movement_query.get_mut(owner) {
                    if let Some(vehicle_entity) = movement.vehicle_entity {
                        if let Ok(mut stats) = vehicle_stats_query.get_mut(vehicle_entity) {
                            stats.fuel = (stats.fuel + amount * amount_mult).min(stats.max_fuel);
                        }
                    }
                }
            }
            ItemEffect::RestoreAmmo { ammo_type, amount } => {
                if let Ok(mut manager) = weapon_manager_query.get_mut(owner) {
                    let mut remaining = amount * quantity;
                    for weapon_entity in manager.weapons_list.iter() {
                        let Ok(mut weapon) = weapon_query.get_mut(*weapon_entity) else { continue };
                        if weapon.ammo_name != *ammo_type {
                            continue;
                        }
                        let needed = (weapon.ammo_capacity - weapon.current_ammo).max(0);
                        let add = remaining.min(needed);
                        if add > 0 {
                            weapon.current_ammo += add;
                            remaining -= add;
                        }
                        if remaining <= 0 {
                            break;
                        }
                    }
                }
            }
            ItemEffect::EquipWeapon { weapon_id } => {
                request_weapon_equip.send(RequestEquipWeaponEvent {
                    owner,
                    weapon_id: weapon_id.clone(),
                });
            }
            ItemEffect::EquipMeleeWeapon { weapon_id } => {
                equip_events.send(EquipMeleeWeaponEvent {
                    owner,
                    weapon_id: weapon_id.clone(),
                });
            }
        }
    }
}
