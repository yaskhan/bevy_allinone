use bevy::prelude::*;
use std::collections::HashMap;

use super::ammo_on_inventory::AmmoOnInventory;
use super::energy_on_inventory::EnergyOnInventory;
use super::health_on_inventory::HealthOnInventory;
use super::jetpack_fuel_on_inventory::JetpackFuelOnInventory;
use super::melee_weapon_on_inventory::MeleeWeaponOnInventory;
use super::object_on_inventory::ObjectOnInventory;
use super::oxygen_on_inventory::OxygenOnInventory;
use super::shield_on_inventory::ShieldOnInventory;
use super::stamina_on_inventory::StaminaOnInventory;
use super::vehicle_fuel_on_inventory::VehicleFuelOnInventory;
use super::weapon_on_inventory::WeaponOnInventory;

#[derive(Debug, Clone)]
pub enum ItemEffect {
    Heal { amount: f32 },
    RestoreStamina { amount: f32 },
    RestoreEnergy { amount: f32 },
    RestoreShield { amount: f32 },
    RestoreOxygen { amount: f32 },
    RestoreJetpackFuel { amount: f32 },
    RestoreVehicleFuel { amount: f32 },
    RestoreAmmo { ammo_type: String, amount: i32 },
    EquipWeapon { weapon_id: String },
    EquipMeleeWeapon { weapon_id: String },
}

#[derive(Resource, Default)]
pub struct ItemEffectRegistry {
    pub effects: HashMap<String, Vec<ItemEffect>>,
}

pub fn register_item_effects(
    mut registry: ResMut<ItemEffectRegistry>,
    query: Query<(
        &ObjectOnInventory,
        Option<&HealthOnInventory>,
        Option<&StaminaOnInventory>,
        Option<&EnergyOnInventory>,
        Option<&ShieldOnInventory>,
        Option<&OxygenOnInventory>,
        Option<&JetpackFuelOnInventory>,
        Option<&VehicleFuelOnInventory>,
        Option<&AmmoOnInventory>,
        Option<&WeaponOnInventory>,
        Option<&MeleeWeaponOnInventory>,
    ), Added<ObjectOnInventory>>,
) {
    for (
        object,
        health,
        stamina,
        energy,
        shield,
        oxygen,
        jetpack,
        vehicle_fuel,
        ammo,
        weapon,
        melee_weapon,
    ) in query.iter() {
        let mut effects = Vec::new();

        if let Some(info) = health {
            effects.push(ItemEffect::Heal { amount: info.amount });
        }
        if let Some(info) = stamina {
            effects.push(ItemEffect::RestoreStamina { amount: info.amount });
        }
        if let Some(info) = energy {
            effects.push(ItemEffect::RestoreEnergy { amount: info.amount });
        }
        if let Some(info) = shield {
            effects.push(ItemEffect::RestoreShield { amount: info.amount });
        }
        if let Some(info) = oxygen {
            effects.push(ItemEffect::RestoreOxygen { amount: info.amount });
        }
        if let Some(info) = jetpack {
            effects.push(ItemEffect::RestoreJetpackFuel { amount: info.amount });
        }
        if let Some(info) = vehicle_fuel {
            effects.push(ItemEffect::RestoreVehicleFuel { amount: info.amount });
        }
        if let Some(info) = ammo {
            effects.push(ItemEffect::RestoreAmmo {
                ammo_type: info.ammo_type.clone(),
                amount: info.amount,
            });
        }
        if let Some(info) = weapon {
            effects.push(ItemEffect::EquipWeapon {
                weapon_id: info.weapon_id.clone(),
            });
        }
        if let Some(info) = melee_weapon {
            effects.push(ItemEffect::EquipMeleeWeapon {
                weapon_id: info.weapon_id.clone(),
            });
        }

        if !effects.is_empty() {
            registry.effects.insert(object.item_id.clone(), effects);
        }
    }
}
