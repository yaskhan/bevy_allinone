use bevy::prelude::*;

use crate::input::InputState;
use super::inventory_quick_access_slots_system::InventoryQuickAccessSlotsSystem;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub enum WeaponMountType {
    Hand,
    Holster,
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct WeaponMountPoint {
    pub slot: String,
    pub mount_type: WeaponMountType,
}

impl Default for WeaponMountPoint {
    fn default() -> Self {
        Self {
            slot: "hand".to_string(),
            mount_type: WeaponMountType::Hand,
        }
    }
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct MeleeWeaponEquipmentState {
    pub equipped_weapon_id: Option<String>,
    pub weapon_entity: Option<Entity>,
    pub drawn: bool,
    pub holster_slot: String,
    pub hand_slot: String,
}

impl Default for MeleeWeaponEquipmentState {
    fn default() -> Self {
        Self {
            equipped_weapon_id: None,
            weapon_entity: None,
            drawn: false,
            holster_slot: "holster".to_string(),
            hand_slot: "hand".to_string(),
        }
    }
}

#[derive(Event, Debug, Clone)]
pub struct EquipMeleeWeaponEvent {
    pub owner: Entity,
    pub weapon_id: String,
}

#[derive(Event, Debug, Clone)]
pub struct UnequipMeleeWeaponEvent {
    pub owner: Entity,
}

#[derive(Event, Debug, Clone)]
pub struct ToggleMeleeWeaponDrawEvent {
    pub owner: Entity,
}

pub fn handle_melee_weapon_quick_access_input(
    input: Res<InputState>,
    quick_access_query: Query<&InventoryQuickAccessSlotsSystem>,
    mut equip_events: EventWriter<EquipMeleeWeaponEvent>,
) {
    let Some(index) = input.select_weapon else { return };
    for system in quick_access_query.iter() {
        if let Some(item_id) = system.slots.get(index).and_then(|slot| slot.clone()) {
            equip_events.send(EquipMeleeWeaponEvent {
                owner: system.owner,
                weapon_id: item_id,
            });
        }
    }
}

pub fn handle_melee_weapon_equip(
    mut commands: Commands,
    mut equip_events: EventReader<EquipMeleeWeaponEvent>,
    mut unequip_events: EventReader<UnequipMeleeWeaponEvent>,
    mut state_query: Query<&mut MeleeWeaponEquipmentState>,
    mount_query: Query<(Entity, &WeaponMountPoint, &Parent)>,
) {
    for event in unequip_events.read() {
        if let Ok(mut state) = state_query.get_mut(event.owner) {
            if let Some(weapon_entity) = state.weapon_entity.take() {
                commands.entity(weapon_entity).despawn_recursive();
            }
            state.equipped_weapon_id = None;
            state.drawn = false;
        }
    }

    for event in equip_events.read() {
        let Ok(mut state) = state_query.get_mut(event.owner) else { continue };

        if let Some(weapon_entity) = state.weapon_entity.take() {
            commands.entity(weapon_entity).despawn_recursive();
        }

        state.equipped_weapon_id = Some(event.weapon_id.clone());
        state.drawn = false;

        let mount_entity = find_mount_point(
            event.owner,
            &state.holster_slot,
            WeaponMountType::Holster,
            &mount_query,
        );

        let weapon_entity = commands
            .spawn((
                Name::new(format!("MeleeWeapon {}", event.weapon_id)),
                Transform::default(),
                GlobalTransform::default(),
            ))
            .id();

        if let Some(mount) = mount_entity {
            commands.entity(weapon_entity).set_parent(mount);
        } else {
            commands.entity(weapon_entity).set_parent(event.owner);
        }

        state.weapon_entity = Some(weapon_entity);
    }
}

pub fn handle_melee_weapon_draw(
    input: Res<InputState>,
    mut toggle_events: EventWriter<ToggleMeleeWeaponDrawEvent>,
    state_query: Query<(Entity, &MeleeWeaponEquipmentState)>,
) {
    if !input.ability_use_pressed {
        return;
    }

    for (entity, state) in state_query.iter() {
        if state.equipped_weapon_id.is_some() {
            toggle_events.send(ToggleMeleeWeaponDrawEvent { owner: entity });
        }
    }
}

pub fn apply_melee_weapon_draw(
    mut commands: Commands,
    mut toggle_events: EventReader<ToggleMeleeWeaponDrawEvent>,
    mut state_query: Query<&mut MeleeWeaponEquipmentState>,
    mount_query: Query<(Entity, &WeaponMountPoint, &Parent)>,
) {
    for event in toggle_events.read() {
        let Ok(mut state) = state_query.get_mut(event.owner) else { continue };
        let Some(weapon_entity) = state.weapon_entity else { continue };

        state.drawn = !state.drawn;

        let (slot, mount_type) = if state.drawn {
            (state.hand_slot.as_str(), WeaponMountType::Hand)
        } else {
            (state.holster_slot.as_str(), WeaponMountType::Holster)
        };

        if let Some(mount) = find_mount_point(event.owner, slot, mount_type, &mount_query) {
            commands.entity(weapon_entity).set_parent(mount);
        } else {
            commands.entity(weapon_entity).set_parent(event.owner);
        }
    }
}

fn find_mount_point(
    owner: Entity,
    slot: &str,
    mount_type: WeaponMountType,
    query: &Query<(Entity, &WeaponMountPoint, &Parent)>,
) -> Option<Entity> {
    for (entity, mount, parent) in query.iter() {
        if parent.get() != owner {
            continue;
        }
        if mount.mount_type == mount_type && mount.slot == slot {
            return Some(entity);
        }
    }
    None
}
