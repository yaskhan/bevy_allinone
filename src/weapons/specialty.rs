use bevy::prelude::*;
use avian3d::prelude::*;
use crate::input::InputState;
use crate::combat::{DamageEventQueue, DamageEvent, DamageType};
use super::types::*;

/// System to handle specialty weapon behaviors
pub fn handle_specialty_behaviors(
    mut commands: Commands,
    time: Res<Time>,
    mut damage_events: ResMut<DamageEventQueue>,
    spatial_query: SpatialQuery,
    mut player_query: Query<(Entity, &InputState, &mut WeaponManager, &GlobalTransform)>,
    mut weapon_query: Query<(&mut Weapon, &mut SpecialtyState, &GlobalTransform)>,
    mut rb_query: Query<(&mut ExternalForce, &GlobalTransform)>,
) {
    let dt = time.delta_secs();

    for (player_entity, input, mut manager, player_transform) in player_query.iter_mut() {
        if let Some(&weapon_entity) = manager.weapons_list.get(manager.current_index) {
            if let Ok((mut weapon, mut state, weapon_transform)) = weapon_query.get_mut(weapon_entity) {
                match &weapon.specialty_behavior {
                    SpecialtyBehavior::GravityGun(settings) => {
                        handle_gravity_gun(
                            &mut commands,
                            input,
                            settings,
                            &mut state,
                            weapon_transform,
                            &spatial_query,
                            &mut rb_query,
                            player_entity,
                        );
                    }
                    SpecialtyBehavior::Beam(settings) => {
                        if input.fire_pressed && weapon.current_ammo > 0 {
                            handle_beam_weapon(
                                &mut commands,
                                dt,
                                settings,
                                &mut state,
                                weapon_transform,
                                &spatial_query,
                                &mut damage_events,
                                player_entity,
                            );
                            // Continuous ammo consumption
                            weapon.current_ammo = (weapon.current_ammo as f32 - settings.energy_per_second * dt).max(0.0) as i32;
                        } else {
                            state.is_active = false;
                        }
                    }
                    SpecialtyBehavior::Flashlight(settings) => {
                        if input.fire_just_pressed {
                            state.is_active = !state.is_active;
                        }
                        // Actual light toggle would happen here or in a separate system
                    }
                    _ => {}
                }
            }
        }
    }
}

pub fn handle_gravity_gun(
    commands: &mut Commands,
    input: &InputState,
    settings: &GravityGunSettings,
    state: &mut SpecialtyState,
    weapon_transform: &GlobalTransform,
    spatial_query: &SpatialQuery,
    rb_query: &mut Query<(&mut ExternalForce, &GlobalTransform)>,
    player_entity: Entity,
) {
    if input.fire_just_pressed {
        if let Some(held_entity) = state.target_entity {
            // Throw
            if let Ok((mut force, _)) = rb_query.get_mut(held_entity) {
                force.apply_force(weapon_transform.forward() * settings.throw_force);
            }
            state.target_entity = None;
            state.is_active = false;
        } else {
            // Try grab
            let ray_origin = weapon_transform.translation();
            let ray_dir = weapon_transform.forward();
            let filter = SpatialQueryFilter::from_excluded_entities([player_entity]);
            
            if let Some(hit) = spatial_query.cast_ray(
                ray_origin,
                Dir3::new(ray_dir).unwrap_or(Dir3::Y),
                settings.max_grab_distance,
                true,
                &filter,
            ) {
                if rb_query.get_mut(hit.entity).is_ok() {
                    state.target_entity = Some(hit.entity);
                    state.is_active = true;
                }
            }
        }
    }

    if let Some(held_entity) = state.target_entity {
        if let Ok((mut force, target_transform)) = rb_query.get_mut(held_entity) {
            let target_pos = weapon_transform.translation() + weapon_transform.forward() * settings.hold_distance;
            let current_pos = target_transform.translation();
            let diff = target_pos - current_pos;
            
            // Simple P-controller for holding
            force.apply_force(diff * settings.hold_speed);
            
            // If it gets too far, drop it
            if diff.length() > settings.hold_distance * 2.0 {
                state.target_entity = None;
                state.is_active = false;
            }
        } else {
            state.target_entity = None;
            state.is_active = false;
        }
    }
}

pub fn handle_beam_weapon(
    commands: &mut Commands,
    dt: f32,
    settings: &BeamSettings,
    state: &mut SpecialtyState,
    weapon_transform: &GlobalTransform,
    spatial_query: &SpatialQuery,
    damage_events: &mut DamageEventQueue,
    player_entity: Entity,
) {
    state.is_active = true;
    let ray_origin = weapon_transform.translation();
    let ray_dir = weapon_transform.forward();
    let filter = SpatialQueryFilter::from_excluded_entities([player_entity]);

    if let Some(hit) = spatial_query.cast_ray(
        ray_origin,
        Dir3::new(ray_dir).unwrap_or(Dir3::Y),
        settings.range,
        true,
        &filter,
    ) {
        // Apply damage/heal over time
        damage_events.0.push(DamageEvent {
            amount: settings.damage_per_second * dt,
            damage_type: match settings.beam_type {
                BeamType::Laser => DamageType::Ranged,
                BeamType::Fire => DamageType::Fire,
                BeamType::Heal => DamageType::Heal,
            },
            source: Some(player_entity),
            target: hit.entity,
        });

        // Visual feedback would be spawned here (particles, smoke)
    }
}
